use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};
use request_inputs::{CreateUserInput, CreateWebsiteInput};
use request_outputs::{CreateUserOutput, CreateWebsiteOutput, GetWebsiteOutput, SigninOutput};
use store::store::Store;
pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(id): Path<String>) -> Json<GetWebsiteOutput> {
    let mut s = Store::new().unwrap();
    let website = s.get_website(id).unwrap();
    Json(GetWebsiteOutput{
        url: website.url
    })
}

#[handler]
fn sign_up(Json(data): Json<CreateUserInput>) -> Json<CreateUserOutput> {
    let mut s = Store::new().unwrap();
    let id = s.sign_up(data.username, data.password).unwrap();

    let response = CreateUserOutput {
        id
    };
    
    Json(response)
}

#[handler]
fn sign_in(Json(data): Json<CreateUserInput>) -> Json<SigninOutput> {
    let mut s = Store::new().unwrap();
    let _exists = s.sign_in(data.username, data.password).unwrap();

    let response = SigninOutput {
        jwt: String::from("harkirtat")
    };
    
    Json(response)
}

#[handler]
fn create_website(Json(data): Json<CreateWebsiteInput>) -> Json<CreateWebsiteOutput> {
    let mut s = Store::new().unwrap();
    let website = s.create_website(String::from("dd020379-1e62-44b2-8a3d-c4e17c30d044"), data.url).unwrap();

    let response = CreateWebsiteOutput {
        id: website.id
    };
    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/website/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/user/signup", post(sign_up))
        .at("/user/signin", post(sign_in));
    // creates and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}