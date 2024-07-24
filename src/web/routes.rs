use axum::extract::DefaultBodyLimit;
use axum::routing::{get, get_service, post};
use axum::{middleware, Router};
use std::path::PathBuf;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::services::{ServeDir, ServeFile};

use crate::run::AppState;
use crate::web::{
    error_handler, index_handler, login_handler, logout_handler, new_album_handler,
    photo_listing_handler, photos_page_handler, post_login_handler, post_new_album_handler,
};

use super::{
    album_listing_handler, album_listing_middleware, album_middleware,
    confirm_delete_photo_handler, delete_album_handler, edit_album_controls_handler,
    edit_album_handler, exec_delete_photo_handler, photo_middleware, post_edit_album_handler,
    pre_delete_photo_handler, require_auth_middleware, upload_handler, upload_page_handler,
};

pub fn assets_routes(dir: &PathBuf) -> Router {
    let target_dir = dir.join("public");
    Router::new()
        .route(
            "/manifest.json",
            get_service(ServeFile::new(target_dir.join("manifest.json"))),
        )
        .route(
            "/favicon.ico",
            get_service(ServeFile::new(target_dir.join("favicon.ico"))),
        )
        .nest_service(
            "/assets",
            get_service(ServeDir::new(target_dir.join("assets"))),
        )
}

pub fn private_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .nest("/albums", album_routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth_middleware,
        ))
        .with_state(state)
}

fn album_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/listing", get(album_listing_handler))
        .route("/new", get(new_album_handler).post(post_new_album_handler))
        .nest("/:album_id", album_inner_routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            album_listing_middleware,
        ))
        .with_state(state)
}

fn album_inner_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(photos_page_handler))
        .route("/edit-controls", get(edit_album_controls_handler))
        .route(
            "/edit",
            get(edit_album_handler).post(post_edit_album_handler),
        )
        .route(
            "/delete",
            get(delete_album_handler).post(delete_album_handler),
        )
        .route("/photo-grid", get(photo_listing_handler))
        .nest("/upload", upload_route(state.clone()))
        .nest("/photos/:photo_id", photo_routes(state.clone()))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            album_middleware,
        ))
        .with_state(state)
}

fn upload_route(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(upload_page_handler).post(upload_handler))
        .layer(DefaultBodyLimit::max(8000000))
        .layer(RequestBodyLimitLayer::new(8000000))
        .with_state(state)
}

fn photo_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/delete",
            get(confirm_delete_photo_handler).post(exec_delete_photo_handler),
        )
        .route("/delete-controls", get(pre_delete_photo_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            photo_middleware,
        ))
        .with_state(state)
}

pub fn public_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", get(login_handler).post(post_login_handler))
        .route("/logout", post(logout_handler))
        .with_state(state)
}

pub fn routes_fallback(state: AppState) -> Router {
    // 404 handler
    Router::new().nest_service("/", get(error_handler).with_state(state))
}
