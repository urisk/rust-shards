use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::rocket_routes::DbConn;
use crate::models::{Circle, NewCircle};
use crate::repositories::CircleRepository;

#[rocket::get("/circles")]
pub async fn get_circles(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    CircleRepository::find_multiple(&mut db,100).await
        .map(|circles| json!(circles))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/circles/<id>")]
pub async fn view_circle(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    CircleRepository::find(&mut db, id).await
        .map(|circle| json!(circle))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/circles",format="json", data="<new_circle>")]
pub async fn create_circle(mut db: Connection<DbConn>, new_circle: Json<NewCircle>) -> Result<Custom<Value>,Custom<Value>> {
    CircleRepository::create(&mut db,new_circle.into_inner()).await
        .map(|circle| Custom(Status::Created, json!(circle)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/circles/<id>",format="json", data="<circle>")]
pub async fn update_circle(mut db: Connection<DbConn>,id:i32, circle: Json<Circle>) -> Result<Value,Custom<Value>> {
    CircleRepository::update(&mut db,id,circle.into_inner()).await
        .map(|circle| json!(circle))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/circles/<id>")]
pub async fn delete_circle(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    CircleRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}