use std::sync::{Arc, Mutex};

use crate::request_inputs::CreateUserInput;
use crate::request_outputs::{CreateUserOutput, SigninOutput};
use poem::{
    handler,
    web::{Data, Json},
};
use store::store::Store;

#[handler]
pub fn sign_up(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<CreateUserOutput> {
    // Handle poisoned mutex by recovering the data
    let mut locked_s = s.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    // Handle sign_up error (adjust default value based on your ID type)
    let id = locked_s
        .sign_up(data.username, data.password)
        .unwrap_or_else(|_| String::from("signup_failed")); // Adjust type as needed

    let response = CreateUserOutput { id };
    Json(response)
}

#[handler]
pub fn sign_in(
    Json(data): Json<CreateUserInput>,
    Data(s): Data<&Arc<Mutex<Store>>>,
) -> Json<SigninOutput> {
    let mut locked_s = s.lock().unwrap_or_else(|poisoned| {
        eprintln!("Mutex was poisoned in sign_in, recovering...");
        poisoned.into_inner()
    });

    let _exists = locked_s
        .sign_in(data.username, data.password)
        .expect("Failed to sign in user");

    let response = SigninOutput {
        jwt: String::from("harkirtat"),
    };

    Json(response)
}
