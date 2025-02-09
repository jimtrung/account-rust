use sqlx::PgPool;
use uuid::Uuid;

use crate::models::users::User;

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users;")
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn get_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = $1;")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn delete_user_by_id(pool: &PgPool, user_id: Uuid) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE user_id = $1;")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
