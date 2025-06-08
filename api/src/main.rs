use poem::{
    EndpointExt, Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path, Query},
};
use redis_lib::RedisStore;
use reqwest::Client as HttpClient;
use store::{
    Store,
    models::{Region, Website},
};

mod input;

#[handler]
async fn get_websites(store: Data<&Store>) -> Json<Vec<Website>> {
    let websites = store.get_websites().await.unwrap_or_default();
    Json(websites)
}

#[handler]
async fn get_website(store: Data<&Store>, id: Path<String>) -> Json<Option<Website>> {
    let website = store.get_website(&id).await.ok();
    Json(website)
}

#[handler]
async fn create_website(
    store: Data<&Store>,
    redis: Data<&RedisStore>,
    input: Json<input::CreateWebsite>,
) -> Result<Json<Website>, poem::Error> {
    let website = store
        .create_website(&input.url, input.name.as_deref())
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

    // Add to Redis stream
    let redis_entry = redis_lib::WebsiteStreamEntry {
        id: website.id.clone(),
        url: website.url.clone(),
        name: website.name.clone(),
    };
    redis
        .add_website_to_stream(redis_entry)
        .await
        .map_err(|e| {
            poem::Error::from_string(
                format!("Redis error: {}", e),
                poem::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    Ok(Json(website))
}

#[handler]
async fn update_website(
    store: Data<&Store>,
    id: Path<String>,
    input: Json<input::UpdateWebsite>,
) -> Json<Option<Website>> {
    let website = store
        .update_website(&id, input.url.as_deref(), input.name.as_deref())
        .await
        .ok();
    Json(website)
}

#[handler]
async fn delete_website(
    store: Data<&Store>,
    redis: Data<&RedisStore>,
    id: Path<String>,
) -> Result<String, poem::Error> {
    redis
        .trim_website_stream(0)
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;
    store
        .delete_website(&id)
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;
    Ok("Website deleted".to_string())
}

#[handler]
async fn get_regions(store: Data<&Store>) -> Json<Vec<Region>> {
    let regions = store.get_regions().await.unwrap_or_default();
    Json(regions)
}

#[handler]
async fn create_region(store: Data<&Store>, input: Json<input::CreateRegion>) -> Json<Region> {
    let region = store.create_region(&input.name).await.unwrap();
    Json(region)
}

// Helper function to query InfluxDB directly via HTTP
async fn query_influxdb(
    client: &HttpClient,
    influx_url: &str,
    influx_token: &str,
    org: &str,
    query: &str,
) -> Result<Vec<input::TimeSeriesPoint>, poem::Error> {
    println!("Executing query: {}", query);

    let response = client
        .post(&format!("{}/api/v2/query?org={}", influx_url, org))
        .header("Authorization", &format!("Token {}", influx_token))
        .header("Content-Type", "application/vnd.flux")
        .body(query.to_string())
        .send()
        .await
        .map_err(|e| {
            println!("HTTP request error: {:?}", e);
            poem::error::InternalServerError(e)
        })?;

    let text = response.text().await.map_err(|e| {
        println!("Response text error: {:?}", e);
        poem::error::InternalServerError(e)
    })?;

    println!("Raw InfluxDB response: {}", text);

    // Parsing CSV response
    let mut time_series_points = Vec::new();

    for line in text.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 11 {
            // CSV format: ,result,table,_start,_stop,_time,_value,_field,_measurement,region_id,status,website_id
            // Index:       0  1     2     3      4     5     6      7       8           9         10     11
            if let (Ok(value), time_str) = (fields[6].parse::<f64>(), fields[5]) {
                time_series_points.push(input::TimeSeriesPoint {
                    time: time_str.to_string(),
                    value,
                });
            }
        }
    }

    // Sort by time
    time_series_points.sort_by(|a, b| a.time.cmp(&b.time));

    println!("Parsed {} data points", time_series_points.len());
    Ok(time_series_points)
}

#[handler]
async fn get_monitor_website(
    store: Data<&Store>,
    config: Data<&input::InfluxConfig>,
    client: Data<&HttpClient>,
    id: Path<String>,
    query: Query<input::TickQuery>,
) -> Result<Json<Vec<input::TimeSeriesPoint>>, poem::Error> {
    let website = store
        .get_website(&id)
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

    println!("Website found: {:?}", website);

    let days = query.0.days.unwrap_or(1);
    let region = query.0.region.as_deref().unwrap_or("");

    let range = match days {
        1 => "-1d",
        7 => "-7d",
        30 => "-30d",
        _ => "-1d",
    };

    let filter_region = if region.is_empty() {
        "".to_string()
    } else {
        format!(r#" |> filter(fn: (r) => r["region_id"] == "{}")"#, region)
    };

    let query_str = format!(
        r#"from(bucket: "website_ticks")
    |> range(start: {})
    |> filter(fn: (r) => r["_measurement"] == "website_tick")
    |> filter(fn: (r) => r["_field"] == "response_time_ms")
    |> filter(fn: (r) => r["website_id"] == "{}"){}
    |> filter(fn: (r) => r["status"] == "Up")
    |> sort(columns: ["_time"])"#,
        range, website.id, filter_region
    );

    let data_points =
        query_influxdb(&client, &config.url, &config.token, &config.org, &query_str).await?;

    Ok(Json(data_points))
}

#[handler]
async fn get_downtime(
    store: Data<&Store>,
    config: Data<&input::InfluxConfig>,
    client: Data<&HttpClient>,
    id: Path<String>,
    query: Query<input::TickQuery>,
) -> Result<Json<Vec<input::TimeSeriesPoint>>, poem::Error> {
    let website = store
        .get_website(&id)
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

    println!("Website found: {:?}", website);

    let days = query.0.days.unwrap_or(1);
    let region = query.0.region.as_deref().unwrap_or("");

    let range = match days {
        1 => "-1d",
        7 => "-7d",
        30 => "-30d",
        _ => "-1d",
    };

    let filter_region = if region.is_empty() {
        "".to_string()
    } else {
        format!(r#" |> filter(fn: (r) => r["region_id"] == "{}")"#, region)
    };

    let query_str = format!(
        r#"from(bucket: "website_ticks")
    |> range(start: {})
    |> filter(fn: (r) => r["_measurement"] == "website_tick")
    |> filter(fn: (r) => r["_field"] == "response_time_ms")
    |> filter(fn: (r) => r["website_id"] == "{}"){}
    |> filter(fn: (r) => r["status"] == "Down")
    |> sort(columns: ["_time"])"#,
        range, website.id, filter_region
    );

    let data_points =
        query_influxdb(&client, &config.url, &config.token, &config.org, &query_str).await?;

    Ok(Json(data_points))
}

#[handler]
async fn get_last_downtime(
    store: Data<&Store>,
    config: Data<&input::InfluxConfig>,
    client: Data<&HttpClient>,
    id: Path<String>,
    query: Query<input::TickQuery>,
) -> Result<Json<Option<input::LastDowntime>>, poem::Error> {
    let website = store
        .get_website(&id)
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

    println!("Website found: {:?}", website);

    let region = query.0.region.as_deref().unwrap_or("");

    let filter_region = if region.is_empty() {
        "".to_string()
    } else {
        format!(r#" |> filter(fn: (r) => r["region_id"] == "{}")"#, region)
    };

    let query_str = format!(
        r#"from(bucket: "website_ticks")
    |> range(start: -30d)
    |> filter(fn: (r) => r["_measurement"] == "website_tick")
    |> filter(fn: (r) => r["_field"] == "response_time_ms")
    |> filter(fn: (r) => r["website_id"] == "{}"){}
    |> filter(fn: (r) => r["status"] == "Down")
    |> last()"#,
        website.id, filter_region
    );

    let data_points =
        query_influxdb(&client, &config.url, &config.token, &config.org, &query_str).await?;

    let last_downtime = data_points
        .into_iter()
        .next()
        .map(|point| input::LastDowntime {
            time: point.time,
            value: point.value,
        });

    Ok(Json(last_downtime))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::from_filename(".env").map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to load .env file: {}", e),
        )
    })?;

    let store = Store::new().await.map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize store: {}", e),
        )
    })?;

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis = RedisStore::new(&redis_url).await.map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize Redis: {}", e),
        )
    })?;

    // InfluxDB configuration
    let influx_config = input::InfluxConfig {
        url: std::env::var("INFLUXDB_URL").expect("INFLUXDB_URL must be set"),
        token: std::env::var("INFLUXDB_TOKEN").expect("INFLUXDB_TOKEN must be set"),
        org: std::env::var("INFLUXDB_ORG").unwrap_or_else(|_| "website_ticks".to_string()),
    };

    // HTTP client for InfluxDB queries
    let http_client = HttpClient::new();

    let app = Route::new()
        .at("/websites", get(get_websites).post(create_website))
        .at(
            "/websites/:id",
            get(get_website)
                .patch(update_website)
                .delete(delete_website),
        )
        .at("/regions", post(create_region).get(get_regions))
        .at("/monitor/:id", get(get_monitor_website))
        .at("/monitor/:id/downtime", get(get_downtime))
        .at("/monitor/:id/last_downtime", get(get_last_downtime))
        .data(store)
        .data(redis)
        .data(influx_config)
        .data(http_client);

    Server::new(TcpListener::bind("0.0.0.0:3002"))
        .run(app)
        .await
}
