use poem::{
    EndpointExt, Route, Server, get, handler,
    listener::TcpListener,
    post,
    web::{Data, Json, Path},
};

use store::{
    Store,
    models::{Region, Website},
};

mod input;

#[handler]
async fn get_websites(store: Data<&Store>) -> Json<Vec<Website>> {
    let websites = store.get_websites().await.unwrap_or_default();
    Json(websites)
}

#[handler]
async fn get_website(store: Data<&Store>, id: Path<String>) -> Json<Option<Website>> {
    let website = store.get_website(&id).await.ok();
    Json(website)
}

#[handler]
async fn create_website(
    store: Data<&Store>,
    input: Json<input::CreateWebsite>,
) -> Result<Json<Website>, poem::Error> {
    let website = store
        .create_website(&input.url, input.name.as_deref())
        .await
        .map_err(|e| poem::error::InternalServerError(e))?;
    Ok(Json(website))
}

#[handler]
async fn update_website(
    store: Data<&Store>,
    id: Path<String>,
    input: Json<input::UpdateWebsite>,
) -> Json<Option<Website>> {
    let website = store
        .update_website(&id, input.url.as_deref(), input.name.as_deref())
        .await
        .ok();
    Json(website)
}

#[handler]
async fn delete_website(store: Data<&Store>, id: Path<String>) -> String {
    match store.delete_website(&id).await {
        Ok(_) => "Website deleted".to_string(),
        Err(_) => "Website not found".to_string(),
    }
}

#[handler]
async fn get_regions(store: Data<&Store>) -> Json<Vec<Region>> {
    let regions = store.get_regions().await.unwrap_or_default();
    Json(regions)
}

#[handler]
async fn create_region(store: Data<&Store>, input: Json<input::CreateRegion>) -> Json<Region> {
    let region = store.create_region(&input.name).await.unwrap();
    Json(region)
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
        .at("/websites", get(get_websites).post(create_website))
        .at(
            "/websites/:id",
            get(get_website)
                .patch(update_website)
                .delete(delete_website),
        )
        .at("/regions", post(create_region).get(get_regions))
        .data(store);

    Server::new(TcpListener::bind("0.0.0.0:3002"))
        .run(app)
        .await
}
