use influxdb2::{Client as InfluxClient, models::Query as InfluxQuery};
use poem::{
    EndpointExt, Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path, Query},
};
use redis_lib::RedisStore;
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

#[handler]
async fn get_monitor(
    store: Data<&Store>,
    influx: Data<&InfluxClient>,
) -> Result<Json<Vec<input::WebsiteStatus>>, poem::Error> {
    let websites = store.get_websites().await.unwrap_or_default();
    eprintln!("websites: {:?}", websites);
    let mut statuses = Vec::new();
    for website in websites {
        let query = format!(
            r#"from(bucket: "website_ticks")
                |> range(start: -1h)
                |> filter(fn: (r) => r._measurement == "website_tick" and r.website_id == "{}")
                |> last()"#,
            website.id
        );
        println!("query: {:?}", query);
        let ticks: Vec<input::WebsiteStatus> = influx
            .query(Some(InfluxQuery::new(query)))
            .await
            .map_err(|e| poem::error::InternalServerError(e))?;
        println!("ticks: {:?},", ticks);
        statuses.extend(ticks);
    }
    Ok(Json(statuses))
}

#[handler]
async fn get_monitor_website(
    store: Data<&Store>,
    influx: Data<&InfluxClient>,
    id: Path<String>,
    query: Query<input::TickQuery>,
) -> Result<Json<Vec<input::WebsiteStatus>>, poem::Error> {
    let website = store
        .get_website(&id)
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

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
        format!(r#" and r.region_id == "{}""#, region)
    };

    let query_str = format!(
        r#"from(bucket: "website_ticks")
            |> range(start: {})
            |> filter(fn: (r) => r._measurement == "website_tick" and r.website_id == "{}"{})"#,
        range, website.id, filter_region
    );

    let ticks: Vec<input::WebsiteStatus> = influx
        .query(Some(InfluxQuery::new(query_str)))
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

    Ok(Json(ticks))
}

#[handler]
async fn get_downtime(
    influx: Data<&InfluxClient>,
    id: Path<String>,
    query: Query<input::TickQuery>,
) -> Result<Json<Vec<input::WebsiteStatus>>, poem::Error> {
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
        format!(r#" and r.region_id == "{}""#, region)
    };

    let query_str = format!(
        r#"from(bucket: "website_ticks")
            |> range(start: {})
            |> filter(fn: (r) => r._measurement == "website_tick" and r.website_id == "{}" and r.status == "Down"{})"#,
        range, id.0, filter_region
    );

    let ticks: Vec<input::WebsiteStatus> = influx
        .query(Some(InfluxQuery::new(query_str)))
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

    Ok(Json(ticks))
}

#[handler]
async fn get_last_downtime(
    influx: Data<&InfluxClient>,
    id: Path<String>,
    query: Query<input::TickQuery>,
) -> Result<Json<Option<input::WebsiteStatus>>, poem::Error> {
    let region = query.0.region.as_deref().unwrap_or("");
    let filter_region = if region.is_empty() {
        "".to_string()
    } else {
        format!(r#" and r.region_id == "{}""#, region)
    };

    let query_str = format!(
        r#"from(bucket: "website_ticks")
            |> range(start: -30d)
            |> filter(fn: (r) => r._measurement == "website_tick" and r.website_id == "{}" and r.status == "Down"{})
            |> last()"#,
        id.0, filter_region
    );

    let ticks: Vec<input::WebsiteStatus> = influx
        .query(Some(InfluxQuery::new(query_str)))
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;

    Ok(Json(ticks.into_iter().next()))
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

    let influx_url = std::env::var("INFLUXDB_URL").expect("INFLUXDB_URL must be set");
    let influx_token = std::env::var("INFLUXDB_TOKEN").expect("INFLUXDB_TOKEN must be set");
    let influx = InfluxClient::new(&influx_url, "website_ticks", &influx_token);

    let app = Route::new()
        .at("/websites", get(get_websites).post(create_website))
        .at(
            "/websites/:id",
            get(get_website)
                .patch(update_website)
                .delete(delete_website),
        )
        .at("/regions", post(create_region).get(get_regions))
        .at("/monitor", get(get_monitor))
        .at("/monitor/:id", get(get_monitor_website))
        .at("/monitor/:id/downtime", get(get_downtime))
        .at("/monitor/:id/last_downtime", get(get_last_downtime))
        .data(store)
        .data(redis)
        .data(influx);

    Server::new(TcpListener::bind("0.0.0.0:3002"))
        .run(app)
        .await
}
