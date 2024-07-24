use askama::Template;
use axum::{body::Body, extract::State, response::Response, Extension, Form};

use crate::models::UpdateAlbumForm;
use crate::run::AppState;
use crate::services::{create_csrf_token, update_album};
use crate::{ctx::Ctx, models::Album, Error};

use crate::web::{enforce_policy, handle_error, Action, ErrorInfo, Resource};

#[derive(Template)]
#[template(path = "widgets/edit_album_form.html")]
struct EditAlbumFormTemplate {
    payload: UpdateAlbumForm,
    album: Album,
    error_message: Option<String>,
    updated: bool,
}

#[derive(Template)]
#[template(path = "widgets/edit_album_controls.html")]
struct EditAlbumControlsTemplate {
    album: Album,
    updated: bool,
    can_edit: bool,
    can_delete: bool,
}

/// Simply re-renders the edit and delete album controls
pub async fn edit_album_controls_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(album): Extension<Album>,
) -> Response<Body> {
    let tpl = EditAlbumControlsTemplate {
        album,
        updated: false,
        can_edit: enforce_policy(ctx.actor(), Resource::Album, Action::Update).is_ok(),
        can_delete: enforce_policy(ctx.actor(), Resource::Album, Action::Delete).is_ok(),
    };

    Response::builder()
        .status(200)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}

/// Renders the edit album form
pub async fn edit_album_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(album): Extension<Album>,
    State(state): State<AppState>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();

    if let Err(err) = enforce_policy(actor, Resource::Album, Action::Update) {
        return handle_error(&state, Some(actor.clone()), err.into(), false);
    }
    let Ok(token) = create_csrf_token(&album.id, &config.jwt_secret) else {
        let error = ErrorInfo::new("Failed to initialize edit album form.".to_string());
        return handle_error(&state, Some(actor.clone()), error, true);
    };

    let label = album.label.clone();
    let tpl = EditAlbumFormTemplate {
        album,
        payload: UpdateAlbumForm { label, token },
        error_message: None,
        updated: false,
    };

    Response::builder()
        .status(200)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}

/// Handles the edit album submission
pub async fn post_edit_album_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(album): Extension<Album>,
    State(state): State<AppState>,
    payload: Option<Form<UpdateAlbumForm>>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();
    let album_id = album.id.clone();

    if let Err(err) = enforce_policy(actor, Resource::Album, Action::Update) {
        return handle_error(&state, Some(actor.clone()), err.into(), false);
    }
    let Ok(token) = create_csrf_token(&album.id, &config.jwt_secret) else {
        let error = ErrorInfo::new("Failed to initialize edit album form.".to_string());
        return handle_error(&state, Some(actor.clone()), error, true);
    };

    let mut tpl = EditAlbumFormTemplate {
        album,
        payload: UpdateAlbumForm {
            label: "".to_string(),
            token,
        },
        error_message: None,
        updated: false,
    };

    let mut status = 200;
    match payload {
        Some(form) => {
            tpl.payload.label = form.label.clone();

            let result =
                update_album(&config, ctx.token(), &config.bucket_id, &album_id, &form).await;
            match result {
                Ok(updated_album) => {
                    tpl.album = updated_album;
                    tpl.updated = true;
                }
                Err(err) => match err {
                    Error::ValidationError(msg) => {
                        status = 400;
                        tpl.error_message = Some(msg);
                    }
                    Error::LoginRequired(msg) => {
                        status = 401;
                        tpl.error_message = Some(msg);
                    }
                    any_err => {
                        status = 500;
                        tpl.error_message = Some(any_err.to_string());
                    }
                },
            }
        }
        None => {
            status = 400;
            tpl.error_message = Some("Invalid form data.".to_string());
        }
    };

    if tpl.updated {
        // Render the controls again with an out-of-bound swap for title
        let tpl = EditAlbumControlsTemplate {
            album: tpl.album,
            updated: true,
            can_edit: enforce_policy(actor, Resource::Album, Action::Update).is_ok(),
            can_delete: enforce_policy(actor, Resource::Album, Action::Delete).is_ok(),
        };
        Response::builder()
            .status(status)
            .body(Body::from(tpl.render().unwrap()))
            .unwrap()
    } else {
        Response::builder()
            .status(status)
            .body(Body::from(tpl.render().unwrap()))
            .unwrap()
    }
}
