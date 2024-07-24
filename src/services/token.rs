use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_csrf_token(subject: &str, secret: &str) -> Result<String> {
    // Limit up to 1 hour only
    let exp = Utc::now() + Duration::hours(1);

    let claims = Claims {
        sub: subject.to_string(),
        exp: exp.timestamp() as usize,
    };

    let Ok(token) = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) else {
        return Err("Error creating JWT token".into());
    };

    Ok(token)
}

pub fn verify_csrf_token(token: &str, secret: &str) -> Result<String> {
    let Ok(decoded) = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) else {
        return Err(Error::InvalidCsrfToken);
    };

    if decoded.claims.sub.len() == 0 {
        return Err(Error::InvalidCsrfToken);
    }

    Ok(decoded.claims.sub)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_token() {
        // Generate token
        let token = create_csrf_token("example", "secret").unwrap();
        assert!(token.len() > 0);
        println!("Token: {}", token);

        // Validate it back
        let value = verify_csrf_token(&token, "secret").unwrap();
        assert_eq!(value, "example".to_string());
    }

    #[test]
    fn test_expired_token() {
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJleGFtcGxlIiwiZXhwIjoxNzIxMDk1MDIyfQ.7ddeJN3Tys_8kc8a02umkNLv42lPHIoSDaqmi-WjRhE".to_string();
        let result = verify_csrf_token(&token, "secret");
        assert!(result.is_err());
    }
}
