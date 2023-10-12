use crate::models::assets::Asset as AssetModel;
use crate::forms::assets::Asset as AssetForm;
use crate::connection::{get_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub struct AssetRepository {
    conn: PgConnection
}

impl AssetRepository {
    pub fn new() -> AssetRepository {
        AssetRepository {
            conn: get_connection()
        }
    }

    pub fn create(&mut self, doc: &AssetForm) -> AssetModel {
        use crate::schema::assets;
        diesel::insert_into(assets::table)
            .values(doc)
            .returning(AssetModel::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new post")
    }

    pub fn all(&mut self) -> Vec<AssetModel> {
        use crate::schema::assets::dsl::*;
        assets.select(AssetModel::as_select())
             .load(&mut self.conn)
             .expect("Error loading assets")
    }
}
