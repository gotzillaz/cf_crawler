// use std::env;
// use dotenvy::dotenv;
// use std::collections::HashMap;
use std::error::Error;

struct Config {
    CF_API_KEY: String,
    CF_API_SECRET: String
}

fn read_config() -> Result<Config, Box<dyn Error>> {
    dotenvy::dotenv()?;
    let key: String = dotenvy::var("CF_API_KEY")?;
    let secret: String = dotenvy::var("CF_API_SECRET")?;
    let env: Config = Config{ CF_API_KEY: key, CF_API_SECRET: secret };
    return Ok(env)
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let env = read_config()?;
    println!("{} {}", env.CF_API_KEY, env.CF_API_SECRET);
    Ok(())
}