extern crate shards_rust;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/",rocket::routes![
            // Shards routes
            shards_rust::rocket_routes::shards::get_shards,
            shards_rust::rocket_routes::shards::view_shard,
            shards_rust::rocket_routes::shards::create_shard,
            shards_rust::rocket_routes::shards::update_shard,
            shards_rust::rocket_routes::shards::delete_shard,
            
            // Categories routes
            shards_rust::rocket_routes::categories::get_categories,
            shards_rust::rocket_routes::categories::view_category,
            shards_rust::rocket_routes::categories::create_category,
            shards_rust::rocket_routes::categories::update_category,
            shards_rust::rocket_routes::categories::delete_category,
            
            // Users routes
            shards_rust::rocket_routes::users::get_users,
            shards_rust::rocket_routes::users::view_user,
            shards_rust::rocket_routes::users::create_user,
            shards_rust::rocket_routes::users::update_user,
            shards_rust::rocket_routes::users::delete_user,
            
            // Roles routes
            shards_rust::rocket_routes::roles::get_roles,
            shards_rust::rocket_routes::roles::view_role,
            shards_rust::rocket_routes::roles::create_role,
            shards_rust::rocket_routes::roles::update_role,
            shards_rust::rocket_routes::roles::delete_role,
            
            // Circles routes
            shards_rust::rocket_routes::circles::get_circles,
            shards_rust::rocket_routes::circles::view_circle,
            shards_rust::rocket_routes::circles::create_circle,
            shards_rust::rocket_routes::circles::update_circle,
            shards_rust::rocket_routes::circles::delete_circle,
            
            // Templates routes
            shards_rust::rocket_routes::templates::get_templates,
            shards_rust::rocket_routes::templates::view_template,
            shards_rust::rocket_routes::templates::create_template,
            shards_rust::rocket_routes::templates::update_template,
            shards_rust::rocket_routes::templates::delete_template,
            
            // Circle Members routes
            shards_rust::rocket_routes::circle_members::get_circle_members,
            shards_rust::rocket_routes::circle_members::view_circle_member,
            shards_rust::rocket_routes::circle_members::create_circle_member,
            shards_rust::rocket_routes::circle_members::update_circle_member,
            shards_rust::rocket_routes::circle_members::delete_circle_member,
            
            // User Friends routes
            shards_rust::rocket_routes::user_friends::get_user_friends,
            shards_rust::rocket_routes::user_friends::view_user_friend,
            shards_rust::rocket_routes::user_friends::create_user_friend,
            shards_rust::rocket_routes::user_friends::update_user_friend,
            shards_rust::rocket_routes::user_friends::delete_user_friend,
            
            // User Roles routes
            shards_rust::rocket_routes::user_roles::get_user_roles,
            shards_rust::rocket_routes::user_roles::view_user_role,
            shards_rust::rocket_routes::user_roles::create_user_role,
            shards_rust::rocket_routes::user_roles::update_user_role,
            shards_rust::rocket_routes::user_roles::delete_user_role,
        ])
        .attach(shards_rust::rocket_routes::DbConn::init())
        .launch()
        .await;
}