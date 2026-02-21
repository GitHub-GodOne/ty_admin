use axum::{
    extract::Request,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use include_dir::{include_dir, Dir};

static FRONTEND_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/frontend/dist");

pub async fn serve_embedded(req: Request) -> Response {
    let path = req.uri().path().trim_start_matches('/');

    // Try to find the file in the embedded directory
    if let Some(file) = FRONTEND_DIR.get_file(path) {
        return serve_file(path, file.contents());
    }

    // SPA fallback: return index.html for non-file paths
    match FRONTEND_DIR.get_file("index.html") {
        Some(file) => {
            let body = file.contents();
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                    (header::CACHE_CONTROL, "no-cache"),
                ],
                body,
            )
                .into_response()
        }
        None => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
    }
}

fn serve_file(path: &str, body: &'static [u8]) -> Response {
    let mime = mime_from_path(path);

    // Hashed assets (js/css with content hash in filename) get long cache
    // index.html gets no-cache so updates are picked up immediately
    let cache = if path == "index.html" {
        "no-cache"
    } else if path.contains("/assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "public, max-age=3600"
    };

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, mime),
            (header::CACHE_CONTROL, cache),
        ],
        body,
    )
        .into_response()
}

fn mime_from_path(path: &str) -> &'static str {
    match path.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("eot") => "application/vnd.ms-fontobject",
        Some("webp") => "image/webp",
        Some("mp4") => "video/mp4",
        Some("webm") => "video/webm",
        Some("txt") => "text/plain; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        _ => "application/octet-stream",
    }
}
