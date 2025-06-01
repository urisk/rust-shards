use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name=categories)]
pub struct Category{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct Circle {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct Role{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub code: String,
    pub name: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct User{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct CircleMember{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub circle_id: i32,
    pub user_id: i32,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct Shard{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub shard_category: i32,
    pub title: String,
    pub owner_id: i32,
    pub visibility: i32,
    pub parent_shard: Option<i32>,
    pub genre: Option<String>,
    pub shard: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct Template{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub owner_id: i32,
    pub version: String,
    pub visibility: i32,
    pub template: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct UserFriend{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: i32,
    pub friend_id: i32,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Insertable)]
pub struct UserRole{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_deserializing)]
    pub updated_at: NaiveDateTime,
}
