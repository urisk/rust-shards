use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::rocket_routes::DbConn;
use crate::models::{Template, NewTemplate};
use crate::repositories::TemplateRepository;

#[rocket::get("/templates")]
pub async fn get_templates(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    TemplateRepository::find_multiple(&mut db,100).await
        .map(|templates| json!(templates))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/templates/<id>")]
pub async fn view_template(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    TemplateRepository::find(&mut db, id).await
        .map(|template| json!(template))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/templates",format="json", data="<new_template>")]
pub async fn create_template(mut db: Connection<DbConn>, new_template: Json<NewTemplate>) -> Result<Custom<Value>,Custom<Value>> {
    TemplateRepository::create(&mut db,new_template.into_inner()).await
        .map(|template| Custom(Status::Created, json!(template)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/templates/<id>",format="json", data="<template>")]
pub async fn update_template(mut db: Connection<DbConn>,id:i32, template: Json<Template>) -> Result<Value,Custom<Value>> {
    TemplateRepository::update(&mut db,id,template.into_inner()).await
        .map(|template| json!(template))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/templates/<id>")]
pub async fn delete_template(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    TemplateRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}