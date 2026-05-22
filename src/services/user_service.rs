use sqlx::MySqlPool;

use uuid::Uuid;

use crate::models::user::{
    CreateUser,
    User,
};

use crate::repositories::user_repository::{
    find_all_users,
    insert_user,
};

pub async fn get_users_service(
    db: &MySqlPool,
) -> Result<Vec<User>, sqlx::Error> {

    find_all_users(db).await
}

pub async fn create_user_service(
    db: &MySqlPool,
    body: CreateUser,
) -> Result<User, sqlx::Error> {

    let user = User {

        id: Uuid::new_v4().to_string(),

        name: body.name,

        email: body.email,
    };

    insert_user(db, &user).await?;

    Ok(user)
}