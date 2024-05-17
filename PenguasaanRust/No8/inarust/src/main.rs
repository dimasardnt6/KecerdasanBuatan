mod routes;
mod controllers;
mod models;
mod configs;


#[tokio::main]
async fn main() {
    //Load config
    let config = configs::load_config().await;
  
    // Start Server
    println!("Running on {}",&config.server_address);
    let listener = tokio::net::TcpListener::bind(&config.server_address).await.unwrap();
    let app = routes::route(config.arc_client);
    axum::serve(listener, app).await.unwrap();
}
