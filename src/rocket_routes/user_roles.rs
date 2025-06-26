use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::DbConn;
use crate::models::{UserRole, NewUserRole};
use crate::repositories::UserRoleRepository;

#[rocket::get("/user-roles")]
pub async fn get_user_roles(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    UserRoleRepository::find_multiple(&mut db,100).await
        .map(|user_roles| json!(user_roles))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/user-roles/<id>")]
pub async fn view_user_role(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    UserRoleRepository::find(&mut db, id).await
        .map(|user_role| json!(user_role))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/user-roles",format="json", data="<new_user_role>")]
pub async fn create_user_role(mut db: Connection<DbConn>, new_user_role: Json<NewUserRole>) -> Result<Custom<Value>,Custom<Value>> {
    UserRoleRepository::create(&mut db,new_user_role.into_inner()).await
        .map(|user_role| Custom(Status::Created, json!(user_role)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/user-roles/<id>",format="json", data="<user_role>")]
pub async fn update_user_role(mut db: Connection<DbConn>,id:i32, user_role: Json<UserRole>) -> Result<Value,Custom<Value>> {
    UserRoleRepository::update(&mut db,id,user_role.into_inner()).await
        .map(|user_role| json!(user_role))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/user-roles/<id>")]
pub async fn delete_user_role(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    UserRoleRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}