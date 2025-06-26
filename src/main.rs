use rocket::routes;
use rocket_db_pools::{diesel, Database};

mod models;
mod repositories;
mod schema;
mod rocket_routes;

#[derive(Database)]
#[database("postgres")]
struct DbConn(diesel::PgPool);

#[rocket::main]
async fn main() {
        let _ = rocket::build()
            .mount("/", routes![
            // Shards routes
            rocket_routes::shards::get_shards,
            rocket_routes::shards::view_shard,
            rocket_routes::shards::create_shard,
            rocket_routes::shards::update_shard,
            rocket_routes::shards::delete_shard,
            
            // Categories routes
            rocket_routes::categories::get_categories,
            rocket_routes::categories::view_category,
            rocket_routes::categories::create_category,
            rocket_routes::categories::update_category,
            rocket_routes::categories::delete_category,
            
            // Users routes
            rocket_routes::users::get_users,
            rocket_routes::users::view_user,
            rocket_routes::users::create_user,
            rocket_routes::users::update_user,
            rocket_routes::users::delete_user,
            
            // Roles routes
            rocket_routes::roles::get_roles,
            rocket_routes::roles::view_role,
            rocket_routes::roles::create_role,
            rocket_routes::roles::update_role,
            rocket_routes::roles::delete_role,
            
            // Circles routes
            rocket_routes::circles::get_circles,
            rocket_routes::circles::view_circle,
            rocket_routes::circles::create_circle,
            rocket_routes::circles::update_circle,
            rocket_routes::circles::delete_circle,
            
            // Templates routes
            rocket_routes::templates::get_templates,
            rocket_routes::templates::view_template,
            rocket_routes::templates::create_template,
            rocket_routes::templates::update_template,
            rocket_routes::templates::delete_template,
            
            // Circle Members routes
            rocket_routes::circle_members::get_circle_members,
            rocket_routes::circle_members::view_circle_member,
            rocket_routes::circle_members::create_circle_member,
            rocket_routes::circle_members::update_circle_member,
            rocket_routes::circle_members::delete_circle_member,
            
            // User Friends routes
            rocket_routes::user_friends::get_user_friends,
            rocket_routes::user_friends::view_user_friend,
            rocket_routes::user_friends::create_user_friend,
            rocket_routes::user_friends::update_user_friend,
            rocket_routes::user_friends::delete_user_friend,
            
            // User Roles routes
            rocket_routes::user_roles::get_user_roles,
            rocket_routes::user_roles::view_user_role,
            rocket_routes::user_roles::create_user_role,
            rocket_routes::user_roles::update_user_role,
            rocket_routes::user_roles::delete_user_role,
        ])
            .attach(DbConn::init())
            .launch()
            .await;
}