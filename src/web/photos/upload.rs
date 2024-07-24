use askama::Template;
use axum::body::Bytes;
use axum::extract::Query;
use axum::http::HeaderMap;
use axum::{body::Body, extract::State, response::Response, Extension};

use crate::models::UploadParams;
use crate::run::AppState;
use crate::services::{create_csrf_token, upload_photo};
use crate::web::{handle_error, handle_error_message, ErrorInfo};
use crate::{
    ctx::Ctx,
    models::{Album, Photo, TemplateData},
};

use crate::web::policies::{enforce_policy, Action, Resource};

#[derive(Template)]
#[template(path = "pages/upload_photos.html")]
struct UploadPageTemplate {
    t: TemplateData,
    token: String,
    album: Album,
}

#[derive(Template)]
#[template(path = "widgets/photo_grid_item.html")]
struct UploadedPhotoTemplate {
    photo: Photo,
}

pub async fn upload_page_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(album): Extension<Album>,
    State(state): State<AppState>,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();

    if let Err(err) = enforce_policy(actor, Resource::Photo, Action::Create) {
        return handle_error(&state, Some(actor.clone()), err.into(), true);
    }
    let Ok(token) = create_csrf_token(&album.id, &config.jwt_secret) else {
        let error = ErrorInfo::new("Failed to initialize upload photos form.".to_string());
        return handle_error(&state, Some(actor.clone()), error, true);
    };
    let mut t = TemplateData::new(&state, Some(actor.clone()));

    t.title = format!("Photos - {} - Upload Photos", &album.label);
    t.scripts = vec![config.assets.upload_js.clone()];

    let tpl = UploadPageTemplate { t, token, album };

    Response::builder()
        .status(200)
        .body(Body::from(tpl.render().unwrap()))
        .unwrap()
}

pub async fn upload_handler(
    Extension(ctx): Extension<Ctx>,
    Extension(album): Extension<Album>,
    State(state): State<AppState>,
    Query(query): Query<UploadParams>,
    headers: HeaderMap,
    body: Bytes,
) -> Response<Body> {
    let config = state.config.clone();
    let actor = ctx.actor();
    let Ok(token) = create_csrf_token(&album.id, &config.jwt_secret) else {
        let error = ErrorInfo::new("Failed to initialize upload photos form.".to_string());
        return handle_error(&state, Some(actor.clone()), error, true);
    };

    let result = upload_photo(
        &config,
        ctx.token(),
        &config.bucket_id,
        &album.id,
        &headers,
        query.token,
        body,
    )
    .await;

    match result {
        Ok(photo) => {
            let tpl = UploadedPhotoTemplate { photo };
            Response::builder()
                .status(201)
                .header("X-Next-Token", token)
                .body(Body::from(tpl.render().unwrap()))
                .unwrap()
        }
        Err(err) => handle_error_message(err),
    }
}
