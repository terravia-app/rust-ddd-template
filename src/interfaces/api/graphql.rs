use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::infrastructure::graphql::AppSchema;

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub fn create_router(schema: AppSchema) -> Router {
    Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::graphql::create_schema;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_graphql_playground() {
        let response = graphql_playground().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_graphql_endpoints() {
        // Test playground endpoint
        let schema = create_schema();
        let app = create_router(schema.clone());

        let playground_request = Request::builder()
            .uri("/graphql")
            .method("GET")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(playground_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        // Test graphql query endpoint (create a new app instance)
        let app = create_router(schema);
        let query = r#"{"query": "{ hello(name: \"Test\") { message } }"}"#;
        let graphql_request = Request::builder()
            .uri("/graphql")
            .method("POST")
            .header("content-type", "application/json")
            .body(Body::from(query))
            .unwrap();

        let response = app.oneshot(graphql_request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
