// use anyhow::{Context, Result};

// use hyper::HeaderMap;
// use leptos::*;
// use leptos_router::*;
// use leptos::Params;
// use leptos-use::storage::{use_session_storage, use_storage};
// use oauth2::IntrospectionUrl;
// use oauth2::{
//     basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
//     ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl,
//     Scope, TokenResponse, TokenUrl,
// };

// use serde::{Deserialize, Serialize};

// static COOKIE_NAME: &str = "SESSION";

// #[derive(Debug, Deserialize)]
// pub struct AuthSettings {
//     pub client_id: String,
//     pub client_secret: String,
//     pub redirect_url: String,
//     pub token_url: String,
//     pub auth_url: String,
//     pub introspection_url: String,
//     pub revocation_url: String,
// }

// #[derive(Serialize, Deserialize)]
// struct Oath2State {
//     pkce_code_verifier_secret: String,
//     return_url: String,
// }

// trait SessionManager {
//     async fn get_session<'a>(&self, session_id: &'a str) -> Result<Option<Session>>;
//     async fn set_session(&self, session: &Session) -> Result<String>;
//     async fn set_verifier(&self, csrf: &CsrfToken, state: &Oath2State) -> Result<()>;
//     async fn get_verifier(&self, csrf: &CsrfToken) -> Result<Oath2State>;
// }

// #[derive(Params, Debug, Deserialize)]
// pub struct AuthRequest {
//     code: String,
//     state: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct AuthentikUser {
//     pub email: String,
//     pub sub: String,
// }

// pub async fn login_authorized() -> Result<impl IntoResponse, AuthError> {
//     let query = use_query::<AuthRequest>();
//     let store = use_session_storage(COOKIE_NAME);
//     let auth_settings = expect_context::<AuthSettings>();
//     let oauth_client = BasicClient::try_from(auth_settings).unwrap();
//     // let client =
//     let AuthRequest { code, state } = query;

//     let state = CsrfToken::new(state);
//     let code = AuthorizationCode::new(code);
//     let Oath2State {
//         pkce_code_verifier_secret,
//         return_url,
//     } = store.get_verifier(&state).await.unwrap();

//     let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier_secret);

//     let token_response = oauth_client
//         .exchange_code(code)
//         .set_pkce_verifier(pkce_code_verifier)
//         .request_async(async_http_client)
//         .await
//         .map_err(|err| match err {
//             oauth2::RequestTokenError::ServerResponse(server_response) => {
//                 format!("Server Error: {}", server_response)
//             }
//             oauth2::RequestTokenError::Request(request_error) => {
//                 format!("Request Error: {}", request_error)
//             }
//             oauth2::RequestTokenError::Parse(s, v) => format!("Parse Error: {} {:?}", s, v),
//             oauth2::RequestTokenError::Other(o) => format!("OAuth: exchange_code failure: {}", o),
//         })?;

//     let access_token_secret = token_response.access_token().secret();
//     let url = oauth_client.introspection_url().unwrap().url().as_str();

//     let user_data_response = client
//         .get(url)
//         .bearer_auth(access_token_secret)
//         .send()
//         .await?;

//     dbg!(&user_data_response);

//     let user_data = user_data_response.text().await?;

//     dbg!(&user_data);

//     let user_data: AuthentikUser = serde_json::from_str(&user_data)?;

//     // Create a new session filled with user data
//     let mut session = Session::new();
//     session.insert("user", &user_data).unwrap();

//     // Store session and get corresponding cookie
//     let cookie = store.set_session(&session).await.unwrap();

//     // Build the cookie
//     let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

//     // Set cookie
//     let mut headers = HeaderMap::new();
//     headers.insert(SET_COOKIE, cookie.parse().unwrap());

//     Ok((headers, Redirect::to(&return_url)))
// }

// pub async fn authorize() -> impl IntoResponse {
//     let auth_settings = expect_context();
//     let oauth_client = BasicClient::try_from(auth_settings).unwrap();
//     let store = use_session_storage(COOKIE_NAME);
//     let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
//     let (auth_url, csrf_token) = oauth_client
//         .authorize_url(CsrfToken::new_random)
//         .add_scope(Scope::new("identify".to_string()))
//         .add_scope(Scope::new("email".to_string()))
//         .add_scope(Scope::new("openid".to_string()))
//         .set_pkce_challenge(pkce_code_challenge)
//         .url();

//     let state = Oath2State {
//         pkce_code_verifier_secret: pkce_code_verifier.secret().to_string(),
//         return_url: "/".to_string(),
//     };

//     store.set_verifier(&csrf_token, &state).await.unwrap();

//     Redirect::to(auth_url.as_str())
// }

// impl TryFrom<AuthSettings> for BasicClient {
//     type Error = anyhow::Error;

//     fn try_from(auth_settings: AuthSettings) -> Result<Self> {
//         Ok(BasicClient::new(
//             ClientId::new(auth_settings.client_id),
//             Some(ClientSecret::new(auth_settings.client_secret)),
//             AuthUrl::new(auth_settings.auth_url)
//                 .context("failed to create new authorization server URL")?,
//             Some(
//                 TokenUrl::new(auth_settings.token_url)
//                     .context("failed to create new token endpoint URL")?,
//             ),
//         )
//         .set_revocation_uri(RevocationUrl::new(auth_settings.revocation_url)?)
//         .set_introspection_uri(IntrospectionUrl::new(auth_settings.introspection_url)?)
//         .set_redirect_uri(
//             RedirectUrl::new(auth_settings.redirect_url)
//                 .context("failed to create new redirection URL")?,
//         ))
//     }
// }
