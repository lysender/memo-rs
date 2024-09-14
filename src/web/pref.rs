use axum::{body::Body, extract::State, response::Response, Form};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

use crate::{models::Pref, run::AppState};

use super::THEME_COOKIE;

pub async fn theme_handler(
    cookies: Cookies,
    State(state): State<AppState>,
    payload: Form<Pref>,
) -> Response<Body> {
    let mut theme: String = "light".to_string();
    let t_val = payload.theme.as_str();
    if t_val == "dark" || t_val == "light" {
        theme = t_val.to_string();
    }

    let theme_cookie = Cookie::build((THEME_COOKIE, theme))
        .http_only(true)
        .max_age(Duration::weeks(999))
        .secure(state.config.ssl)
        .build();

    cookies.add(theme_cookie);

    // Render the selected theme button
    // then let the frontend switch the theme

    todo!()
}
