use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::data::DataResponse;

const AUTH_TOKEN_LENGTH: usize = 64;

/// Get a secure token for session tokens
pub fn get_auth_token() -> String {
    rand::rngs::OsRng
        .sample_iter(&Alphanumeric)
        .take(AUTH_TOKEN_LENGTH)
        .map(char::from)
        .collect::<String>()
}

pub fn sample(n: Option<u32>, data: Vec<DataResponse>) -> Vec<DataResponse> {
    if let Some(n) = n {
        if n > data.len() as u32 {
            return data;
        }
        let mut sampled = Vec::with_capacity(n as usize);
        let ratio = n as f64 / data.len() as f64;
        let mut acc = 0.0;

        for d in data.into_iter() {
            acc += ratio;
            if acc > 1.0 {
                sampled.push(d);
                acc -= 1.0;
            }
        }

        sampled
    } else {
        data
    }
}
