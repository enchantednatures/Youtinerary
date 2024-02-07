use anyhow::Context;
use anyhow::Result;
use oauth2::basic::BasicClient;
use oauth2::AuthUrl;
use oauth2::ClientId;
use oauth2::IntrospectionUrl;
use oauth2::RedirectUrl;
use oauth2::RevocationUrl;
use oauth2::TokenUrl;
use serde::Deserialize;
use serde::Serialize;

pub static COOKIE_NAME: &str = "SESSION";

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub client_id: String,
    pub redirect_url: String,
    pub token_url: String,
    pub auth_url: String,
    pub introspection_url: String,
    pub revocation_url: String,
}

impl TryFrom<AuthSettings> for BasicClient {
    type Error = anyhow::Error;

    fn try_from(auth_settings: AuthSettings) -> Result<Self> {
        Ok(BasicClient::new(
            ClientId::new(auth_settings.client_id),
            None,
            AuthUrl::new(auth_settings.auth_url)
                .context("failed to create new authorization server URL")?,
            Some(
                TokenUrl::new(auth_settings.token_url)
                    .context("failed to create new token endpoint URL")?,
            ),
        )
        .set_revocation_uri(RevocationUrl::new(auth_settings.revocation_url)?)
        .set_introspection_uri(IntrospectionUrl::new(auth_settings.introspection_url)?)
        .set_redirect_uri(
            RedirectUrl::new(auth_settings.redirect_url)
                .context("failed to create new redirection URL")?,
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Oath2State {
    pub pkce_code_verifier_secret: String,
    pub return_url: String,
}



#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AuthentikUser {
    pub email: String,
    pub sub: String,
}
