use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Json, Value};
use crate::DbConn;
use crate::models::{UserFriend, NewUserFriend};
use crate::repositories::UserFriendRepository;

#[rocket::get("/user-friends")]
pub async fn get_user_friends(mut db: Connection<DbConn>) -> Result<Value,Custom<Value>> {
    UserFriendRepository::find_multiple(&mut db,100).await
        .map(|user_friends| json!(user_friends))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::get("/user-friends/<id>")]
pub async fn view_user_friend(mut db: Connection<DbConn>, id:i32) -> Result<Value,Custom<Value>> {
    UserFriendRepository::find(&mut db, id).await
        .map(|user_friend| json!(user_friend))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::post("/user-friends",format="json", data="<new_user_friend>")]
pub async fn create_user_friend(mut db: Connection<DbConn>, new_user_friend: Json<NewUserFriend>) -> Result<Custom<Value>,Custom<Value>> {
    UserFriendRepository::create(&mut db,new_user_friend.into_inner()).await
        .map(|user_friend| Custom(Status::Created, json!(user_friend)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::put("/user-friends/<id>",format="json", data="<user_friend>")]
pub async fn update_user_friend(mut db: Connection<DbConn>,id:i32, user_friend: Json<UserFriend>) -> Result<Value,Custom<Value>> {
    UserFriendRepository::update(&mut db,id,user_friend.into_inner()).await
        .map(|user_friend| json!(user_friend))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}

#[rocket::delete("/user-friends/<id>")]
pub async fn delete_user_friend(mut db: Connection<DbConn>, id:i32) -> Result<NoContent,Custom<Value>> {
    UserFriendRepository::delete(&mut db,id).await
        .map(|_|  NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error:")))
}