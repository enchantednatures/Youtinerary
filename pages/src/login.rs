mod login_auth;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::use_session_storage_with_options;
use leptos_use::storage::UseStorageOptions;
use leptos_use::utils::JsonCodec;
use login_auth::AuthRequest;
pub use login_auth::Oath2State;
use login_auth::*;
use oauth2::basic::BasicClient;
use oauth2::reqwest::Error;
use oauth2::AuthorizationCode;
use oauth2::HttpRequest;
use oauth2::HttpResponse;
use oauth2::PkceCodeVerifier;
use oauth2::TokenResponse;
use oauth2::reqwest::async_http_client;
use serde::Deserialize;
use serde::Serialize;
use state::GlobalStateSignal;
use state::LoggedInUser;
use web_sys::MouseEvent;

pub fn logout() {
    let state = expect_context::<GlobalStateSignal>();

    state.update(|s| s.user = None);
}

pub fn login() {
    let state = expect_context::<GlobalStateSignal>();

    state.update(|s| {
    s.user = Some(
            LoggedInUser {
        id: 1,
        email: "tcook@mail.com".to_string(),
        name: "Tom Cook".to_string(),
        username: "tcook".to_string(),
        avatar_url: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
        }
        )
    });
}

// pub async fn async_http_client(
//     request: HttpRequest,
// ) -> Result<HttpResponse, Error<reqwest::Error>> {
//     let client = {
//         let builder = reqwest::Client::builder();

//         // Following redirects opens the client up to SSRF vulnerabilities.
//         // but this is not possible to prevent on wasm targets
//         #[cfg(not(target_arch = "wasm32"))]
//         let builder = builder.redirect(reqwest::redirect::Policy::none());

//         builder.build().map_err(Error::Reqwest)?
//     };

//     let mut request_builder = client
//         .request(request.method, request.url.as_str())
//         .body(request.body);
//     for (name, value) in &request.headers {
//         request_builder = request_builder.header(name.as_str(), value.as_bytes());
//     }

//     request_builder = request_builder.header("Access-Control-Allow-Origin", "*");
//     request_builder = request_builder.header("Access-Control-Allow-Method", "*");
//     let request = request_builder.build().map_err(Error::Reqwest)?;

//     let response = client.execute(request).await.map_err(Error::Reqwest)?;

//     let status_code = response.status();
//     let headers = response.headers().to_owned();
//     let chunks = response.bytes().await.map_err(Error::Reqwest)?;
//     Ok(HttpResponse {
//         status_code,
//         headers,
//         body: chunks.to_vec(),
//     })
// }

#[component]
pub fn Authorize() -> impl IntoView {
    let query = use_query::<AuthRequest>();

    let login_callback = create_resource(query, |query_params| async move {
        console_log("logging in");
        let Oath2State {
            pkce_code_verifier_secret,
            return_url,
        } = expect_context::<Oath2State>();

        let oauth_client = expect_context::<BasicClient>();
        let client = expect_context::<reqwest::Client>();
        match query_params {
            Ok(AuthRequest { code, state: _ }) => {
                // let state = CsrfToken::new(state.to_string());

                console_log(&code);
                console_log(&pkce_code_verifier_secret);
                let code = AuthorizationCode::new(code.to_string());
                let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier_secret);
                let token_response = oauth_client
                    .exchange_code(code)
                    .set_pkce_verifier(pkce_code_verifier)
                    .request_async(async_http_client)
                    .await
                    .map_err(|err| match err {
                        oauth2::RequestTokenError::ServerResponse(server_response) => {
                            format!("Server Error: {}", server_response)
                        }
                        oauth2::RequestTokenError::Request(request_error) => {
                            format!("Request Error: {}", request_error)
                        }
                        oauth2::RequestTokenError::Parse(s, v) => {
                            format!("Parse Error: {} {:?}", s, v)
                        }
                        oauth2::RequestTokenError::Other(o) => {
                            format!("OAuth: exchange_code failure: {}", o)
                        }
                    })
                    .unwrap();

                let access_token_secret = token_response.access_token().secret();
                let url = oauth_client.introspection_url().unwrap().url().as_str();

                let state = expect_context::<GlobalStateSignal>();

                let user_data: AuthentikUser = client
                    .get(url)
                    .header("Accept", "application/json")
                    .header("Access-Control-Allow-Origin", "*")
                    .bearer_auth(access_token_secret)
                    .send()
                    .await
                    .expect("failed to get response from auth provider")
                    .json()
                    .await
                    .expect("failed to deserialze auth response");
                let (_, writer, _) = use_session_storage_with_options::<
                    Option<AuthentikUser>,
                    JsonCodec,
                >("user", UseStorageOptions::default());
                writer.set(Some(user_data.clone()));

                state.update(|s| {
                    s.user = Some(
                            LoggedInUser {
                        id: 1,
                        email: user_data.email.clone(),
                        name: user_data.email.clone(),
                        username: user_data.email.clone(),
                        avatar_url: "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80".to_string(),
                        }
                        )
                    });
                return_url
            }
            Err(_) => panic!(),
        }
    });

    let navigate = use_navigate();
    if let Some(return_url) = login_callback() {
        navigate(
            &return_url,
            NavigateOptions {
                resolve: false,
                ..NavigateOptions::default()
            },
        );
    } else {
        navigate("/", NavigateOptions::default())
    }
    view! {
        <h1>"My Data"</h1>
        {move || match login_callback.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <p>{data}</p> }.into_view(),
        }}
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct LoginState;

#[component]
pub fn Login() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Login"/>
        <button
            type="button"
            class="rounded bg-indigo-600 px-2 py-1 text-xs font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            on:click=move |_ev: MouseEvent| {
                login();
                let navigate = use_navigate();
                navigate("/", NavigateOptions::default());
            }
        >

            Login
        </button>
    }
}

#[component]
pub fn LogOut() -> impl IntoView {
    view! {
        <button
            type="button"
            class="rounded bg-indigo-600 px-2 py-1 text-xs font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            on:click=move |_ev: MouseEvent| {
                logout();
                let navigate = use_navigate();
                navigate("/signup", NavigateOptions::default());
            }
        >

            Log Out
        </button>
    }
}
