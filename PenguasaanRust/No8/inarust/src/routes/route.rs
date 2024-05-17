use axum::{
    routing::{get, post,delete},
    Router,
    extract::Extension,
};
use mongodb::Client;
use std::sync::Arc;

use crate::controllers;

pub fn route(client: Arc<Client>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .route("/create-user", post(controllers::create_user))
        .route("/add-user", post(controllers::add_user))
        .route("/users", get(controllers::list_users))
        .route("/get-users", get(controllers::get_users))
        .route("/item/:id", get(controllers::show_item))
        .route("/add-item", post(controllers::add_item))
        .route("/delete-user/:user_id", delete(controllers::delete_user))
        .layer(Extension(client))
}
