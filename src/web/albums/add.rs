use askama::Template;
use axum::http::StatusCode;
use axum::{body::Body, extract::State, response::Response, Extension, Form};

use crate::models::NewAlbumForm;
use crate::run::AppState;
use crate::services::create_csrf_token;
use crate::{ctx::Ctx, models::TemplateData, services::create_album};

use crate::web::{enforce_policy, handle_error, Action, ErrorInfo, Resource};

#[derive(Template)]
#[template(path = "pages/new_album.html")]
struct NewAlbumTemplate {
    t: TemplateData,
    action: String,
    payload: NewAlbumForm,
    error_message: Option<String>,
}

#[derive(Template)]
#[template(path = "widgets/new_album_form.html")]
struct AlbumFormTemplate {
    action: String,
    payload: NewAlbumForm,
    error_message: Option<String>,
}

pub async fn new_album_handler(
    Extension(ctx): Extension<Ctx>,
    State(state): State<AppState>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();

    if let Err(err) = enforce_policy(actor, Resource::Album, Action::Create) {
        return handle_error(&state, Some(actor.clone()), err.into(), true);
    }

    let mut t = TemplateData::new(&state, Some(actor.clone()));
    t.title = String::from("Create New Album");

    let Ok(token) = create_csrf_token("new_album", &config.jwt_secret) else {
        let error = ErrorInfo::new("Failed to initialize new album form.".to_string());
        return handle_error(&state, Some(actor.clone()), error, true);
    };

    let tpl = NewAlbumTemplate {
        t,
        action: "/albums/new".to_string(),
        payload: NewAlbumForm {
            name: "".to_string(),
            label: "".to_string(),
            token,
        },
        error_message: None,
    };

    Response::builder()
        .status(200)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}

pub async fn post_new_album_handler(
    Extension(ctx): Extension<Ctx>,
    State(state): State<AppState>,
    payload: Option<Form<NewAlbumForm>>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();

    if let Err(err) = enforce_policy(actor, Resource::Album, Action::Create) {
        return handle_error(&state, Some(actor.clone()), err.into(), false);
    }

    let Ok(token) = create_csrf_token("new_album", &config.jwt_secret) else {
        let error = ErrorInfo::new("Failed to initialize new album form.".to_string());
        return handle_error(&state, Some(actor.clone()), error, true);
    };

    let mut tpl = AlbumFormTemplate {
        action: "/albums/new".to_string(),
        payload: NewAlbumForm {
            name: "".to_string(),
            label: "".to_string(),
            token,
        },
        error_message: None,
    };

    let mut status: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;

    if let Some(form) = payload {
        let album = NewAlbumForm {
            name: form.name.clone(),
            label: form.label.clone(),
            token: form.token.clone(),
        };

        let result = create_album(&config, ctx.token(), &config.bucket_id, album).await;

        match result {
            Ok(album) => {
                let next_url = format!("/albums/{}", &album.id);
                // Weird but can't do a redirect here, let htmx handle it
                return Response::builder()
                    .status(200)
                    .header("HX-Redirect", next_url)
                    .body(Body::from("".to_string()))
                    .unwrap();
            }
            Err(err) => {
                let error_info: ErrorInfo = err.into();
                status = error_info.status_code;
                tpl.error_message = Some(error_info.message);
            }
        }

        tpl.payload.name = form.name.clone();
        tpl.payload.label = form.label.clone();
    } else {
        tpl.error_message = Some("Invalid form data. Refresh the page and try again.".to_string());
    }

    // Will only arrive here on error
    Response::builder()
        .status(status)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}
