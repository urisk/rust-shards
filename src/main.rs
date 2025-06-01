use rocket::routes;
use rocket_db_pools::{diesel, Database};

mod models;
mod repositories;
mod schema;
mod rocket_routes;

#[derive(Database)]
#[database("postgres")]
struct DbConn(diesel::PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            rocket_routes::shards::get_shards,
            rocket_routes::shards::get_shard,
            rocket_routes::shards::create_shard,
            rocket_routes::shards::update_shard,
            rocket_routes::shards::delete_shard,
            rocket_routes::categories::get_categories,
            rocket_routes::categories::get_category,
            rocket_routes::categories::create_category,
            rocket_routes::categories::update_category,
            rocket_routes::categories::delete_category,
        ])
        .attach(DbConn::init())
        .launch()
        .await;
}
