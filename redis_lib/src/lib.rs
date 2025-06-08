use std::sync::Arc;

use redis::{
    AsyncCommands, Client,
    streams::{StreamMaxlen, StreamReadOptions, StreamReadReply},
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct RedisStore {
    client: Arc<Client>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebsiteStreamEntry {
    pub id: String,
    pub url: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NotificationEntry {
    pub website_id: String,
    pub region_id: String,
    pub status: String,
    pub response_time_ms: i32,
    pub timestamp: i64,
}

type BoxError = Box<dyn std::error::Error + Send + Sync>;

impl RedisStore {
    pub async fn new(redis_url: &str) -> Result<Self, BoxError> {
        let client = Client::open(redis_url)?;
        Ok(RedisStore {
            client: Arc::new(client),
        })
    }

    pub async fn add_website_to_stream(
        &self,
        website: WebsiteStreamEntry,
    ) -> Result<String, BoxError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let serialized = serde_json::to_string(&website)?;
        let stream_id: String = conn
            .xadd("website_stream", "*", &[("data", serialized)])
            .await?;
        Ok(stream_id)
    }

    pub async fn get_websites_from_stream(
        &self,
        consumer_group: &str,
        consumer_name: &str,
        count: usize,
    ) -> Result<Vec<(String, WebsiteStreamEntry)>, BoxError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        // Create consumer group if it doesn't exist
        let _: Result<(), redis::RedisError> = conn
            .xgroup_create_mkstream("website_stream", consumer_group, "0")
            .await;

        let opts = StreamReadOptions::default()
            .group(consumer_group, consumer_name)
            .count(count)
            .block(1000);

        let result: StreamReadReply = conn
            .xread_options(&["website_stream"], &[">"], &opts)
            .await?;

        let mut entries = Vec::new();
        for key in result.keys {
            for entry_id in key.ids {
                let data: String = entry_id
                    .get("data")
                    .ok_or("Missing 'data' field in stream entry")?;
                let website: WebsiteStreamEntry = serde_json::from_str(&data)?;
                entries.push((entry_id.id, website));
            }
        }
        Ok(entries)
    }

    pub async fn ack_website(
        &self,
        consumer_group: &str,
        id: &str,
    ) -> Result<i32, redis::RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let ack_count: i32 = conn.xack("website_stream", consumer_group, &[id]).await?;
        Ok(ack_count)
    }

    pub async fn add_notification(
        &self,
        notification: NotificationEntry,
    ) -> Result<String, BoxError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let serialized = serde_json::to_string(&notification)?;
        let stream_id: String = conn
            .xadd("notification_stream", "*", &[("data", serialized)])
            .await?;
        Ok(stream_id)
    }

    pub async fn get_notifications(
        &self,
        consumer_group: &str,
        consumer_name: &str,
        count: usize,
    ) -> Result<Vec<(String, NotificationEntry)>, BoxError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let _: Result<(), redis::RedisError> = conn
            .xgroup_create_mkstream("notification_stream", consumer_group, "0")
            .await;

        let opts = StreamReadOptions::default()
            .group(consumer_group, consumer_name)
            .count(count)
            .block(1000);

        let result: StreamReadReply = conn
            .xread_options(&["notification_stream"], &[">"], &opts)
            .await?;

        let mut entries = Vec::new();
        for key in result.keys {
            for entry_id in key.ids {
                let data: String = entry_id
                    .get("data")
                    .ok_or("Missing 'data' field in stream entry")?;
                let notification: NotificationEntry = serde_json::from_str(&data)?;
                entries.push((entry_id.id, notification));
            }
        }
        Ok(entries)
    }

    pub async fn ack_notification(
        &self,
        consumer_group: &str,
        id: &str,
    ) -> Result<i32, redis::RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let ack_count: i32 = conn
            .xack("notification_stream", consumer_group, &[id])
            .await?;
        Ok(ack_count)
    }

    pub async fn get_all_websites(&self) -> Result<Vec<WebsiteStreamEntry>, BoxError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: StreamReadReply = conn.xread(&["website_stream"], &["0-0"]).await?;

        let mut entries = Vec::new();
        for key in result.keys {
            for entry_id in key.ids {
                let data: String = entry_id
                    .get("data")
                    .ok_or("Missing 'data' field in stream entry")?;
                let website: WebsiteStreamEntry = serde_json::from_str(&data)?;
                entries.push(website);
            }
        }
        Ok(entries)
    }

    pub async fn trim_website_stream(&self, max_len: usize) -> Result<i32, redis::RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let trimmed_count: i32 = conn
            .xtrim("website_stream", StreamMaxlen::Equals(max_len))
            .await?;
        Ok(trimmed_count)
    }

    pub async fn set_emails_for_url(&self, url: &str, emails: Vec<String>) -> Result<(), BoxError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let serialized = serde_json::to_string(&emails)?;
        let _: () = conn.hset("website_emails", url, serialized).await?;
        Ok(())
    }

    pub async fn get_emails_for_url(&self, url: &str) -> Result<Vec<String>, BoxError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let result: Option<String> = conn.hget("website_emails", url).await?;
        match result {
            Some(serialized) => {
                let emails: Vec<String> = serde_json::from_str(&serialized)?;
                Ok(emails)
            }
            None => Ok(Vec::new()),
        }
    }
}
