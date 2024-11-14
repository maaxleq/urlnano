use axum::{extract::Path, response::Redirect, Extension, Json};
use nanoid::nanoid;

use crate::{
    axum_extension::RepositoryExtension,
    dto::{GetUrlResponse, SetUrlRequest, SetUrlResponse},
    error::HttpError,
};

pub async fn set_url(
    Extension(repo_ext): Extension<RepositoryExtension>,
    Json(request): Json<SetUrlRequest>,
) -> Json<SetUrlResponse> {
    let repository = repo_ext.as_ref();
    let id = nanoid!();
    repository.set(&id, &request.full_url).await;
    Json(SetUrlResponse { short_url: id })
}

pub async fn get_url(
    Path(key): Path<String>,
    Extension(repo_ext): Extension<RepositoryExtension>,
) -> Result<Json<GetUrlResponse>, HttpError> {
    let repository = repo_ext.as_ref();
    let full_url = repository.get(&key).await;
    match full_url {
        Some(full_url) => Ok(Json(GetUrlResponse { full_url })),
        None => Err(HttpError::not_found("URL not found")),
    }
}

pub async fn redirect(
    Path(key): Path<String>,
    Extension(repo_ext): Extension<RepositoryExtension>,
) -> Result<Redirect, HttpError> {
    let repository = repo_ext.as_ref();
    let full_url = repository.get(&key).await;
    match full_url {
        Some(full_url) => Ok(Redirect::permanent(&full_url)),
        None => Err(HttpError::not_found("URL not found")),
    }
}
