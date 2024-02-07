use anyhow::Result;
use leptos::*;
use leptos_router::*;
use oauth2::basic::BasicClient;
use oauth2::CsrfToken;
use oauth2::PkceCodeChallenge;
use oauth2::Scope;
use reqwest::Url;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone)]
pub struct Oath2State {
    pub pkce_code_verifier_secret: String,
    pub return_url: String,
}

#[derive(Debug, Clone, Deserialize, Params, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthRequest {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AuthentikUser {
    pub email: String,
    pub sub: String,
}

pub struct AuthRedirect;

impl From<anyhow::Error> for AuthRedirect {
    fn from(_value: anyhow::Error) -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginThing(CsrfToken, Oath2State, Url);

impl From<LoginThing> for Url {
    fn from(value: LoginThing) -> Self {
        value.2
    }
}

pub(super) trait Login {
    async fn oauth_login(&self) -> Result<LoginThing>;
}
impl Login for BasicClient {
    async fn oauth_login(&self) -> Result<LoginThing> {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (auth_url, csrf_token) = self
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("identify".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("openid".to_string()))
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        let state = Oath2State {
            pkce_code_verifier_secret: pkce_code_verifier.secret().to_string(),
            return_url: "/".to_string(),
        };
        Ok(LoginThing(csrf_token, state, auth_url))
    }
}
