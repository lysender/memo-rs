use askama::Template;
use axum::{
    extract::{Form, State},
    http::Response,
    response::IntoResponse,
};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};
use validator::Validate;

use crate::{
    models::{Actor, Pref},
    run::AppState,
};
use crate::{
    models::{LoginFormPayload, TemplateData},
    services::{authenticate, validate_catpcha, AuthPayload},
    Error,
};

use super::{ErrorInfo, AUTH_TOKEN_COOKIE};

#[derive(Template)]
#[template(path = "pages/login.html")]
struct LoginTemplate {
    t: TemplateData,
    captcha_key: String,
    error_message: Option<String>,
}

#[derive(Template)]
#[template(path = "widgets/login_form.html")]
struct SubmitLoginData {
    error_message: Option<String>,
    captcha_key: String,
}

pub async fn login_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pref = Pref::new();
    let actor: Option<Actor> = None;
    let mut t = TemplateData::new(&state, actor, &pref);
    t.title = String::from("Login");
    t.async_scripts = vec![String::from(
        "https://www.google.com/recaptcha/api.js?onload=onloadCallbackRecaptcha&render=explicit",
    )];

    let config = state.config.clone();
    let captcha_key = config.captcha_site_key.clone();

    let tpl = LoginTemplate {
        t,
        captcha_key,
        error_message: None,
    };

    Response::builder()
        .status(200)
        .header("Surrogate-Control", "no-store")
        .header(
            "Cache-Control",
            "no-store, no-cache, must-revalidate, proxy-revalidate",
        )
        .header("Pragma", "no-cache")
        .header("Expires", 0)
        .body(tpl.render().unwrap())
        .unwrap()
}

pub async fn post_login_handler(
    cookies: Cookies,
    State(state): State<AppState>,
    Form(login_payload): Form<LoginFormPayload>,
) -> impl IntoResponse {
    let config = state.config.clone();
    let captcha_key = config.captcha_site_key.clone();
    let captcha_secret = config.captcha_site_secret.clone();

    // Validate data
    if let Err(err) = login_payload.validate() {
        let errors: Vec<&str> = err
            .field_errors()
            .keys()
            .map(|k| match *k {
                "g-recaptcha-response" => "captcha",
                other => other,
            })
            .collect();
        let mut error_message = "Invalid username or password.".to_string();
        if errors.contains(&"captcha") {
            error_message = "Click the I'm not a robot checkbox.".to_string();
        }
        return handle_error(state, Error::ValidationError(error_message));
    }

    // Validate captcha
    if let Err(captcha_err) =
        validate_catpcha(&captcha_secret, login_payload.g_recaptcha_response.as_str()).await
    {
        return handle_error(state, captcha_err);
    }

    // Validate login information
    let auth_payload = AuthPayload {
        username: login_payload.username,
        password: login_payload.password,
    };
    let login_result = authenticate(&config.api_url, auth_payload).await;
    let auth = match login_result {
        Ok(val) => val,
        Err(err) => {
            return handle_error(state, err);
        }
    };

    let tpl = SubmitLoginData {
        captcha_key,
        error_message: None,
    };

    let auth_cookie = Cookie::build((AUTH_TOKEN_COOKIE, auth.token.clone()))
        .http_only(true)
        .max_age(Duration::weeks(1))
        .secure(state.config.ssl)
        .path("/")
        .build();

    cookies.add(auth_cookie);

    Response::builder()
        .status(200)
        .header("HX-Redirect", "/")
        .body(tpl.render().unwrap())
        .unwrap()
}

fn handle_error(state: AppState, error: Error) -> Response<String> {
    let config = state.config.clone();
    let error_info: ErrorInfo = error.into();
    let tpl = SubmitLoginData {
        captcha_key: config.captcha_site_key.clone(),
        error_message: Some(error_info.message),
    };
    Response::builder()
        .status(error_info.status_code)
        .body(tpl.render().unwrap())
        .unwrap()
}
