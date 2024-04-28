use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: &u16) -> Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    info!("Serving {:?} on {}", &path, addr);
    let state = HttpServeState { path: path.clone() };
    let dir_service = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir_service)
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file{:?}", p);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        );
    } else {
        // TODO: test p is a directory
        // if it is a directory, list all files/subdirectories
        // as <li><a href="/path/to/file">file</a><li>
        // <html><body><ul>...</ul></body></html>
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(err) => {
                warn!("Error reading file: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            path: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert
        !(content.contains("[package]"));
    }
}
