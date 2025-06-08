use chrono::Utc;
use futures::stream::{self, StreamExt};
use influxdb2::{Client as InfluxClient, models::DataPoint};
use redis_lib::{NotificationEntry, RedisStore};
use reqwest::Client as HttpClient;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::from_filename(".env")?;

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis = RedisStore::new(&redis_url).await?;

    let influx_url = std::env::var("INFLUXDB_URL").expect("INFLUXDB_URL must be set");
    let influx_token = std::env::var("INFLUXDB_TOKEN").expect("INFLUXDB_TOKEN must be set");
    let influx = InfluxClient::new(&influx_url, "website_ticks", &influx_token);

    let http_client = HttpClient::new();
    // let worker_id = uuid::Uuid::new_v4().to_string();
    // let consumer_group = "website_checkers";

    let mut interval = interval(Duration::from_secs(20)); // Every 3 minutes
    loop {
        interval.tick().await;
        process_websites(&redis, &influx, &http_client).await?;
    }
}

async fn process_websites(
    redis: &RedisStore,
    influx: &InfluxClient,
    http_client: &HttpClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let websites = redis.get_all_websites().await?;

    // Create a stream of futures to process websites concurrently
    let mut futures = stream::iter(websites.into_iter().map(|website| {
        let redis = redis.clone();
        let influx = influx.clone();
        let http_client = http_client.clone();
        async move {
            println!("Processing website: {:?}", website);
            let start = Utc::now().timestamp_millis();
            let status = match http_client.get(&website.url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        "Up"
                    } else {
                        "Down"
                    }
                }
                Err(_) => "Down",
            };
            println!("Status: {:?}", status);
            let response_time_ms = (Utc::now().timestamp_millis() - start) as i32;

            // Write to InfluxDB
            let point = DataPoint::builder("website_tick")
                .tag("website_id", website.id.clone())
                .tag("region_id", "europe")
                .tag("status", status.to_string())
                .field("response_time_ms", response_time_ms as i64)
                .timestamp(start * 1_000_000)
                .build()?;
            influx
                .write("website_ticks", stream::iter(vec![point]))
                .await?;

            println!("Status for {}: {:?}", website.url, status);

            // If down, add to notification stream
            if status == "Down" {
                let notification = NotificationEntry {
                    website_id: website.id.clone(),
                    region_id: "europe".to_string(),
                    status: status.to_string(),
                    response_time_ms,
                    timestamp: start,
                };
                redis.add_notification(notification).await?;
            }
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        }
    }))
    .buffer_unordered(50); // Process up to 50 websites concurrently

    // Collect results and handle errors
    while let Some(result) = futures.next().await {
        if let Err(e) = result {
            eprintln!("Error processing website: {}", e);
        }
    }

    Ok(())
}
