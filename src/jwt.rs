use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Result};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::fs::read_to_string;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims { pub id: String, pub exp: usize }


// Calculate the JWT Expiry date (30 days from generation)
fn calculate_jwt_expiry() -> usize {
    let now = SystemTime::now();
    let thirty_days = Duration::from_secs(30 * 24 * 60 * 60);
    let expiry_date = now.checked_add(thirty_days);
    expiry_date.unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs().try_into().unwrap() 
    // Literally no reason for this to fail (famous last words)
}

fn get_secret_key() -> Vec<u8> {
    let envfile = read_to_string(".env").expect("Could not find .env");
    let secret_key: &[u8] = envfile
        .lines()
        .filter(|line| line.starts_with("JWT_SECRET_KEY="))
        .next()
        .expect("Could not find JWT_SECRET_KEY in .env")
        .as_bytes();
    secret_key.to_vec()
}

pub fn create_jwt(id: &str) -> Result<String> {
    let exp = calculate_jwt_expiry();
    let claims = Claims { id: id.to_owned(), exp: exp };
    let secret_key = get_secret_key();
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_slice()))
}

pub fn validate_jwt(token: &str) -> Result<Claims> {
    let validation = Validation::new(Algorithm::HS256);
    let secret_key = get_secret_key();
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret_key.as_slice()), &validation)?;
    Ok(token_data.claims)
}

