use rocket::serde::{json::{Json, Value}};
use rocket::serde::json::json;
use rocket::fs::{FileServer, relative};
use rocket::http::Method;
use rocket::post;
use rocket_cors::{AllowedOrigins, AllowedHeaders, CorsOptions};
use std::fs::File;
use std::io::{Write};
use std::process::Command;
use reqwest;
use dotenv::dotenv;


#[macro_use] extern crate rocket;

#[derive(serde::Deserialize)]
struct ChatRequest {
    messages: Vec<Message>,
}

#[derive(serde::Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(serde::Deserialize)]
struct CompileRequest {
    source: String,
}

#[get("/")]
fn index() -> &'static str {
    "Ink GPT plugin built by github.com/markeljan"
}
#[post("/api/chat", format = "json", data = "<chat_request>")]
async fn chat(chat_request: Json<ChatRequest>) -> Result<Json<Value>, &'static str> {
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let client = reqwest::Client::new();

    let mut messages = Vec::new();

    // Add system message
    messages.push(json!({
        "role": "system",
        "content": "You are an AI assistant that generates Ink smart contracts which are written in Rust.  Keep the contracts simple so they will compile."
    }));

    // Add user and assistant messages
    for message in &chat_request.messages {
        messages.push(json!({
            "role": &message.role,
            "content": &message.content,
        }));
    }

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "ft:gpt-3.5-turbo-0613:w3gpt::8Fl7XO8U",
            "messages": messages,
        }))
        .send()
        .await
        .map_err(|_| "Failed to send request")?;

    let json = response.json::<Value>()
        .await
        .map_err(|_| "Failed to parse response as JSON")?;

    Ok(Json(json))
}

#[post("/api/compile", format = "json", data = "<source_code>")]
fn compile(source_code: Json<CompileRequest>) -> String {
    // Path to the contract file
    let file_path = "../deployer/lib.rs";

    // Write the new contract code to the file
    match File::create(file_path) {
        Err(why) => {
            return format!("couldn't create {}: {}", file_path, why);
        }
        Ok(mut file) => {
            if let Err(why) = file.write_all(source_code.source.as_bytes()) {
                return format!("couldn't write to {}: {}", file_path, why);
            }
        }
    }

    let output = Command::new("cargo")
        .current_dir("../deployer")
        .arg("+nightly")
        .arg("contract")
        .arg("build")
        .arg("--release")
        .output();

    match output {
        Ok(output) => {
            if !output.stderr.is_empty() {
                return String::from_utf8_lossy(&output.stderr).to_string();
            }
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(e) => {
            format!("Failed to execute command: {}", e)
        }
    }
}

#[get("/api/deploy")]
fn deploy() -> String {
    let output = Command::new("mxpy")
        .current_dir("../deployer/contract")
        .arg("contract")
        .arg("deploy")
        .arg("--bytecode")
        .arg("./output/contract.wasm")
        .arg("--pem")
        .arg("../../wallet/wallet-owner.pem")
        .arg("--recall-nonce")
        .arg("--gas-limit")
        .arg("60000000")
        .arg("--chain")
        .arg("D")
        .arg("--outfile")
        .arg("deploy-devnet.interaction.json")
        .arg("--send")
        .output();

    match output {
        Ok(output) => {
            if !output.stderr.is_empty() {
                return String::from_utf8_lossy(&output.stderr).to_string();
            }
            String::from_utf8_lossy(&output.stdout).to_string()
        }
        Err(e) => {
            format!("Failed to execute command: {}", e)
        }
    }
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8000",
        "https://chat.openai.com",
        "http://localhost:8081",
    ]);
    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(
            vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        )
        .allowed_headers(AllowedHeaders::all())
        .allow_credentials(true)
        .to_cors()
        .unwrap();

    rocket::build()
        .attach(cors)
        .mount("/", routes![index])
        .mount("/", routes![compile])
        .mount("/", routes![deploy])
        .mount("/", routes![chat])
        .mount("/.well-known", FileServer::from(relative!("static/.well-known")).rank(1))
        .mount("/", FileServer::from(relative!("static")).rank(2))
}