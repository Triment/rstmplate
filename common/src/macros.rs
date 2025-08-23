#[macro_export]
macro_rules! create_asset_handler {
    ($folder:expr) => {
        {
            #[derive(rust_embed::RustEmbed)]
            #[folder = $folder] 
            struct Assets;

            async move |Path(path): Path<String>| -> Response {
                match Assets::get(path.as_str()) {
                  Some(content) => {
                    let mime = mime_guess::from_path(path).first_or_octet_stream();
                    ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
                  }
                  None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
                }
            }
        }
    };
}
