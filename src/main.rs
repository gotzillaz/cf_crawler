use rand::Rng;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use scraper::{Html, Selector};
use serde_json;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time;

struct Config {
    submission_json_path: String,
    html_path: String,
    output_path: String,
    lang_ext_path: String,
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let json_path: String = dotenvy::var("SUBMISSION_JSON_PATH")?;
    let html_path: String = dotenvy::var("HTML_PATH")?;
    let output_path: String = dotenvy::var("OUTPUT_PATH")?;
    let lang_ext_path: String = dotenvy::var("LANG_EXT_PATH")?;
    let env: Config = Config {
        submission_json_path: json_path,
        html_path: html_path,
        output_path: output_path,
        lang_ext_path: lang_ext_path,
    };
    return Ok(env);
}

fn main() -> Result<(), Box<dyn Error>> {
    let env: Config = read_config()?;

    if !Path::new(&env.html_path).exists() {
        fs::create_dir_all(&env.html_path)?;
    }

    let data: String = fs::read_to_string(env.submission_json_path)?;
    let json_obj: serde_json::Value = serde_json::from_str(&data)?;
    let client: Client = reqwest::blocking::Client::new();

    let lang_ext_data: String = fs::read_to_string(env.lang_ext_path)?;
    let lang_ext_map: serde_json::Value = serde_json::from_str(&lang_ext_data)?;

    let start_time: time::Instant = time::Instant::now();
    let mut ac_index: Vec<usize> = vec![];
    let mut retry_count: u32 = 0;

    let mut submission_count: u32 = 0;
    for i in 0..json_obj["result"].as_array().unwrap().len() {
        if json_obj["result"][i]["verdict"].as_str().unwrap().eq("OK") {
            ac_index.push(i);
            submission_count += 1;
        }
    }

    println!("Total Accepted submssion: {}", submission_count);
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for (pos, i) in ac_index.iter().enumerate() {
        if json_obj["result"][i]["verdict"].as_str().unwrap().eq("OK") {
            let contest_id: u64 = json_obj["result"][i]["contestId"].as_u64().unwrap();
            let submission_id: u64 = json_obj["result"][i]["id"].as_u64().unwrap();
            let html_file_path: PathBuf =
                Path::new(&env.html_path).join(format!("{submission_id}.html"));

            if Path::new(&html_file_path).exists() {
                continue;
            }

            println!(
                "File #{}: {} from contest {}.",
                pos, submission_id, contest_id
            );

            let result: Response = loop {
                let res: Response = client
                    .get(format!(
                        "https://codeforces.com/contest/{contest_id}/submission/{submission_id}"
                    ))
                    .send()?;
                if res.status().is_success() {
                    break res;
                } else {
                    retry_count += 1;
                    if retry_count > 3 {
                        panic!("Failed to continue (Attempt = {})", retry_count);
                    }
                    println!(
                        "Got {} at {}, waiting the process for 15 minutes (Attempt #{}).",
                        res.status(),
                        submission_id,
                        retry_count
                    );
                    thread::sleep(time::Duration::from_secs(60 * 15));
                }
            };
            fs::write(html_file_path, result.text()?)?;

            thread::sleep(time::Duration::from_secs(rng.gen_range(5..15)));
            println!("Total time (File #{}): {:?}", pos, start_time.elapsed());
            retry_count = 0;
        }
    }

    if !Path::new(&env.output_path).exists() {
        fs::create_dir_all(&env.output_path)?;
    }

    for (_, i) in ac_index.iter().enumerate() {
        if json_obj["result"][i]["verdict"].as_str().unwrap().eq("OK") {
            let contest_id: u64 = json_obj["result"][i]["contestId"].as_u64().unwrap();
            let index_id: &str = json_obj["result"][i]["problem"]["index"].as_str().unwrap();
            let submission_id: u64 = json_obj["result"][i]["id"].as_u64().unwrap();
            let language_name: &str = json_obj["result"][i]["programmingLanguage"]
                .as_str()
                .unwrap();
            let ext_name: &str = lang_ext_map[language_name].as_str().unwrap();

            let html_file_path: PathBuf =
                Path::new(&env.html_path).join(format!("{submission_id}.html"));
            let submission_file_path: PathBuf = Path::new(&env.output_path).join(format!(
                "{contest_id}/{contest_id}{index_id}-{submission_id}{ext_name}"
            ));

            if !Path::new(&env.output_path)
                .join(format!("{contest_id}"))
                .exists()
            {
                fs::create_dir_all(Path::new(&env.output_path).join(format!("{contest_id}")))?;
            }

            let submission_html: String = fs::read_to_string(html_file_path)?;
            let html_obj: Html = Html::parse_fragment(&submission_html);
            let source_selector: Selector = Selector::parse(&"#program-source-text")?;
            let source_obj: scraper::html::Select<'_, '_> = html_obj.select(&source_selector);

            for code in source_obj {
                let source_code: String = code.text().collect::<Vec<_>>().join(" ");
                fs::write(submission_file_path, source_code)?;
                break;
            }
        }
    }

    Ok(())
}
