extern crate core;

use clap::Parser;
use commands::*;
use mtotp_lib::{save_register, save_url};
use std::process::exit;

mod commands;

#[tokio::main]
async fn main() {
    let cli = MtotpCli::parse();
    mtotp_lib::init().await;
    match cli {
        MtotpCli::List(_) => print_codes().await,
        MtotpCli::Add(args) => register(args).await,
        MtotpCli::Remove(_) => remove().await,
        MtotpCli::Rename(_) => rename().await,
    }
}

async fn print_codes() {
    let registered_codes = mtotp_lib::print_codes().await;
    println!("┍ -------------------- ┯ ---------- ┑");
    println!("| {:20} | {:>10} |", "label", "code");
    println!("| -------------------- ┿ ---------- |");
    for x in registered_codes {
        println!("| {:20} | {:>10} |", x.label, x.code);
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
        let saved = save_url(url_or_key).await.expect("Failed to save url");
        println!(
            "Totp has been registered, label : {}, current code : {}",
            saved.label, saved.code
        );
        return;
    }
    let regex = regex::Regex::new("^[ABCDEFGHIJKLMNOPQRSTUVWXYZ234567]+$").expect("Wrong regexp");
    if regex.is_match(url_or_key.as_str()) {
        let label = dialoguer::Input::new()
            .with_prompt("Input totp label")
            .interact_text()
            .unwrap();
        let issuer = "".to_owned();
        let saved = save_register(label, url_or_key, issuer)
            .await
            .expect("Failed to save key");
        println!(
            "Totp has been registered, label : {}, current code : {}",
            saved.label, saved.code
        );
        return;
    }
    println!("Invalid key or url");
    exit(1);
}

async fn remove() {
    let registered_codes = mtotp_lib::print_codes().await;
    if registered_codes.is_empty() {
        println!("No registered codes");
        exit(1);
    }
    let selected = dialoguer::MultiSelect::new()
        .with_prompt("Choose totp to remove")
        .items(
            registered_codes
                .iter()
                .map(|x| x.label.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .interact()
        .unwrap();
    if !selected.is_empty() {
        let selected_tps = registered_codes
            .iter()
            .enumerate()
            .filter(|(i, _)| selected.contains(i))
            .map(|(_, x)| x)
            .map(|x| (x.label.as_str(), x.uuid.as_str()))
            .collect::<Vec<(&str, &str)>>();
        let confirm = dialoguer::Confirm::new()
            .with_prompt(format!(
                "Are you sure to remove? : {:?}",
                selected_tps.iter().map(|x| x.0).collect::<Vec<&str>>()
            ))
            .interact()
            .unwrap();
        if confirm {
            for (_, uuid) in selected_tps {
                mtotp_lib::remove(uuid).await;
            }
            println!("Removed")
        }
    }
}

async fn rename() {
    let registered_codes = mtotp_lib::print_codes().await;
    if registered_codes.is_empty() {
        println!("No registered codes");
        exit(1);
    }
    let selected = dialoguer::Select::new()
        .with_prompt("Choose totp to rename")
        .items(
            registered_codes
                .iter()
                .map(|x| x.label.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .default(0)
        .interact_opt()
        .unwrap();
    if let Some(selected) = selected {
        let selected = registered_codes.get(selected).unwrap();
        let rename_to: String = dialoguer::Input::new()
            .with_prompt(format!("Rename {} to", selected.label))
            .interact_text()
            .unwrap();
        if rename_to.is_empty() || rename_to.trim().is_empty() {
            println!("Rename to can not be empty");
            exit(1);
        }
        mtotp_lib::rename(selected.uuid.as_str(), rename_to.as_str()).await;
        println!("Totp has been renamed");
    }
}
