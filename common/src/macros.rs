#[macro_export]
macro_rules! create_asset_handler {
    ($folder:expr) => {{
        #[derive(rust_embed::RustEmbed)]
        #[folder = $folder]
        struct Assets;
        use axum::extract::Path;
        use axum::http::header;
        use axum::response::{IntoResponse, Response};
        async move |axum::extract::Path(path): axum::extract::Path<String>| -> Response {
            match Assets::get(path.as_str()) {
                Some(content) => {
                    let mime = mime_guess::from_path(path).first_or_octet_stream();
                    ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
                }
                None => {
                    if let Some(index) = Assets::get("index.html") {
                        let mime = mime_guess::from_path("index.html").first_or_octet_stream();
                        Response::builder()
                            .status(axum::http::StatusCode::OK)
                            .header(header::CONTENT_TYPE, mime.as_ref())
                            .body(index.data.into())
                            .unwrap()
                    } else {
                        axum::http::StatusCode::NOT_FOUND.into_response()
                    }
                }
            }
        }
    }};
}
