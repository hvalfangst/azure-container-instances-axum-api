use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use serde_json::json;
use hvalfangst_rust_crud_with_axum::users::{
    router::users_routes,
    model::User,
};

fn create_test_app() -> axum::Router {
    let hashmap: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    users_routes(hashmap)
}

async fn get_response_body<B>(body: B) -> String
where
    B: axum::body::HttpBody,
    B::Error: std::fmt::Debug,
{
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn test_create_user_success() {
    let app = create_test_app();

    let request_body = json!({
        "email": "jerry@seinfeld.com",
        "password": "whats_the_deal",
        "fullname": "Jerry Seinfeld",
        "role": "comedian"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = get_response_body(response.into_body()).await;
    let user: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert_eq!(user["email"], "jerry@seinfeld.com");
    assert_eq!(user["fullname"], "Jerry Seinfeld");
    assert_eq!(user["role"], "comedian");
    assert_eq!(user["id"], 1);
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    let app = create_test_app();

    let request_body = json!({
        "email": "newman-at-usps",
        "password": "hello_jerry",
        "fullname": "Newman",
        "role": "mailman"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = get_response_body(response.into_body()).await;
    let error: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert!(error["error"].as_str().unwrap().contains("Invalid input for field 'email'"));
}

#[tokio::test]
async fn test_create_duplicate_user() {
    let hashmap: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    let app = users_routes(hashmap);

    let request_body = json!({
        "email": "george@vandalayindustries.com",
        "password": "bosco123",
        "fullname": "George Costanza",
        "role": "architect"
    });

    // Create first user
    let response1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response1.status(), StatusCode::CREATED);

    // Try to create duplicate user
    let response2 = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(request_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::ALREADY_REPORTED);

    let body = get_response_body(response2.into_body()).await;
    let error: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert!(error["error"].as_str().unwrap().contains("already exists"));
}

#[tokio::test]
async fn test_get_user_success() {
    let hashmap: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    let app = users_routes(hashmap);

    // First create a user
    let create_body = json!({
        "email": "elaine@jpeterman.com",
        "password": "spongeworthy",
        "fullname": "Elaine Benes",
        "role": "editor"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then get the user
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/elaine@jpeterman.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = get_response_body(response.into_body()).await;
    let user: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert_eq!(user["email"], "elaine@jpeterman.com");
    assert_eq!(user["fullname"], "Elaine Benes");
}

#[tokio::test]
async fn test_get_user_not_found() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/susan@envelopes.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = get_response_body(response.into_body()).await;
    let error: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert!(error["error"].as_str().unwrap().contains("not found"));
}

#[tokio::test]
async fn test_update_user_success() {
    let hashmap: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    let app = users_routes(hashmap);

    // First create a user
    let create_body = json!({
        "email": "kramer@kramerica.com",
        "password": "giddyup123",
        "fullname": "Cosmo Kramer",
        "role": "entrepreneur"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then update the user
    let update_body = json!({
        "email": "kramer@kramerica.com",
        "password": "the_timeless_art_of_seduction",
        "fullname": "Cosmo Kramer",
        "role": "model"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/users/kramer@kramerica.com")
                .header("content-type", "application/json")
                .body(Body::from(update_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = get_response_body(response.into_body()).await;
    let user: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert_eq!(user["email"], "kramer@kramerica.com");
    assert_eq!(user["fullname"], "Cosmo Kramer");
    assert_eq!(user["role"], "model");
    assert_eq!(user["password"], "the_timeless_art_of_seduction");
}

#[tokio::test]
async fn test_update_user_not_found() {
    let app = create_test_app();

    let update_body = json!({
        "email": "leo@hellojerry.com",
        "password": "swarm_swarm",
        "fullname": "Uncle Leo",
        "role": "retiree"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/users/leo@hellojerry.com")
                .header("content-type", "application/json")
                .body(Body::from(update_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_user_success() {
    let hashmap: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    let app = users_routes(hashmap);

    // First create a user
    let create_body = json!({
        "email": "newman@usps.gov",
        "password": "when_you_control_the_mail",
        "fullname": "Newman",
        "role": "mailman"
    });

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Then delete the user
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/users/newman@usps.gov")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = get_response_body(response.into_body()).await;
    let result: serde_json::Value = serde_json::from_str(&body).unwrap();

    assert!(result["message"].as_str().unwrap().contains("deleted"));

    // Verify user is deleted by trying to get it
    let get_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/newman@usps.gov")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/users/peterman@catalog.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_full_crud_workflow() {
    let hashmap: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));
    let app = users_routes(hashmap);

    // 1. Create a user
    let create_body = json!({
        "email": "frank@festivus.com",
        "password": "serenity_now",
        "fullname": "Frank Costanza",
        "role": "salesman"
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(create_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);

    // 2. Read the user
    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/frank@festivus.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    // 3. Update the user
    let update_body = json!({
        "email": "frank@festivus.com",
        "password": "i_got_a_lot_of_problems_with_you_people",
        "fullname": "Frank Costanza",
        "role": "bra_inventor"
    });

    let update_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/users/frank@festivus.com")
                .header("content-type", "application/json")
                .body(Body::from(update_body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);

    // 4. Delete the user
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/users/frank@festivus.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::OK);

    // 5. Verify deletion
    let final_get_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users/frank@festivus.com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(final_get_response.status(), StatusCode::NOT_FOUND);
}
