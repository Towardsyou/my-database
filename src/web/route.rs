use crate::user::user::User;
use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use http_body_util::BodyExt;
use sqlx::SqlitePool;

const DB_URL: &str = "sqlite://data.db";

pub fn get_router() -> Router {
    Router::new()
        .layer(middleware::from_fn(print_request_response))
        .route("/", post(get_user))
}

async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print("request", body).await?;
    let req = Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(direction: &str, body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::info!("{direction} body = {body:?}");
    }

    Ok(bytes)
}

async fn get_user() -> String {
    let db: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await.unwrap();
    let user = sqlx::query_as::<_, User>("SELECT * FROM user")
        .fetch_all(&db)
        .await
        .unwrap();
    format!("{:?}", user)
}
