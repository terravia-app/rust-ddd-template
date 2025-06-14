use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(String);

#[derive(Debug, Error)]
pub enum NameError {
    #[error("Name cannot be empty")]
    Empty,
    #[error("Name is too long (maximum 100 characters)")]
    TooLong,
}

impl Name {
    pub fn new(value: String) -> Result<Self, NameError> {
        if value.trim().is_empty() {
            return Err(NameError::Empty);
        }

        if value.len() > 100 {
            return Err(NameError::TooLong);
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.value()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = Name::new("John".to_string()).unwrap();
        assert_eq!(name.value(), "John");
    }

    #[test]
    fn test_empty_name() {
        let result = Name::new("".to_string());
        assert!(matches!(result, Err(NameError::Empty)));
    }

    #[test]
    fn test_too_long_name() {
        let long_name = "a".repeat(101);
        let result = Name::new(long_name);
        assert!(matches!(result, Err(NameError::TooLong)));
    }

    #[test]
    fn test_as_ref() {
        let name = Name::new("John".to_string()).unwrap();
        let name_ref: &str = name.as_ref();
        assert_eq!(name_ref, "John");
    }
}
