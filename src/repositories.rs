use diesel::prelude::*;
use diesel::QueryResult;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::models::*;
use crate::schema::*;
pub struct CategoryRepository;

impl CategoryRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Category> {
        categories::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Category> {
        categories::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_category: Category) -> QueryResult<Category> {
        diesel::insert_into(categories::table)
            .values(new_category)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, category: Category) -> QueryResult<Category> {
        diesel::update(categories::table.find(id))
            .set(
                categories::name.eq(category.name)
            )
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(categories::table.find(id)).execute(conn)
            .await
    }
}
