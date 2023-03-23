mod data;
use data::*;

use axum::{ routing::get, http::StatusCode, Json, Router };
use std::net::SocketAddr;
use tower_http::cors::{ CorsLayer, Any };
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let app = Router::new()
        .route("/", get(root))
        .route("/recipes", get(get_recipes))
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("PORT").unwrap_or("3000".into()).parse::<u16>().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("listening on http://localhost:{port}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "Keto Recipes API is alive and kicking!"
}

async fn get_recipes() -> (StatusCode, Json<Vec<Recipe>>) {
    let recipe = Recipe {
        id: "1337".into(),
        name: "Biffsallad".into(),
        ingress: "Jättegod biffsallad".into(),
        image_url: "https://img.koket.se/standard-mega/jennie-walldens-thailandska-biffsallad.jpg".into(),
        steps: vec!["Blanda alla ingredienser".into(), "Ät".into()],
        ingredients: vec![Ingredient {
            name: "biff".into(),
            amount: Amount { quantity: 1, unit: "kg".into() },
        }],
    };
    let recipes = vec![recipe];

    (StatusCode::OK, Json(recipes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}