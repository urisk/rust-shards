use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::*;
#[derive(Queryable)]
pub struct Category{
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
#[diesel(table_name=circle_members)]
pub struct CircleMember{
    pub id: i32,
    pub circle_id: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Circle{
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Role{
    pub id: i32,
    pub code: String,
    pub name: String,
    pub owner_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Shard{
    pub id: i32,
    pub shard_category: i32,
    pub title: String,
    pub owner_id: i32,
    pub visibility: i32,
    pub parent_shard: Option<i32>,
    pub genre: Option<String>,
    pub shard: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Template{
    pub id: i32,
    pub title: String,
    pub owner_id: i32,
    pub version: String,
    pub visibility: i32,
    pub template: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
#[diesel(table_name=user_friends)]
pub struct UserFriend{
    pub id: i32,
    pub user_id: String,
    pub friend_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
#[diesel(table_name=user_roles)]
pub struct UserRole{
    pub id: i32,
    pub user_id: String,
    pub role_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
