use rocket::http::Status;
use rocket::routes;
use rocket_db_pools::{diesel, Database};
use crate::repositories::ShardRepository;
use rocket::serde::json::{json, Json, Value};
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;

mod models;
mod repositories;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);
#[rocket::get("/shards")]
async fn get_shards(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    ShardRepository::find_multiple(&mut db,100).await
        .map(|shards| json!(shards))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_shards
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
