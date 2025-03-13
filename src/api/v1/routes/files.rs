use axum::{
    extract::{DefaultBodyLimit, Multipart, Path},
    http::HeaderMap,
    response::IntoResponse,
    routing::{get, post},
    Router
};

pub fn config() -> Router<()> {
    Router::new()
        .route("/v1/files", post(upload_file))
        .nest(
            "/v1/files",
            Router::new()
                .route("/{id}", get(download_file).delete(delete_file))
        )
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024 * 1024))
}



async fn upload_file(
    headers: HeaderMap,
    mut multipart: Multipart
) -> impl IntoResponse {

}



async fn download_file(
    headers: HeaderMap,
    Path(file_id): Path<String>
) -> impl IntoResponse {

}



async fn delete_file(
    headers: HeaderMap,
    Path(file_id): Path<String>
) -> impl IntoResponse {

}