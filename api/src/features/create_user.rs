use axum::extract::State;

use anyhow::Result;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error_handling::AppError;

#[tracing::instrument(name = "Create User", skip(db))]
pub async fn create_user(
    State(db): State<PgPool>,
    Json(create_user): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = db.create_user(&InsertUser::from(&create_user)).await?;
    Ok((StatusCode::CREATED, format!("/users/{}", user_id)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    email: String,
}

impl<'a> From<&'a CreateUserRequest> for InsertUser<'a> {
    fn from(val: &'a CreateUserRequest) -> Self {
        InsertUser {
            email: &val.email,
        }
    }
}

enum CreateUserError{
    DuplicateUser,
}

struct InsertUser<'a> {
    email: &'a str,
}

trait CreateUserRespository {
    async fn create_user<'a>(&self, create_user: &'a InsertUser<'a>) -> Result<i32>;
}

impl CreateUserRespository for PgPool {
    async fn create_user<'a>(&self, create_user: &'a InsertUser<'a>) -> Result<i32> {
        let inserted = sqlx::query!(
            r#"
                insert into users ( email )
                values ( $1 )
                returning user_id;
            "#,
            create_user.email
        )
        .fetch_one(self)
        .await?;

        Ok(inserted.user_id)
    }
}
