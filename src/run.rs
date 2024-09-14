use std::sync::Arc;

use axum::extract::FromRef;
use axum::{middleware, Router};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

use crate::config::Config;
use crate::web::{assets_routes, pref_middleware, private_routes, public_routes, routes_fallback};
use crate::Result;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Arc<Config>,
}

pub async fn run(config: Config) -> Result<()> {
    let port = config.port;
    let frontend_dir = config.frontend_dir.clone();
    let state = AppState {
        config: Arc::new(config),
    };

    let routes_all = Router::new()
        .merge(private_routes(state.clone()))
        .merge(public_routes(state.clone()))
        .merge(assets_routes(&frontend_dir))
        .fallback_service(routes_fallback(state))
        .route_layer(middleware::from_fn(pref_middleware))
        .layer(CookieManagerLayer::new())
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(DefaultOnResponse::new().level(Level::INFO)),
            ),
        );

    // Setup the server
    let ip = "127.0.0.1";
    let addr = format!("{}:{}", ip, port);
    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
