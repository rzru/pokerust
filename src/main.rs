mod pokemon;
mod http;

#[cfg(test)] #[macro_use]
extern crate yup_hyper_mock as hyper_mock;

use crate::pokemon::Pokemon;
use crate::http::Http;

use clap::{App, load_yaml};

static BASE_URL: &str = "https://pokeapi.co/api/v2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = run().await;
    if let Err(e) = result {
        eprintln!("Couldn't parse your request! Try again. Error: {}", e);
    }

    Ok(())
}

async fn run<'a>() -> Result<(), &'a str> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from(yaml).get_matches();
    let client = Http::new();

    if let Some(pokemon_name) = matches.value_of("pokemon") {
        let pokemon = Pokemon::new(&client, pokemon_name).await.ok_or("pokemon not found")?;
        let should_render_moves = matches.is_present("moves");

        pokemon.render(should_render_moves).unwrap();
    }

    Ok(())
}