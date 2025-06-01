use std::time;

use diesel::{data_types::PgTimestamp, prelude::{Insertable, Queryable}, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::store::Store;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::Website)]
#[derive(Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
  pub id: String,
  pub url: String
}

impl Store {
    pub fn create_website(&self, url: String) -> Result<Website, diesel::result::Error> {
        let new_website = Website {
            id: Uuid::new_v4().to_string(),
            url,
        };
        let mut conn_mut = self.conn.lock().unwrap();

        diesel::insert_into(crate::schema::Website::table)
            .values(&new_website)
            .returning(Website::as_returning())
            .get_result(&mut *conn_mut)?;

        Ok(new_website)
    }

 
    pub fn get_website(&self, website_id: String) -> Result<Website, diesel::result::Error> {
        let mut conn_mut = self.conn.lock().unwrap();
        
        use diesel::prelude::*;
        
        crate::schema::Website::table
            .select(Website::as_select())
            .filter(crate::schema::Website::id.eq(website_id))
            .first(&mut *conn_mut)
    }

}