use reqwest;
use std::fs;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let num_termos = &args[1]; // O primeiro argumento da linha de comando
    let response = reqwest::get(&format!("https://random-word-api.herokuapp.com/word?number={}", num_termos)).await?;
    let words: Vec<String> = response.json().await?;
    let nome_projeto = words.join("_");
    fs::create_dir_all(&nome_projeto)?;
    println!("Pasta '{}' criada com sucesso!", nome_projeto);
    Ok(())
}
