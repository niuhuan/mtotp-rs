pub use error::*;
pub use database::join_paths;
use std::collections::HashMap;

mod database;
pub mod error;
pub mod otp;

pub type Register = database::registered::Model;

pub async fn init(folder: String) {
    database::init(folder).await;
}

pub async fn print_codes() -> Vec<PrintCode> {
    let mut codes: Vec<PrintCode> = Vec::new();
    let registered_codes = database::registered::find_all().await;
    for x in registered_codes {
        let label = x.label;
        let uuid = x.uuid;
        match otp::totp(x.secret.as_str(), 30, 6) {
            Ok(code) => {
                codes.push(PrintCode { uuid, label, code });
            }
            Err(err) => {
                codes.push(PrintCode {
                    uuid,
                    label,
                    code: match err {
                        Error::Hmac(_) => "HMac key error".to_owned(),
                        Error::Message(msg) => msg.content,
                        Error::Other(_) => "Exception".to_owned(),
                    },
                });
            }
        };
    }
    codes
}

pub async fn registered_codes() -> Vec<Register> {
    database::registered::find_all().await
}

pub async fn save_url(url: String) -> Result<PrintCode> {
    let url = match url::Url::parse(url.as_str()) {
        Ok(url) => url,
        Err(err) => {
            return Err(Error::message(format!("Invalid url : {}", err)));
        }
    };
    let path_split = url.path().split("/");
    let label = path_split.last().expect("").to_owned();
    let mut query_map: HashMap<String, String> = HashMap::new();
    for (key, value) in url.query_pairs() {
        query_map.insert(key.to_string(), value.to_string());
    }
    let secret = query_map
        .get::<String>(&("secret".to_string()))
        .expect("secret not found")
        .to_string();
    let issuer = match query_map.get::<String>(&("issuer".to_string())) {
        None => "".to_string(),
        Some(so) => so.to_string(),
    };
    Ok(save_register(label, secret, issuer).await?)
}

pub async fn save_register(label: String, secret: String, issuer: String) -> Result<PrintCode> {
    let code = match otp::totp(secret.as_str(), 30, 6) {
        Ok(code) => code,
        Err(err) => {
            return Err(err);
        }
    };
    let registered = database::registered::Model {
        uuid: uuid::Uuid::new_v4().to_string(),
        label: label.clone(),
        secret,
        issuer,
    };
    database::registered::insert(registered.clone()).await;
    Ok(PrintCode {
        uuid: registered.uuid,
        label: registered.label,
        code,
    })
}

pub fn register_to_url(label: String, secret: String, issuer: String) -> Result<String> {
    let url = format!("otpauth://totp/{}?secret={}&issuer={}", label, secret, issuer);
    Ok(url)
}

pub async fn remove(uuid: &str) {
    database::registered::delete_by_uuid(uuid).await;
}

pub async fn rename(uuid: &str, label: &str) {
    database::registered::update_label_by_uuid(uuid, label).await;
}

pub struct PrintCode {
    pub uuid: String,
    pub label: String,
    pub code: String,
}
