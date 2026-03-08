use crate::domain::model::{Location, Measurement};
use crate::infra::db::models::{DbLocation, DbMeasurement};
use crate::infra::db::schema::{locations, measurements};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

// SQLiteデータベースへの接続を確立する
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub struct SqliteRepository {
    conn: SqliteConnection,
}

impl SqliteRepository {
    pub fn new() -> Self {
        Self {
            conn: establish_connection(),
        }
    }

    // Measurement（測定値）のリストをデータベースに一括保存する
    pub fn insert_measurements(
        &mut self,
        new_measurements: Vec<Measurement>,
    ) -> QueryResult<usize> {
        let db_measurements: Vec<DbMeasurement> =
            new_measurements.into_iter().map(|m| m.into()).collect();

        diesel::insert_into(measurements::table)
            .values(&db_measurements)
            .execute(&mut self.conn)
    }

    // データベースからすべてのMeasurement（測定値）を取得する
    pub fn get_all_measurements(&mut self) -> QueryResult<Vec<Measurement>> {
        let db_measurements = measurements::table.load::<DbMeasurement>(&mut self.conn)?;
        Ok(db_measurements.into_iter().map(|m| m.into()).collect())
    }

    // Location（献血場所）のリストをデータベースに一括保存する
    pub fn insert_locations(&mut self, new_locations: Vec<Location>) -> QueryResult<usize> {
        let db_locations: Vec<DbLocation> =
            new_locations.into_iter().map(|loc| loc.into()).collect();

        diesel::insert_into(locations::table)
            .values(&db_locations)
            .execute(&mut self.conn)
    }

    // データベースからすべてのLocation（献血場所）を取得する
    pub fn get_all_locations(&mut self) -> QueryResult<Vec<Location>> {
        let db_locations = locations::table.load::<DbLocation>(&mut self.conn)?;
        Ok(db_locations.into_iter().map(|loc| loc.into()).collect())
    }
}
