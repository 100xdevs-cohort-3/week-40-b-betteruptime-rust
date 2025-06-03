use sqlx::{PgPool, pool::PoolOptions};
use uuid::Uuid;

pub mod models;

pub use models::{Region, Website, WebsiteStatus, WebsiteTick};

#[derive(Clone)]
pub struct Store {
    pool: PgPool,
}

impl Store {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let pool = PgPool::connect(&database_url).await?;
        let pool = PoolOptions::new()
            .max_connections(50)
            .connect(&database_url)
            .await?;
        Ok(Store { pool })
    }

    pub async fn create_website(
        &self,
        url: &str,
        name: Option<&str>,
    ) -> Result<models::Website, sqlx::Error> {
        let website = sqlx::query_as!(
            models::Website,
            r#"
            INSERT INTO website (id, url, name)
            VALUES ($1, $2, $3)
            RETURNING id, url, name, time_added
            "#,
            Uuid::new_v4().to_string(),
            url,
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(website)
    }

    pub async fn get_website(&self, id: &str) -> Result<models::Website, sqlx::Error> {
        let website = sqlx::query_as!(
            models::Website,
            r#"
            SELECT id, url, name, time_added
            FROM website
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(website)
    }

    pub async fn get_websites(&self) -> Result<Vec<models::Website>, sqlx::Error> {
        let websites = sqlx::query_as!(
            models::Website,
            r#"
            SELECT id, url, name, time_added
            FROM website
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(websites)
    }

    pub async fn update_website(
        &self,
        id: &str,
        url: Option<&str>,
        name: Option<&str>,
    ) -> Result<models::Website, sqlx::Error> {
        let website = sqlx::query_as!(
            models::Website,
            r#"
            UPDATE website
            SET url = COALESCE($2, url),
                name = COALESCE($3, name)
            WHERE id = $1
            RETURNING id, url, name, time_added
            "#,
            id,
            url,
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(website)
    }

    pub async fn delete_website(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM website
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_regions(&self) -> Result<Vec<models::Region>, sqlx::Error> {
        let regions = sqlx::query_as!(models::Region, r#"SELECT id, name FROM region"#,)
            .fetch_all(&self.pool)
            .await?;
        Ok(regions)
    }

    pub async fn create_region(&self, name: &str) -> Result<models::Region, sqlx::Error> {
        let region = sqlx::query_as!(
            models::Region,
            r#"
            INSERT INTO region (id, name)
            VALUES ($1, $2)
            RETURNING id, name
            "#,
            Uuid::new_v4().to_string(),
            name
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(region)
    }
}
