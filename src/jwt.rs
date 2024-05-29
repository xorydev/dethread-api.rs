use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Result};

const SECRET_KEY: &[u8] = b"temp";

#[derive(Debug, Serialize, Deserialize)]
struct Claims { id: String }

pub fn create_jwt(id: &str) -> Result<String> {
    let claims = Claims { id: id.to_owned() };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
    }

pub fn validate_jwt(token: &str) -> Result<Claims> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation)?;
    Ok(token_data.claims)
}

