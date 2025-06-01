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

pub struct CircleRepository;

impl CircleRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Circle> {
        circles::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Circle> {
        circles::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_circle: Circle) -> QueryResult<Circle> {
        diesel::insert_into(circles::table)
            .values(new_circle)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, circle: Circle) -> QueryResult<Circle> {
        diesel::update(circles::table.find(id))
            .set((
                circles::name.eq(circle.name),
                circles::owner_id.eq(circle.owner_id)
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(circles::table.find(id)).execute(conn)
            .await
    }
}
pub struct RoleRepository;

impl RoleRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Role> {
        roles::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Role> {
        roles::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_role: Role) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, role: Role) -> QueryResult<Role> {
        diesel::update(roles::table.find(id))
            .set((
                roles::code.eq(role.code),
                roles::name.eq(role.name)
            ))
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(roles::table.find(id)).execute(conn)
            .await
    }
}

pub struct UserRepository;

impl UserRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<User> {
        users::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_user: User) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set((
                users::username.eq(user.username),
                users::first_name.eq(user.first_name),
                users::last_name.eq(user.last_name),
                users::email.eq(user.email),
                users::phone.eq(user.phone),
                users::bio.eq(user.bio),
                users::password.eq(user.password),
            ))
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(conn)
            .await
    }
}

pub struct CircleMemberRepository;

impl CircleMemberRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<CircleMember> {
        circle_members::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<User> {
        users::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_circle_member: CircleMember) -> QueryResult<CircleMember> {
        diesel::insert_into(circle_members::table)
            .values(new_circle_member)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, circle_member: CircleMember) -> QueryResult<CircleMember> {
        diesel::update(circle_members::table.find(id))

            .set((
                circle_members::circle_id.eq(circle_member.circle_id),
                circle_members::user_id.eq(circle_member.user_id)
            ))
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(circle_members::table.find(id)).execute(conn)
            .await
    }
}

pub struct ShardRepository;

impl ShardRepository {
    // pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Shard> {
    //     shards::table.find(id).get_result(conn).await
    // }

    // pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Shard> {
    //     shards::table.limit(limit).get_result(conn).await
    // }
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Shard> {
        shards::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Shard>> {
        shards::table.limit(limit).load(conn).await
    }
    
    pub async fn create(c: &mut AsyncPgConnection, new_shard: Shard) -> QueryResult<Shard> {
        diesel::insert_into(shards::table)
            .values(new_shard)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, shard: Shard) -> QueryResult<Shard> {
        diesel::update(shards::table.find(id))

            .set((
                shards::shard_category.eq(shard.shard_category),
                shards::title.eq(shard.title),
                shards::owner_id.eq(shard.owner_id),
                shards::visibility.eq(shard.visibility),
                shards::parent_shard.eq(shard.parent_shard),
                shards::genre.eq(shard.genre),
                shards::shard.eq(shard.shard)
            ))
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(shards::table.find(id)).execute(conn)
            .await
    }
}

pub struct UserFriendRepository;

impl UserFriendRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<UserFriend> {
        user_friends::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<UserFriend> {
        user_friends::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_user_friend: UserFriend) -> QueryResult<UserFriend> {
        diesel::insert_into(user_friends::table)
            .values(new_user_friend)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, user_friend: UserFriend) -> QueryResult<UserFriend> {
        diesel::update(user_friends::table.find(id))
            .set((
                user_friends::user_id.eq(user_friend.user_id),
                user_friends::friend_id.eq(user_friend.friend_id)
            ))
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(user_friends::table.find(id)).execute(conn)
            .await
    }
}

pub struct UserRoleRepository;

impl UserRoleRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<UserRole> {
        user_roles::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<UserRole> {
        user_roles::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_user_role: UserRole) -> QueryResult<UserRole> {
        diesel::insert_into(user_roles::table)
            .values(new_user_role)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, user_role: UserRole) -> QueryResult<UserRole> {
        diesel::update(user_roles::table.find(id))
            .set((
                user_roles::user_id.eq(user_role.user_id),
                user_roles::role_id.eq(user_role.role_id)
            ))
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(user_roles::table.find(id)).execute(conn)
            .await
    }
}

pub struct TemplateRepository;

impl TemplateRepository {
    pub async fn find(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<Template> {
        templates::table.find(id).get_result(conn).await
    }

    pub async fn find_multiple(conn: &mut AsyncPgConnection, limit: i64) -> QueryResult<Template> {
        templates::table.limit(limit).get_result(conn).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_template: Template) -> QueryResult<Template> {
        diesel::insert_into(templates::table)
            .values(new_template)
            .get_result(c)
            .await
    }
    pub async fn update(c: &mut AsyncPgConnection, id: i32, template: Template) -> QueryResult<Template> {
        diesel::update(templates::table.find(id))

            .set((
                templates::title.eq(template.title),
                templates::owner_id.eq(template.owner_id),
                templates::version.eq(template.version),
                templates::visibility.eq(template.visibility),
                templates::template.eq(template.template)
            ))
            .get_result(c)
            .await
    }
    pub async fn delete(conn: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(templates::table.find(id)).execute(conn)
            .await
    }
}
