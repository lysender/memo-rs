use askama::Template;
use axum::http::StatusCode;
use axum::Form;
use axum::{body::Body, extract::State, response::Response, Extension};

use crate::models::{DeletePhotoForm, Photo};
use crate::run::AppState;
use crate::services::{create_csrf_token, delete_photo};
use crate::{ctx::Ctx, models::Album, Error};

use crate::web::{enforce_policy, handle_error_message, Action, ErrorInfo, Resource};

#[derive(Template)]
#[template(path = "widgets/pre_delete_photo_form.html")]
struct PreDeletePhotoTemplate {
    photo: Photo,
}

#[derive(Template)]
#[template(path = "widgets/confirm_delete_photo_form.html")]
struct ConfirmDeletePhotoTemplate {
    photo: Photo,
    payload: DeletePhotoForm,
    error_message: Option<String>,
}

/// Shows pre-delete form controls
pub async fn pre_delete_photo_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(photo): Extension<Photo>,
) -> Response<Body> {
    let actor = ctx.actor();

    if let Err(err) = enforce_policy(actor, Resource::Photo, Action::Delete) {
        return handle_error_message(err);
    }

    // Just render the form on first load or on error
    let tpl = PreDeletePhotoTemplate { photo };

    Response::builder()
        .status(200)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}

/// Shows delete/cancel form controls
pub async fn confirm_delete_photo_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(photo): Extension<Photo>,
    State(state): State<AppState>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();

    if let Err(err) = enforce_policy(actor, Resource::Photo, Action::Delete) {
        return handle_error_message(err);
    }

    let Ok(token) = create_csrf_token(&photo.id, &config.jwt_secret) else {
        return handle_error_message(Error::AnyError(
            "Failed to initialize delete photo form.".into(),
        ));
    };

    // Just render the form on first load or on error
    let tpl = ConfirmDeletePhotoTemplate {
        photo,
        payload: DeletePhotoForm { token },
        error_message: None,
    };

    Response::builder()
        .status(200)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}

pub async fn exec_delete_photo_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(album): Extension<Album>,
    Extension(photo): Extension<Photo>,
    State(state): State<AppState>,
    payload: Option<Form<DeletePhotoForm>>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();
    let default_bucket_id = actor.default_bucket_id.clone();
    let Some(bucket_id) = default_bucket_id else {
        return handle_error_message(Error::NoDefaultBucket);
    };

    if let Err(err) = enforce_policy(actor, Resource::Photo, Action::Delete) {
        return handle_error_message(err);
    }

    let Ok(token) = create_csrf_token(&photo.id, &config.jwt_secret) else {
        return handle_error_message(Error::AnyError(
            "Failed to initialize delete photo form.".to_string(),
        ));
    };

    let mut status_code = StatusCode::BAD_REQUEST;
    let mut error_message = Some("Invalid form data. Refresh the page and try again.".to_string());

    if let Some(form) = payload {
        let result = delete_photo(
            &config,
            ctx.token(),
            &bucket_id,
            &album.id,
            &photo.id,
            &form.token,
        )
        .await;
        match result {
            Ok(_) => {
                return Response::builder()
                    .status(204)
                    .header("HX-Trigger", "PhotoDeletedEvent")
                    .body(Body::from("".to_string()))
                    .unwrap();
            }
            Err(err) => {
                let error_info: ErrorInfo = err.into();
                error_message = Some(error_info.message);
                status_code = error_info.status_code;
            }
        }
    }

    // Re-render the form with a new token
    // We may need to render an error message somewhere in the page
    let tpl = ConfirmDeletePhotoTemplate {
        photo,
        payload: DeletePhotoForm { token },
        error_message,
    };

    Response::builder()
        .status(status_code)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}
