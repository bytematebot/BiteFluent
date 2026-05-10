mod api;
#[cfg(feature = "server")]
mod auth;
mod components;
mod views;

#[cfg(feature = "server")]
use crate::auth::{BiteFluentAuthAdapter, BiteFluentSessionStore, Db};
use crate::views::AppLayout;
use crate::views::AppPlatform;
use crate::views::Home;
use crate::views::Start;
use dioxus::logger::tracing::Level;
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/app")]
        AppPlatform {},
        #[route("/start")]
        Start {},
}
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FAVICON: Asset = asset!("/assets/favicon/favicon-32x32.png");

fn main() {
    let _ = dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG);

    #[cfg(feature = "server")]
    {
        dioxus::serve(server_router);
    }

    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
    }
}

#[cfg(feature = "server")]
async fn server_router() -> Result<axum::Router, anyhow::Error> {
    use dioxus_auth::prelude::*;
    use tower_cookies::CookieManagerLayer;

    dotenvy::dotenv().ok();

    let github_client_id = std::env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID is required");

    let github_client_secret =
        std::env::var("GITHUB_CLIENT_SECRET").expect("GITHUB_CLIENT_SECRET is required");

    let mut config = AuthConfig::new(
        std::env::var("AUTH_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
    );

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");

    let client = byteorm_client::Client::new(&*database_url).await?;

    let db = Db::new(client);

    config.cookie.secure = false;
    config.cookie.session_cookie_name = "bitefluent.session".to_string();
    config.cookie.state_cookie_name = "bitefluent.oauth_state".to_string();

    let auth = Auth::new(
        config,
        BiteFluentAuthAdapter::new(db.clone()),
        BiteFluentSessionStore::new(db.clone()),
    )
    .with_provider(GitHubProvider::new(github_client_id, github_client_secret));

    let auth_router = dioxus_auth::axum::router(auth);

    let app = dioxus::server::router(App)
        .nest("/auth", auth_router)
        .layer(CookieManagerLayer::new());

    Ok(app)
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: FAVICON }
        Router::<Route> {}
    }
}
