use mongodb::{options::ClientOptions, Client};
use std::{env, sync::Arc};
pub struct AppConfig {
    pub server_address: String,
    pub arc_client: Arc<Client>,
    // ... other configuration parameters ...
}

pub async fn load_config() -> AppConfig {
    // Set up the MongoDB connection options
    let client_options = ClientOptions::parse(env::var("MONGOSTRING").unwrap_or_default())
        .await
        .unwrap();
    // Connect to the MongoDB server
    let client = Client::with_options(client_options).unwrap();
    let arc_client = Arc::new(client);
    // Logic to load configuration from a file, environment variables, etc.
    AppConfig {
        server_address: env::var("SERVERADDRESS").unwrap_or_default().to_string(),
        arc_client: arc_client,
        // ... initialize other configuration parameters ...
    }
}
