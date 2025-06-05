use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};
use redis_lib::RedisStore;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::from_filename(".env")?;

    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
    let redis = RedisStore::new(&redis_url).await?;

    let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
    let smtp_pass = std::env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = SmtpTransport::relay(&smtp_host)?.credentials(creds).build();

    let consumer_group = "notifiers";
    let consumer_name = uuid::Uuid::new_v4().to_string();

    let mut interval = interval(Duration::from_secs(60)); // Checking every minute
    loop {
        interval.tick().await;
        process_notifications(&redis, &mailer, &consumer_group, &consumer_name).await?;
    }
}

async fn process_notifications(
    redis: &RedisStore,
    mailer: &SmtpTransport,
    consumer_group: &str,
    consumer_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let notifications = redis
        .get_notifications(consumer_group, consumer_name, 100)
        .await?;
    for (stream_id, notification) in notifications {
        // For simplicity, assume email is stored elsewhere or hardcoded
        let email = Message::builder()
            .from("Monitor <0xpankaj@gmail.com>".parse()?)
            .to("User <pankaj98699@gmail.com>".parse()?)
            .subject("Website Down Alert")
            .header(ContentType::TEXT_PLAIN)
            .body(format!(
                "Website {} is down in region {} at {}. Response time: {}ms",
                notification.website_id,
                notification.region_id,
                notification.timestamp,
                notification.response_time_ms
            ))?;

        mailer.send(&email)?;
        redis.ack_notification(consumer_group, &stream_id).await?;
    }
    Ok(())
}
