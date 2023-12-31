use axum::Router;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};
mod user;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_filter(LevelFilter::INFO),
        )
        .init();

    let app: Router = web::route::get_router().await;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::user::user::{User, UserRole, UserStatus};
    use crate::web::route::get_router;

    use axum::http::Method;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use chrono::{Duration, DurationRound};
    use http_body_util::BodyExt; // for `collect`
    use serde_json::json;

    use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn test_create_delete_select_user() {
        let mut app = get_router().await.into_service();
        let now = chrono::Utc::now();
        let now_string = &now.format("%Y-%m-%dT%H:%M:%S%Z").to_string();

        let create_req = Request::builder()
            .uri("/user")
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::to_vec(&json!({
                        "id":0,
                        "name": "test-user",
                        "email": "test-user@example.com",
                        "password": "password",
                        "role": "Admin",
                        "status": "Inactive",
                        "created_at": now_string
                }))
                .unwrap(),
            ))
            .unwrap();
        let create_rsp = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(create_req)
            .await
            .unwrap();

        assert_eq!(create_rsp.status(), StatusCode::OK);
        let create_rsp_bytes = create_rsp.into_body().collect().await.unwrap().to_bytes();
        let create_rsp_str = std::str::from_utf8(&create_rsp_bytes).unwrap();
        let create_rsp_object: User = serde_json::from_str(create_rsp_str).unwrap();
        assert_eq!(create_rsp_object.name, "test-user");
        assert_eq!(create_rsp_object.email, "test-user@example.com");
        assert_eq!(create_rsp_object.password, "password");
        assert_eq!(create_rsp_object.role, UserRole::User);
        assert_eq!(create_rsp_object.status, UserStatus::Active);
        assert!(
            create_rsp_object.created_at - now.duration_round(Duration::seconds(1)).unwrap()
                < Duration::seconds(1)
        );

        let delete_req = Request::builder()
            .uri(format!("/user/{}", create_rsp_object.id))
            .method(Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let delete_rsp = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(delete_req)
            .await
            .unwrap();
        assert_eq!(delete_rsp.status(), StatusCode::OK);

        let read_req = Request::builder()
            .uri(format!("/user/{}", create_rsp_object.id))
            .method(Method::GET)
            .body(Body::empty())
            .unwrap();
        let read_rsp = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(read_req)
            .await
            .unwrap();
        assert_eq!(read_rsp.status(), StatusCode::OK);
        let read_rsp_bytes = read_rsp.into_body().collect().await.unwrap().to_bytes();
        let read_rsp_object: User =
            serde_json::from_str(std::str::from_utf8(&read_rsp_bytes).unwrap()).unwrap();
        assert_eq!(read_rsp_object.status, UserStatus::Inactive);
    }
}
