use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::DbConn;
use crate::models::{Role, NewRole};
use crate::repositories::RoleRepository;

#[rocket::get("/roles")]
pub async fn get_roles(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    RoleRepository::find_multiple(&mut db,100).await
        .map(|roles| json!(roles))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/roles/<id>")]
pub async fn view_role(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    RoleRepository::find(&mut db, id).await
        .map(|role| json!(role))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/roles",format="json", data="<new_role>")]
pub async fn create_role(mut db: Connection<DbConn>, new_role: Json<NewRole>) -> Result<Custom<Value>,Custom<Value>> {
    RoleRepository::create(&mut db,new_role.into_inner()).await
        .map(|role| Custom(Status::Created, json!(role)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/roles/<id>",format="json", data="<role>")]
pub async fn update_role(mut db: Connection<DbConn>,id:i32, role: Json<Role>) -> Result<Value,Custom<Value>> {
    RoleRepository::update(&mut db,id,role.into_inner()).await
        .map(|role| json!(role))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/roles/<id>")]
pub async fn delete_role(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    RoleRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}