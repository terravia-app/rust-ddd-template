use crate::application::dtos::{GreetingRequest, GreetingResponse};
use crate::domain::services::GreetingService;
use crate::domain::value_objects::{Name, NameError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GreetingUseCaseError {
    #[error("Invalid name: {0}")]
    InvalidName(#[from] NameError),
}

pub struct GreetingUseCase {
    greeting_service: GreetingService,
}

impl GreetingUseCase {
    pub fn new(greeting_service: GreetingService) -> Self {
        Self { greeting_service }
    }

    pub fn execute(
        &self,
        request: GreetingRequest,
    ) -> Result<GreetingResponse, GreetingUseCaseError> {
        let name = Name::new(request.name)?;
        let message = self.greeting_service.greet(&name);

        Ok(GreetingResponse { message })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting_use_case() {
        let greeting_service = GreetingService::new();
        let use_case = GreetingUseCase::new(greeting_service);

        let request = GreetingRequest {
            name: "World".to_string(),
        };

        let response = use_case.execute(request).unwrap();
        assert_eq!(response.message, "Hello, World!");
    }

    #[test]
    fn test_greeting_use_case_empty_name() {
        let greeting_service = GreetingService::new();
        let use_case = GreetingUseCase::new(greeting_service);

        let request = GreetingRequest {
            name: "".to_string(),
        };

        let result = use_case.execute(request);
        assert!(result.is_err());
    }
}
