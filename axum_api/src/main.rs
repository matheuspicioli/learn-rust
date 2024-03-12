use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
extern crate mongodb;
use mongodb::{bson::doc, options::ClientOptions, Client};

#[derive(Clone)]
struct AppState {
    mongo_client: Client,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Mongodb configuration
    let mongo_client_options =
        ClientOptions::parse(std::env::var("MONGODB_URL").expect("Define MONGODB_URL env var"))
            .await
            .unwrap();
    let mongo_client = Client::with_options(mongo_client_options).unwrap();
    initialize_mongo_configurations(&mongo_client, "rust_axum_with_mongo_api", "users").await;

    // Axum server configuration & start
    let app = Router::new()
        .route("/", get(health_check))
        .route("/user", post(create_user))
        .with_state(AppState { mongo_client });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// @TODO: make this function returns a client pre configureated with database name, probably
// creating a mongo mod to centralize all these logics
async fn initialize_mongo_configurations(client: &Client, db_name: &str, coll_name: &str) {
    let db = client.database(db_name);
    // create collection - i am going to use this?
    db.create_collection(coll_name, None).await.unwrap();
}

// @TODO: use the mongo mod or something like that removing the mongodb parameters
async fn insert_mongo_user(client: &Client, db_name: &str, coll_name: &str, user: &User) {
    let db = client.database(db_name);
    let coll = db.collection(coll_name);

    let doc = doc! { "username": &user.username };
    coll.insert_one(doc, None).await.unwrap();
}

// @TODO: create health check mod and move these struct/fn for there
async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck {
        message: String::from("Application is healthy!!!"),
    })
}

#[derive(Serialize)]
struct HealthCheck {
    message: String,
}

async fn create_user(
    state: State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    insert_mongo_user(
        &state.mongo_client,
        &String::from("rust_axum_with_mongo_api"),
        &String::from("users"),
        &user,
    )
    .await;

    (StatusCode::CREATED, Json(user))
}

// @TODO: create User mod to centralize all these struct/enum/impl
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
