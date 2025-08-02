use std::io::Write;
use std::str::FromStr;

use chrono::NaiveDateTime;
use diesel::serialize::ToSql;
use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::{prelude::*, deserialize::FromSqlRow};
use diesel::expression::AsExpression;
use diesel::sql_types::Text;
use rocket_db_pools::Database;
use crate::schema::*;
use serde::{Serialize, Deserialize};

pub mod shards;
pub mod categories;
pub mod users;
pub mod roles;
pub mod circles;
pub mod templates;
pub mod circle_members;
pub mod user_friends;
pub mod user_roles;

#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);
