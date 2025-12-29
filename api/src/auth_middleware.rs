use jsonwebtoken::{DecodingKey, Validation, decode};
use poem::http::StatusCode;
use poem::{Error, FromRequest, Result};

use crate::routes::user::Claims;

pub struct UserId(pub String);

impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a poem::Request, body: &mut poem::RequestBody) -> Result<Self> {
        let token = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("Missing token", StatusCode::UNAUTHORIZED))?;

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::UNAUTHORIZED))?;

        Ok(UserId(token_data.claims.sub))
    }
}
