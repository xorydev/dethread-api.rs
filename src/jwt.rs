use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Result};
use std::time::{SystemTime, Duration, UNIX_EPOCH};

const SECRET_KEY: &[u8] = b"temp";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims { pub id: String, pub exp: usize }

fn calculate_jwt_expiry() -> usize {
    let now = SystemTime::now();
    let thirty_days = Duration::from_secs(30 * 24 * 60 * 60);
    let expiry_date = now.checked_add(thirty_days);
    expiry_date.unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap() 
    // Literally no reason for this to fail (famous last words)
}

pub fn create_jwt(id: &str) -> Result<String> {
    let exp = calculate_jwt_expiry();
    let claims = Claims { id: id.to_owned(), exp: exp };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
    }

pub fn validate_jwt(token: &str) -> Result<Claims> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation)?;
    Ok(token_data.claims)
}

