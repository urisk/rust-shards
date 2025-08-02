use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::models::{Category, NewCategory};
use crate::repositories::CategoryRepository;
use crate::rocket_routes::DbConn;

#[rocket::get("/categories")]
pub async fn get_categories(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    CategoryRepository::find_multiple(&mut db,100).await
        .map(|categories| json!(categories))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/categories/<id>")]
pub async fn view_category(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    CategoryRepository::find(&mut db, id).await
        .map(|categories| json!(categories))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/categories",format="json", data="<new_category>")]
pub async fn create_category(mut db: Connection<DbConn>, new_category: Json<NewCategory>) -> Result<Custom<Value>,Custom<Value>> {
    CategoryRepository::create(&mut db,new_category.into_inner()).await
        .map(|category| Custom(Status::Created, json!(category)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/categories/<id>",format="json", data="<category>")]
pub async fn update_category(mut db: Connection<DbConn>,id:i32, category: Json<Category>) -> Result<Value,Custom<Value>> {
    CategoryRepository::update(&mut db,id,category.into_inner()).await
        .map(|category| json!(category))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/categories/<id>")]
pub async fn delete_category(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    CategoryRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}
