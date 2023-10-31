// use std::env;
// use dotenvy::dotenv;
// use std::collections::HashMap;
use std::error::Error;
use std::fs;
use serde_json;

struct Config {
    cf_api_key: String,
    cf_api_secret: String,
    submission_json_path: String
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let key: String = dotenvy::var("CF_API_KEY")?;
    let secret: String = dotenvy::var("CF_API_SECRET")?;
    let json_path: String = dotenvy::var("SUBMISSION_JSON_PATH")?;
    let env: Config = Config{ cf_api_key: key, cf_api_secret: secret, submission_json_path: json_path };
    return Ok(env)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let env: Config = read_config()?;
    println!("{} {}", env.cf_api_key, env.cf_api_secret);

    let data: String = fs::read_to_string(env.submission_json_path)?;
    let json_obj: serde_json::Value = serde_json::from_str(&data)?;

    for i in 0..json_obj["result"].as_array().unwrap().len() {
        if json_obj["result"][i]["verdict"].as_str().unwrap().eq("OK"){
            println!("{:?} {:?}", i, json_obj["result"][i]["id"]);
            let contest_id: u64 = json_obj["result"][i]["contestId"].as_u64().unwrap();
            let submission_id: u64 = json_obj["result"][i]["id"].as_u64().unwrap();
            println!("{:?} {:?}", contest_id, submission_id); 
        }
    }
    println!("JSON: {} {}", json_obj["result"][0], json_obj["result"].as_array().unwrap().len());
    Ok(())
}