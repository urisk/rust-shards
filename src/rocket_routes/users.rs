use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::DbConn;
use crate::models::{User, NewUser};
use crate::repositories::UserRepository;

#[rocket::get("/users")]
pub async fn get_users(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    UserRepository::find_multiple(&mut db,100).await
        .map(|users| json!(users))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/users/<id>")]
pub async fn view_user(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    UserRepository::find(&mut db, id).await
        .map(|user| json!(user))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/users",format="json", data="<new_user>")]
pub async fn create_user(mut db: Connection<DbConn>, new_user: Json<NewUser>) -> Result<Custom<Value>,Custom<Value>> {
    UserRepository::create(&mut db,new_user.into_inner()).await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/users/<id>",format="json", data="<user>")]
pub async fn update_user(mut db: Connection<DbConn>,id:i32, user: Json<User>) -> Result<Value,Custom<Value>> {
    UserRepository::update(&mut db,id,user.into_inner()).await
        .map(|user| json!(user))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/users/<id>")]
pub async fn delete_user(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    UserRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}