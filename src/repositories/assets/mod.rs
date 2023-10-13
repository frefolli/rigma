use crate::models;
use crate::forms;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use crate::schema;

pub struct Asset {
    conn: PgConnection
}

impl Asset {
    pub fn new() -> Asset {
        Asset {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, asset: &forms::Asset) -> models::Asset {
        diesel::insert_into(schema::assets::table)
            .values(asset)
            .returning(models::Asset::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new asset")
    }

    pub fn all(&mut self) -> Vec<models::Asset> {
        use crate::schema::assets::dsl::*;
        assets.select(models::Asset::as_select())
             .load(&mut self.conn)
             .expect("Error loading assets")
    }

    pub fn delete_all(&mut self) {
        use crate::schema::assets::dsl::*;
        diesel::delete(assets)
            .execute(&mut self.conn)
            .expect("Error deleting all assets");
    }

    pub fn find(&mut self, asset_id: i32) -> models::Asset {
        use crate::schema::assets::dsl::*;
        assets.find(asset_id)
            .select(models::Asset::as_returning())
            .first(&mut self.conn)
             .expect("Error finding asset")
    }
}
