use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::DbConn;
use crate::models::{CircleMember, NewCircleMember};
use crate::repositories::CircleMemberRepository;

#[rocket::get("/circle-members")]
pub async fn get_circle_members(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    CircleMemberRepository::find_multiple(&mut db,100).await
        .map(|circle_members| json!(circle_members))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/circle-members/<id>")]
pub async fn view_circle_member(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    CircleMemberRepository::find(&mut db, id).await
        .map(|circle_member| json!(circle_member))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/circle-members",format="json", data="<new_circle_member>")]
pub async fn create_circle_member(mut db: Connection<DbConn>, new_circle_member: Json<NewCircleMember>) -> Result<Custom<Value>,Custom<Value>> {
    CircleMemberRepository::create(&mut db,new_circle_member.into_inner()).await
        .map(|circle_member| Custom(Status::Created, json!(circle_member)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/circle-members/<id>",format="json", data="<circle_member>")]
pub async fn update_circle_member(mut db: Connection<DbConn>,id:i32, circle_member: Json<CircleMember>) -> Result<Value,Custom<Value>> {
    CircleMemberRepository::update(&mut db,id,circle_member.into_inner()).await
        .map(|circle_member| json!(circle_member))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/circle-members/<id>")]
pub async fn delete_circle_member(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    CircleMemberRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}