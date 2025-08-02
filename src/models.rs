use std::io::Write;
use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::serialize::ToSql;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::{prelude::*, deserialize::FromSqlRow};
use diesel::expression::AsExpression;
use diesel::sql_types::Text;
use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(AsExpression, Debug, FromSqlRow)]
#[diesel(sql_type=Text)]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::Admin => String::from("admin"),
            RoleCode::Editor => String::from("editor"),
            RoleCode::Viewer => String::from("viewer"),
        }
    }
}

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=categories)]
pub struct NewCategory{
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=circles)]
pub struct NewCircle {
    pub name: String,
    pub owner_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=roles)]
pub struct NewRole{
    pub code: String,
    pub name: String,
}

#[derive(Queryable, Serialize, Deserialize)]
#[derive(Debug)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser{
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
    pub password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=circle_members)]
pub struct NewCircleMember{
    pub circle_id: i32,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name=shards)]
pub struct NewShard{
    pub shard_category: i32,
    pub title: String,
    pub owner_id: i32,
    pub visibility: i32,
    pub parent_shard: Option<i32>,
    pub genre: Option<String>,
    pub shard: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=templates)]
pub struct NewTemplate{
    pub title: String,
    pub owner_id: i32,
    pub version: String,
    pub visibility: i32,
    pub template: Option<String>,
}

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=user_friends)]
pub struct NewUserFriend{
    pub user_id: i32,
    pub friend_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name=user_roles)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
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

#[derive(Insertable, Deserialize)]
#[diesel(table_name=user_roles)]
pub struct NewUserRole{
    pub user_id: i32,
    pub role_id: i32,
}
