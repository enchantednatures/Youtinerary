use anyhow::Result;
use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use sqlx::PgPool;
use auth::{AuthRedirect, AuthentikUser};

use axum::http::request::Parts;

use crate::User;


trait UserRepository {
    async fn get_user<'a>(&self, email: &'a str) -> Result<Option<User>>;
}
impl UserRepository for PgPool {
    async fn get_user<'a>(&self, email: &'a str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            r#"
                select user_id as "id: _", email
                from users
                where email = $1
                limit 1;
            "#,
            email
        )
        .fetch_one(self)
        .await?;

        Ok(Some(user))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
    redis::Client: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = AuthentikUser::from_request_parts(parts, state).await?;
        let db = PgPool::from_ref(state);
        let user = db.get_user(&user.email).await?;

        match user {
            Some(user) => Ok(user),
            None => Err(AuthRedirect::from(anyhow::Error::msg("No user found"))),
        }
    }
}
