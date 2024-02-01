use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub struct AuthError {
    code: StatusCode,
    message: String,
    user_message: String,
}

impl AuthError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            user_message: "".to_owned(),
            code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn with_user_message(self, user_message: impl Into<String>) -> Self {
        Self {
            user_message: user_message.into(),
            ..self
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        println!("AuthError: {}", self.message);
        (
            self.code,
            Html(format!(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Oops!</title>
                </head>
                <body>
                    <h1>Oops!</h1>
                    <p>Sorry, but something went wrong.</p>
                    <p>{}</p>
                </body>
                </html>
                "#,
                self.user_message
            )),
        )
            .into_response()
    }
}


impl From<reqwest::Error> for AuthError {
    fn from(err: reqwest::Error) -> Self {
        AuthError::new(format!("reqwest error: {:#}", err))
    }
}


impl From<serde_json::Error> for AuthError {
    fn from(err: serde_json::Error) -> Self {
        AuthError::new(format!("serde_json error: {:#}", err))
    }
}

impl From<String> for AuthError {
    fn from(err: String) -> Self {
        AuthError::new(err)
    }
}

impl From<&str> for AuthError {
    fn from(err: &str) -> Self {
        AuthError::new(err)
    }
}


