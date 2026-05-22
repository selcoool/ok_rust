use sqlx::MySqlPool;

use crate::models::user::User;

pub async fn find_all_users(
    db: &MySqlPool,
) -> Result<Vec<User>, sqlx::Error> {

    sqlx::query_as::<_, User>(
        "SELECT * FROM users"
    )
    .fetch_all(db)
    .await
}

pub async fn insert_user(
    db: &MySqlPool,
    user: &User,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        "INSERT INTO users(
            id,
            name,
            email
         )
         VALUES(?,?,?)"
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&user.email)
    .execute(db)
    .await?;

    Ok(())
}