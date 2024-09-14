use askama::Template;
use axum::http::{Method, StatusCode};
use axum::Form;
use axum::{body::Body, extract::State, response::Response, Extension};

use crate::models::{DeleteAlbumForm, Pref};
use crate::run::AppState;
use crate::services::{create_csrf_token, delete_album};
use crate::Error;
use crate::{ctx::Ctx, models::Album};

use crate::web::{enforce_policy, handle_error, Action, ErrorInfo, Resource};

#[derive(Template)]
#[template(path = "widgets/delete_album_form.html")]
struct DeleteAlbumTemplate {
    album: Album,
    payload: DeleteAlbumForm,
    error_message: Option<String>,
}

/// Deletes album then redirect or show error
pub async fn delete_album_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(pref): Extension<Pref>,
    Extension(album): Extension<Album>,
    State(state): State<AppState>,
    method: Method,
    payload: Option<Form<DeleteAlbumForm>>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();
    let default_bucket_id = actor.default_bucket_id.clone();
    let Some(bucket_id) = default_bucket_id else {
        return handle_error(
            &state,
            Some(actor.clone()),
            &pref,
            Error::NoDefaultBucket.into(),
            false,
        );
    };

    if let Err(err) = enforce_policy(actor, Resource::Album, Action::Delete) {
        return handle_error(&state, Some(actor.clone()), &pref, err.into(), false);
    }

    let Ok(token) = create_csrf_token(&album.id, &config.jwt_secret) else {
        let error = ErrorInfo::new("Failed to initialize delete album form.".to_string());
        return handle_error(&state, Some(actor.clone()), &pref, error, true);
    };

    let mut error_message: Option<String> = None;
    let mut status_code: StatusCode = StatusCode::OK;

    if method == Method::POST {
        if let Some(form) = payload {
            let result =
                delete_album(&config, ctx.token(), &bucket_id, &album.id, &form.token).await;
            match result {
                Ok(_) => {
                    // Render same form but trigger a redirect to home
                    let tpl = DeleteAlbumTemplate {
                        album,
                        payload: DeleteAlbumForm {
                            token: "".to_string(),
                        },
                        error_message,
                    };
                    return Response::builder()
                        .status(200)
                        .header("HX-Redirect", "/")
                        .body(Body::from(tpl.render().unwrap()))
                        .unwrap();
                }
                Err(err) => {
                    let error_info: ErrorInfo = err.into();
                    error_message = Some(error_info.message);
                    status_code = error_info.status_code;
                }
            }
        } else {
            status_code = StatusCode::BAD_REQUEST;
            error_message = Some("Invalid form data. Refresh the page and try again.".to_string());
        }
    }

    // Just render the form on first load or on error
    let tpl = DeleteAlbumTemplate {
        album,
        payload: DeleteAlbumForm { token },
        error_message,
    };

    Response::builder()
        .status(status_code)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}
