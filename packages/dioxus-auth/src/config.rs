use time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthConfig {
    pub base_url: String,
    pub callback_path: String,
    pub sign_in_path: String,
    pub sign_out_path: String,
    pub cookie: CookieConfig,
}

impl AuthConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            callback_path: "/auth/callback".to_string(),
            sign_in_path: "/auth/signin".to_string(),
            sign_out_path: "/auth/signout".to_string(),
            cookie: CookieConfig::default(),
        }
    }

    pub fn callback_url(&self, provider: &str) -> String {
        format!(
            "{}{}/{}",
            self.base_url.trim_end_matches('/'),
            self.callback_path,
            provider
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieConfig {
    pub session_cookie_name: String,
    pub state_cookie_name: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSiteMode,
    pub path: String,
    pub session_max_age: Duration,
    pub state_max_age: Duration,
}

impl Default for CookieConfig {
    fn default() -> Self {
        Self {
            session_cookie_name: "dioxus_auth.session".to_string(),
            state_cookie_name: "dioxus_auth.state".to_string(),
            secure: true,
            http_only: true,
            same_site: SameSiteMode::Lax,
            path: "/".to_string(),
            session_max_age: Duration::days(30),
            state_max_age: Duration::minutes(10),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSiteMode {
    Lax,
    Strict,
    None,
}
