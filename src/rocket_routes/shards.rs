use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::DbConn;
use crate::models::{NewShard, Shard};
use crate::repositories::ShardRepository;
//these two alternative functions were generated via AI. I want to keep them around for now,
//as these have better logging etc, but I don't want to confuse myself by having different
//methods of doing the same thing.

// #[rocket::get("/shards")]
// async fn get_shards(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
//     ShardRepository::find_multiple(&mut db,100).await
//         .map(|shards| json!(shards))
//         .map_err(|e| Custom(Status::InternalServerError, json!({
//             "error": "Database error",
//             "details": e.to_string()
//         })))
// }
// #[rocket::get("/shards")]
// pub async fn get_shards(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
//     println!("Starting get_shards function");
//
//     match ShardRepository::find_multiple(&mut db, 100).await {
//         Ok(shards) => {
//             println!("Successfully retrieved {} shards", shards.len());
//             Ok(json!(shards))
//         },
//         Err(e) => {
//             println!("Error type: {:?}", e);
//             println!("Error details: {}", e);
//             Err(Custom(Status::InternalServerError, json!({
//                 "error": "Database error",
//                 "details": format!("{:?}", e),  // Use {:?} for more detailed error info
//                 "error_string": e.to_string()
//             })))
//         }
//     }
// }
#[rocket::get("/shards")]
pub async fn get_shards(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    ShardRepository::find_multiple(&mut db,100).await
        .map(|shards| json!(shards))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/shards/<id>")]
pub async fn view_shard(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    ShardRepository::find(&mut db, id).await
        .map(|shard| json!(shard))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/shards", format = "json", data = "<shard>")]
pub async fn create_shard(mut db: Connection<DbConn>, shard: Json<NewShard>) -> Result<Custom<Value>, Custom<Value>> {
    ShardRepository::create(&mut db, shard.into_inner()).await
        .map(|shard| Custom(Status::Created, json!(shard)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/shards/<id>",format="json", data="<shard>")]
pub async fn update_shard(mut db: Connection<DbConn>,id:i32, shard: Json<Shard>) -> Result<Value,Custom<Value>> {
    ShardRepository::update(&mut db,id,shard.into_inner()).await
        .map(|shard| json!(shard))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/shards/<id>")]
pub async fn delete_shard(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
        ShardRepository::delete(&mut db,id).await
            .map(|_|  NoContent)
            .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}
