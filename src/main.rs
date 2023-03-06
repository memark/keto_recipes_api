use axum::{ routing::get, http::StatusCode, Json, Router };
use serde::{ Serialize };
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root)).route("/recipes", get(get_recipes));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on http://{}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "Keto Recipes API is alive and kicking!"
}

async fn get_recipes() -> (StatusCode, Json<Vec<Recipe>>) {
    let recipe = Recipe {
        id: "1337".into(),
        name: "biffsallad".into(),
        ingress: "jättegod biffsallad".into(),
        image_url: "someUrl".into(),
        steps: vec![],
        ingredients: vec![Ingredient {
            name: "biff".into(),
            amount: Amount { quantity: 1, unit: "kg".into() },
        }],
    };
    let recipes = vec![recipe];

    (StatusCode::OK, Json(recipes))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Recipe {
    id: String,
    name: String,
    ingress: String,
    image_url: String,
    steps: Vec<String>,
    ingredients: Vec<Ingredient>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Ingredient {
    name: String,
    amount: Amount,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Amount {
    quantity: u8,
    unit: String,
}