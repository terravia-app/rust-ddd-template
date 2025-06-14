use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};

use crate::application::dtos::GreetingRequest;
use crate::application::use_cases::GreetingUseCase;
use crate::domain::services::GreetingService;

#[derive(SimpleObject)]
struct GreetingResult {
    message: String,
}

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, _ctx: &Context<'_>, name: String) -> Result<GreetingResult, String> {
        let greeting_service = GreetingService::new();
        let greeting_use_case = GreetingUseCase::new(greeting_service);

        let request = GreetingRequest { name };

        match greeting_use_case.execute(request) {
            Ok(response) => Ok(GreetingResult {
                message: response.message,
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hello_query_with_valid_name() {
        let schema = create_schema();
        let query = "{ hello(name: \"World\") { message } }";

        let res = schema.execute(query).await;

        assert!(res.is_ok());
        // Use contains instead of exact match to avoid formatting issues
        assert!(res.data.to_string().contains("Hello, World!"));
    }

    #[tokio::test]
    async fn test_hello_query_with_empty_name() {
        let schema = create_schema();
        let query = "{ hello(name: \"\") { message } }";

        let res = schema.execute(query).await;

        assert!(res.errors.len() > 0);
        let error_message = &res.errors[0].message;
        assert!(error_message.contains("Name cannot be empty"));
    }
}
