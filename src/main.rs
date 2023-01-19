mod settings;
mod templates;

use settings::Settings;
use templates::MyTemplate;

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

use tracing::{info, warn};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    static ref SETTINGS: Settings = match Settings::new() {
        Some(s) => s,
        _ => {
            warn!("Failed to parse settings, defaults will be used instead");
            Settings::from_str("").unwrap()
        }
    };
}

#[tokio::main]
async fn main() {
    // Initialize logging subsystem.
    let trace_sub = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new("askama_axum_rust_template=debug"))
        .finish();
    tracing::subscriber::set_global_default(trace_sub).unwrap();

    let app = Router::new()
        .route("/", get(handle_main))
        .route("/_assets/*path", get(handle_assets));

    let listen_addr: SocketAddr = format!("{}:{}", SETTINGS.ip, SETTINGS.port)
        .parse()
        .unwrap();

    info!("Listening on http://{}", listen_addr);

    axum::Server::bind(&listen_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

static THEME_CSS: &str = include_str!("../assets/theme.css");
static FAVICON: &str = include_str!("../assets/favicon.svg");

async fn handle_assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    if path == "theme.css" {
        headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
        (StatusCode::OK, headers, THEME_CSS)
    } else if path == "favicon.svg" {
        (StatusCode::OK, headers, FAVICON)
    } else {
        (StatusCode::NOT_FOUND, headers, "")
    }
}

async fn handle_main() -> impl IntoResponse {
    let template = MyTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
