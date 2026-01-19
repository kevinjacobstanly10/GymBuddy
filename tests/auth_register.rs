use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use gymbuddy::api::routes::create_api_router;
use gymbuddy::db::connection::{establish_connection, init_db};

#[tokio::test]
async fn user_can_register() {
    dotenvy::dotenv().ok();

    let pool = establish_connection().await;
    init_db(&pool).await.unwrap();

    let app = create_api_router().with_state(pool);

    let payload = r#"
    {
        "username": "testuser",
        "email": "testuser@example.com",
        "password": "password123"
    }
    "#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/register")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}