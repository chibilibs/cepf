//use std::collections::HashMap;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Cep {
    cep: String,
    logradouro: String,
    complemento: String,
    bairro: String,
    localidade: String,
    uf: String,
    ibge: String,
    gia: String,
    ddd: String,
    siafi: String,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    cep: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let url = format!("https://viacep.com.br/ws/{}/json/", args.cep);
    let body = reqwest::get(url).await?.json::<Cep>().await?;

    let caganeira = format!(" cep : {} \n logradouro : {} \n complemento : {} \n bairro : {} \n localidade : {} \n uf : {} \n ddd : {}", body.cep, body.logradouro, body.complemento, body.bairro, body.localidade, body.uf, body.ddd);
    print!("{}", caganeira);
    Ok(())
}
