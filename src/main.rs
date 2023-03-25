mod data;
use data::*;

use axum::{ routing::get, http::StatusCode, Json, Router };
use tower_http::compression::CompressionLayer;
use std::net::SocketAddr;
use tower_http::cors::{ CorsLayer, Any };
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let port = std::env::var("PORT").unwrap_or("3000".into()).parse::<u16>().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let router = create_router();

    tracing::info!("listening on http://localhost:{port}");
    axum::Server::bind(&addr).serve(router.into_make_service()).await.unwrap();
}

fn create_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/recipes", get(get_recipes))
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
}

async fn root() -> &'static str {
    "Keto Recipes API is alive and kicking!"
}

async fn get_recipes() -> (StatusCode, Json<Vec<Recipe>>) {
    let recipe1 = Recipe {
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
    let recipe2 = Recipe {
        id: "1338".into(),
        name: "Kålpudding".into(),
        ingress: "Mustig kålpudding som sköter sig själv i ugnen.".into(),
        image_url: "https://assets.icanet.se/e_sharpen:80,q_auto,dpr_1.25,w_718,h_718,c_lfill/imagevaultfiles/id_143436/cf_259/kalpudding_(grundrecept).jpg".into(),
        steps: vec!["Blanda alla ingredienser".into(), "Ät".into()],
        ingredients: vec![
            Ingredient {
                name: "kål".into(),
                amount: Amount { quantity: 1, unit: "kg".into() },
            },
            Ingredient {
                name: "pudding".into(),
                amount: Amount { quantity: 1, unit: "kg".into() },
            }
        ],
    };
    let recipes = vec![recipe1, recipe2];

    (StatusCode::OK, Json(recipes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{ body::Body, http::{ Request, StatusCode, HeaderValue } };
    use pretty_assertions::assert_eq;
    use tower::ServiceExt;

    #[tokio::test]
    async fn root_should_return_ok() {
        let router = create_router();

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_return_cors_header() {
        let router = create_router();

        let response = router
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await
            .unwrap();

        assert_eq!(response.headers().get("access-control-allow-origin").unwrap(), "*");
    }

    #[tokio::test]
    async fn get_recipes_should_return_ok_and_data() {
        let router = create_router();

        let response = router
            .oneshot(Request::builder().uri("/recipes").body(Body::empty()).unwrap()).await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn should_compress_data_correctly() {
        let response = create_router()
            .clone()
            .oneshot(Request::builder().uri("/recipes").body(Body::empty()).unwrap()).await
            .unwrap();
        assert_eq!(response.headers().contains_key("content-encoding"), false);

        let response = create_router()
            .oneshot(
                Request::builder()
                    .uri("/recipes")
                    .header("accept-encoding", "gzip")
                    .body(Body::empty())
                    .unwrap()
            ).await
            .unwrap();
        assert_eq!(
            response.headers().get("content-encoding").unwrap_or(&HeaderValue::from_static("")),
            "gzip"
        );
    }
}