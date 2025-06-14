use crate::domain::value_objects::Name;

pub struct GreetingService;

impl GreetingService {
    pub fn new() -> Self {
        Self
    }

    pub fn greet(&self, name: &Name) -> String {
        format!("Hello, {}!", name.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let service = GreetingService::new();
        let name = Name::new("World".to_string()).unwrap();

        assert_eq!(service.greet(&name), "Hello, World!");
    }
}
