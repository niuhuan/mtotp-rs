extern crate core;

use std::collections::HashMap;
use std::process::exit;
use clap::Parser;
use sea_orm::ColIdx;
use crate::error::Error;
use commands::*;

mod database;
pub mod error;
mod otp;
mod commands;

#[tokio::main]
async fn main() {
    let cli = MtotpCli::parse();
    database::init().await;
    match cli {
        MtotpCli::List(_) => print_codes().await,
        MtotpCli::Add(args) => register(args).await,
        MtotpCli::Remove(_) => remove().await,
        MtotpCli::Rename(_) => rename().await,
    }
}

async fn print_codes() {
    let registered_codes = database::registered::find_all().await;
    println!("┍ -------------------- ┯ ---------- ┑");
    println!("| {:20} | {:>10} |", "label", "code");
    println!("| -------------------- ┿ ---------- |");
    for x in registered_codes {
        let label = x.label;
        let code = match otp::totp(x.secret.as_str(), 30, 6) {
            Ok(data) => data,
            Err(err) => match err {
                Error::Hmac(_) => "HMac key error".to_owned(),
                Error::Message(msg) => msg.content,
                Error::Other(_) => "Exception".to_owned(),
            }
        };
        println!("| {:20} | {:>10} |", label, code);
    }
    println!("└ -------------------- ┴ ---------- ┘");
}

async fn register(args: AddArgs) {
    let url_or_key = if let Some(url_or_key) = args.url_or_key {
        url_or_key
    } else {
        dialoguer::Input::new()
            .with_prompt("Input totp url or setup key")
            .interact_text()
            .unwrap()
    };
    if url_or_key.is_empty() {
        println!("Please input url or key");
        exit(1);
    }
    if url_or_key.starts_with("otpauth://totp/") {
        let url = url::Url::parse(url_or_key.as_str()).expect("Invalid url");
        let path_split = url.path().split("/");
        let label = path_split.last().expect("").as_str().expect("").to_owned();
        let mut query_map: HashMap<String, String> = HashMap::new();
        for (key, value) in url.query_pairs() {
            query_map.insert(key.to_string(), value.to_string());
        }
        let secret = query_map.get::<String>(&("secret".to_string())).expect("secret not found").to_string();
        let issuer = match query_map.get::<String>(&("issuer".to_string())) {
            None => "".to_string(),
            Some(so) => so.to_string(),
        };
        save_register(label, secret, issuer).await;
        return;
    }
    let regex = regex::Regex::new("^[ABCDEFGHIJKLMNOPQRSTUVWXYZ234567]+$").expect("Wrong regexp");
    if regex.is_match(url_or_key.as_str()) {
        let label = dialoguer::Input::new()
            .with_prompt("Input totp label")
            .interact_text()
            .unwrap();
        let issuer = "".to_owned();
        save_register(label, url_or_key, issuer).await;
        return;
    }
    println!("Invalid key or url");
    exit(1);
}

async fn save_register(label: String, secret: String, issuer: String) {
    let code = match otp::totp(secret.as_str(), 30, 6) {
        Ok(code) => code,
        Err(err) => {
            println!("{:?}", err);
            exit(1);
        }
    };
    let registered = database::registered::Model {
        uuid: uuid::Uuid::new_v4().to_string(),
        label: label.clone(),
        secret,
        issuer,
    };
    database::registered::insert(registered).await;
    println!("Totp has been registered, label : {}, current code : {}", label, code);
}

async fn remove() {
    let registered_codes = database::registered::find_all().await;
    if registered_codes.is_empty() {
        println!("No registered codes");
        exit(1);
    }
    let selected = dialoguer::MultiSelect::new()
        .with_prompt("Choose totp to remove")
        .items(
            registered_codes.iter().map(|x| x.label.as_str()).collect::<Vec<&str>>().as_slice()
        ).interact().unwrap();
    if !selected.is_empty() {
        let selected_tps = registered_codes.iter()
            .enumerate()
            .filter(|(i, _)| selected.contains(i))
            .map(|(_, x)| x)
            .map(|x| (x.label.as_str(), x.uuid.as_str()))
            .collect::<Vec<(&str, &str)>>();
        let confirm = dialoguer::Confirm::new()
            .with_prompt(
                format!("Are you sure to remove? : {:?}", selected_tps.iter().map(|x| x.0).collect::<Vec<&str>>())
            ).interact().unwrap();
        if confirm {
            for (_, uuid) in selected_tps {
                database::registered::delete_by_uuid(uuid).await;
            }
            println!("Removed")
        }
    }
}

async fn rename() {
    let registered_codes = database::registered::find_all().await;
    if registered_codes.is_empty() {
        println!("No registered codes");
        exit(1);
    }
    let selected = dialoguer::Select::new()
        .with_prompt("Choose totp to rename")
        .items(
            registered_codes.iter().map(|x| x.label.as_str()).collect::<Vec<&str>>().as_slice()
        )
        .default(0)
        .interact_opt().unwrap();
    if let Some(selected) = selected {
        let selected = registered_codes.get(selected).unwrap();
        let rename_to: String = dialoguer::Input::new()
            .with_prompt(format!("Rename {} to", selected.label))
            .interact_text().unwrap();
        if rename_to.is_empty() || rename_to.trim().is_empty() {
            println!("Rename to can not be empty");
            exit(1);
        }
        database::registered::update_label_by_uuid(
            selected.uuid.as_str(),
            rename_to.as_str(),
        ).await;
        println!("Totp has been renamed");
    }
}

