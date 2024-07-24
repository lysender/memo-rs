use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::Response,
    Extension,
};

use crate::{
    ctx::Ctx,
    models::AlbumParams,
    run::AppState,
    services::get_album,
    web::{enforce_policy, handle_error, Action, Resource},
};

pub async fn album_listing_middleware(
    State(state): State<AppState>,
    Extension(ctx): Extension<Ctx>,
    req: Request,
    next: Next,
) -> Response {
    // Ensure that users has access to albums and everything under it
    let full_page = req.headers().get("HX-Request").is_none();
    if let Err(err) = enforce_policy(ctx.actor(), Resource::Album, Action::Read) {
        return handle_error(&state, Some(ctx.actor().clone()), err.into(), full_page);
    }

    next.run(req).await
}

pub async fn album_middleware(
    State(state): State<AppState>,
    Extension(ctx): Extension<Ctx>,
    Path(params): Path<AlbumParams>,
    mut req: Request,
    next: Next,
) -> Response {
    let full_page = req.headers().get("HX-Request").is_none();
    if let Err(err) = enforce_policy(ctx.actor(), Resource::Photo, Action::Read) {
        return handle_error(&state, Some(ctx.actor().clone()), err.into(), full_page);
    }

    let album_id = params.album_id.expect("album_id is required");
    let result = get_album(
        &state.config.api_url,
        ctx.token(),
        &state.config.bucket_id,
        &album_id,
    )
    .await;

    match result {
        Ok(album) => {
            req.extensions_mut().insert(album);
        }
        Err(err) => {
            return handle_error(&state, Some(ctx.actor().clone()), err.into(), full_page);
        }
    };

    next.run(req).await
}
