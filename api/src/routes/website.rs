use std::sync::{Arc, Mutex};

use crate::request_inputs::CreateWebsiteInput;
use crate::request_outputs::{CreateWebsiteOutput, GetWebsiteOutput};
use poem::{
    handler,
    web::{Data, Json, Path},
};
use store::store::Store;

#[handler]
pub fn get_website(
    Path(id): Path<String>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<GetWebsiteOutput> {
    let mut locked_s = s.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let website = locked_s.get_website(id).unwrap();
    Json(GetWebsiteOutput { url: website.url })
}

#[handler]
pub fn create_website(
    Json(data): Json<CreateWebsiteInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateWebsiteOutput> {
    let mut locked_s = match s.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let website = locked_s
        .create_website(
            String::from("b4d6e7d1-0d0e-40bd-a199-1aa5d5fcf7cf"),
            data.url,
        )
        .expect("Failed to create website");

    let response = CreateWebsiteOutput { id: website.id };
    Json(response)
}
