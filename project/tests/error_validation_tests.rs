use dmnapi::models::{ErrorResponse, ValidationErrorResponse};
use std::collections::HashMap;

#[test]
fn test_error_response_serialization() {
    let err = ErrorResponse {
        code: "bad.request".to_string(),
        message: "Bad request".to_string(),
        description: "Invalid input".to_string(),
    };
    let json = serde_json::to_string(&err).unwrap();
    assert!(json.contains("bad.request"));
    assert!(json.contains("Invalid input"));
}

#[test]
fn test_validation_error_response_serialization() {
    let mut errors = HashMap::new();
    errors.insert("field1".to_string(), vec!["must not be empty".to_string()]);
    let err = ValidationErrorResponse {
        code: "validation.error".to_string(),
        message: "Validation failed".to_string(),
        description: "Some fields are invalid".to_string(),
        errors,
    };
    let json = serde_json::to_string(&err).unwrap();
    assert!(json.contains("validation.error"));
    assert!(json.contains("field1"));
}

#[test]
fn test_validation_error_response_code_is_constant() {
    let err = ValidationErrorResponse {
        code: "validation.error".to_string(),
        message: "Validation failed".to_string(),
        description: "desc".to_string(),
        errors: HashMap::new(),
    };
    assert_eq!(err.code, "validation.error");
}
