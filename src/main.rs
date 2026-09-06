// The sample entity is deliberately named `Foo`, which is exactly the
// placeholder name this lint exists to catch. Rename it away when you fork
// this boilerplate into a real service, then drop this allow.
#![allow(clippy::disallowed_names)]

use std::net::SocketAddr;
use axum::{Router, routing::get};
use tower_http::{
  cors::CorsLayer,
  trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
  infra::{
    config::load, gateway::AwsMediaGateway,
    persistence::pg::repository::PgFooRepository, use_case::FooUseCase,
    worker::FooWorker,
  },
  presentation::{
    api_doc::ApiDoc, controller::foo_controller, state::AppState, utils::worker,
  },
};

mod app;
mod domain;
mod infra;
mod presentation;

#[tokio::main]
async fn main() {
  // Load config
  let app_config = load().await;

  // Initialize AppState
  let app_state = AppState::new(app_config)
    .await
    .expect("AppState initialization failed");

  // Initialize workers listeners
  app_worker_listener(app_state.clone());

  // Initialize server listener
  let socket_addr: SocketAddr =
    format!("{}:{}", app_config.host, app_config.port)
      .parse()
      .expect("Failed to parse socket address");

  let listener = tokio::net::TcpListener::bind(socket_addr)
    .await
    .expect("Listener to be bind");

  tracing::info!("listening on http://{}", socket_addr);

  axum::serve(listener, app_router(app_state))
    .await
    .expect("Server to be started");
}

fn app_worker_listener(state: AppState) {
  let media_gateway = AwsMediaGateway::new(state.aws_s3_config);

  worker::spawn(FooWorker::new(media_gateway), state.foo_command_bus);
}

fn app_router(state: AppState) -> Router {
  let foo_use_case =
    FooUseCase::new(PgFooRepository::new(state.pool), state.foo_command_bus);

  let api_router = Router::new()
    .route("/", get(root))
    .nest("/foo", foo_controller::routes(foo_use_case))
    .layer(CorsLayer::very_permissive())
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO)),
    );

  let mut router = Router::new().merge(api_router);

  // Create a separate docs router with Swagger UI
  if state.openapi_enabled {
    router = router.merge(
      SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()),
    );
  }

  router
}

// basic handler that responds with a static string
async fn root() -> &'static str {
  "Hello, World!"
}
