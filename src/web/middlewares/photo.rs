use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
    Extension,
};

use crate::{
    ctx::Ctx,
    models::PhotoParams,
    run::AppState,
    services::get_photo,
    web::{enforce_policy, handle_error, Action, Resource},
};

pub async fn photo_middleware(
    State(state): State<AppState>,
    Extension(ctx): Extension<Ctx>,
    Path(params): Path<PhotoParams>,
    mut req: Request,
    next: Next,
) -> Response {
    let full_page = req.headers().get("HX-Request").is_none();
    if let Err(err) = enforce_policy(ctx.actor(), Resource::Photo, Action::Read) {
        return handle_error(&state, Some(ctx.actor().clone()), err.into(), full_page);
    }

    let album_id = params.album_id.expect("album_id is required");
    let photo_id = params.photo_id.expect("photo_id is required");

    let config = state.config.clone();
    let result = get_photo(
        &config.api_url,
        ctx.token(),
        &config.bucket_id,
        &album_id,
        &photo_id,
    )
    .await;

    match result {
        Ok(photo) => {
            req.extensions_mut().insert(photo);
        }
        Err(err) => {
            return handle_error(&state, Some(ctx.actor().clone()), err.into(), full_page);
        }
    };

    next.run(req).await
}
