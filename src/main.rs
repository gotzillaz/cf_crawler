use std::fs;
use std::error::Error;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time;
use reqwest::blocking::Response;
use serde_json;

struct Config {
    cf_api_key: String,
    cf_api_secret: String,
    submission_json_path: String,
    html_path: String
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let key: String = dotenvy::var("CF_API_KEY")?;
    let secret: String = dotenvy::var("CF_API_SECRET")?;
    let json_path: String = dotenvy::var("SUBMISSION_JSON_PATH")?;
    let html_path: String = dotenvy::var("HTML_PATH")?;
    let env: Config = Config{ cf_api_key: key, cf_api_secret: secret, submission_json_path: json_path, html_path: html_path};
    return Ok(env)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let env: Config = read_config()?;
    println!("{} {}", env.cf_api_key, env.cf_api_secret);

    if !Path::new(&env.html_path).exists() {
        fs::create_dir_all(&env.html_path)?;
    }

    let data: String = fs::read_to_string(env.submission_json_path)?;
    let json_obj: serde_json::Value = serde_json::from_str(&data)?;
    let client = reqwest::blocking::Client::new();

    for i in 0..json_obj["result"].as_array().unwrap().len() {
        if json_obj["result"][i]["verdict"].as_str().unwrap().eq("OK"){
            let contest_id: u64 = json_obj["result"][i]["contestId"].as_u64().unwrap();
            let submission_id: u64 = json_obj["result"][i]["id"].as_u64().unwrap();
            let html_file_path: PathBuf = Path::new(&env.html_path).join(format!("{submission_id}.html"));

            println!("File #{}: {} from {} contest", i, submission_id, contest_id);
            let result: Response = client.get(format!("https://codeforces.com/contest/{contest_id}/submission/{submission_id}"))
                .send()?;
            fs::write(html_file_path, result.text()?)?;

            thread::sleep(time::Duration::from_secs(2));
            break;
        }
    }
    Ok(())
}