use std::sync::Arc;

use crate::user::user::{User, UserRole, UserStatus};
use axum::{
    body::{Body, Bytes},
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use http_body_util::BodyExt;
use sqlx::SqlitePool;

const DB_URL: &str = "sqlite://data.db";

struct AppState {
    db_pool: SqlitePool,
}

pub async fn get_router() -> Router {
    let db: sqlx::Pool<sqlx::Sqlite> = SqlitePool::connect(DB_URL).await.unwrap();
    let shared_state = Arc::new(AppState { db_pool: db });

    Router::new()
        .route("/user/:user_id", get(get_user_by_id))
        .route("/user/:user_id", delete(delete_user_by_id))
        .route("/user", post(create_user))
        .with_state(shared_state)
        .layer(middleware::from_fn(print_request_response))
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

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> Result<Json<User>, AppError> {
    let db = &state.db_pool;
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO user (name, email, password, role, status, created_at)
        VALUES ($1, $2, $3, $4, $5, current_timestamp) RETURNING *",
    )
    .bind(user.name)
    .bind(user.email)
    .bind(user.password)
    .bind(UserRole::User)
    .bind(UserStatus::Active)
    .fetch_one(db)
    .await?;
    Ok(Json(user))
}

async fn delete_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<(), AppError> {
    let db = &state.db_pool;
    sqlx::query("UPDATE user SET status=$1 WHERE id=$2")
        .bind(UserStatus::Inactive)
        .bind(user_id)
        .execute(db)
        .await?;
    Ok(())
}

async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, AppError> {
    let db = &state.db_pool;
    let user = sqlx::query_as::<_, User>("SELECT * FROM user where id=$1")
        .bind(user_id)
        .fetch_one(db)
        .await?;
    Ok(Json(user))
}
