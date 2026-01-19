use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use gymbuddy::api::routes::create_api_router;
use gymbuddy::db::connection::{establish_connection, init_db};

#[tokio::test]
async fn health_check_works() {
    // Load env
    dotenvy::dotenv().ok();

    let pool = establish_connection().await;
    init_db(&pool).await.unwrap();

    let app = create_api_router().with_state(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}