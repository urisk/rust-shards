use diesel::QueryResult;
use diesel_async::AsyncConnection;
use crate::{models::User, schema::users};
pub struct UserRepository;

impl UserRepository {
    pub async fn find(c: &mut AsyncConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await;
    }
}