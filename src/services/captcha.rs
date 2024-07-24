use reqwest::Client;
use serde::Deserialize;

use crate::{Error, Result};

const VERIFY_URL: &str = "https://www.google.com/recaptcha/api/siteverify";

#[derive(Deserialize)]
struct CaptchaResponse {
    success: bool,
}

pub async fn validate_catpcha(secret: &str, response: &str) -> Result<()> {
    let post_body = [("secret", secret), ("response", response)];

    let result = Client::new().post(VERIFY_URL).form(&post_body).send().await;
    let Ok(response) = result else {
        return Err("Unable to validate captcha. Try again later.".into());
    };
    if !response.status().is_success() {
        return Err("Unable to validate captcha. Try again later.".into());
    }
    let captcha_res = response.json::<CaptchaResponse>().await;
    match captcha_res {
        Ok(captcha_res) => {
            if captcha_res.success {
                Ok(())
            } else {
                Err(Error::InvalidCaptcha("Invalid captcha.".to_string()))
            }
        }
        Err(_) => Err(Error::JsonParseError(
            "Unable to validate captcha. Try again later.".to_string(),
        )),
    }
}
