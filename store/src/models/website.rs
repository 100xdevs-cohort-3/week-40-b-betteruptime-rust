use crate::store::Store;
use chrono::NaiveDateTime;
use diesel::{
    RunQueryDsl, Selectable, SelectableHelper,
    prelude::{Insertable, Queryable},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[derive(Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub time_added: NaiveDateTime,
}

impl Store {
    pub fn create_website(&self, url: String) -> Result<Website, diesel::result::Error> {
        let new_website = Website {
            id: Uuid::new_v4().to_string(),
            url,
            time_added: chrono::Utc::now().naive_utc(),
        };
        let mut conn_mut = self.conn.lock().unwrap();

        let inserted_website = diesel::insert_into(crate::schema::website::table)
            .values(&new_website)
            .returning(Website::as_returning())
            .get_result(&mut *conn_mut)?;

        Ok(inserted_website)
    }

    pub fn get_website(&self, website_id: String) -> Result<Website, diesel::result::Error> {
        let mut conn_mut = self.conn.lock().unwrap();

        use diesel::prelude::*;

        crate::schema::website::table
            .select(Website::as_select())
            .filter(crate::schema::website::id.eq(website_id))
            .first(&mut *conn_mut)
    }
}
