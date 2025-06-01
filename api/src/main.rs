use input::CreateWebsite;
use poem::{
    EndpointExt, Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path},
};
use store::{models::website::Website, store::Store};
pub mod input;

#[handler]
fn get_website(Path(website_id): Path<String>, store: Data<&Store>) -> Json<Website> {
    let website = store.0.get_website(website_id).unwrap();
    Json(website)
}

#[handler]
fn create_website(website_input: Json<CreateWebsite>, store: Data<&Store>) -> Json<Website> {
    let website = store.0.create_website(website_input.url.clone()).unwrap();
    Json(website)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv::from_filename(".env").map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to load .env file: {}", e),
        )
    })?;

    let store = Store::new().await.map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize store: {}", e),
        )
    })?;
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .data(store);
    Server::new(TcpListener::bind("0.0.0.0:3002"))
        .run(app)
        .await
}
