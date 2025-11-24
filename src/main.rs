use std::{
    collections::HashMap,
    sync::{Arc, Mutex}
};
use hvalfangst_rust_crud_with_axum::{
    users::{router::users_routes, model::User}
};

#[tokio::main]
async fn main() {

    // Arc<Mutex> is necessary as our HashMap will be mutated across threads
    let hashmap: Arc<Mutex<HashMap<String, User>>> = Arc::new(Mutex::new(HashMap::new()));

    // Port 80 is chosen due to the very fact that Azure Container Instances targets this
    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(users_routes(hashmap).into_make_service())
        .await
        .unwrap();
}





