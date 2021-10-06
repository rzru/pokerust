mod ability;
mod generations;
mod held_item;
mod http;
mod named_api_resource;
mod pk_move;
mod pk_type;
mod pokemon;
mod sprites;
mod version_game_index;

#[cfg(test)]
#[macro_use]
extern crate yup_hyper_mock as hyper_mock;

use crate::http::Http;
use crate::pokemon::Pokemon;

use crate::generations::is_allowed_gg;
use clap::{load_yaml, App};

static BASE_URL: &str = "https://pokeapi.co/api/v2";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let result = run().await;
    if let Err(e) = result {
        eprintln!("{}", e);
    }

    Ok(())
}

async fn run<'a>() -> Result<(), &'a str> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from(yaml).get_matches();
    let client = Http::new();

    if let Some(pokemon_name) = matches.value_of("pokemon") {
        let pokemon = Pokemon::new(&client, pokemon_name)
            .await
            .ok_or("Pokemon not found")?;
        let should_render_moves = matches.is_present("moves");
        let gg = matches.value_of("games-gen");
        if should_render_moves {
            if gg.is_none() {
                return Err("You should use --moves flag only with --games-gen option specified");
            }
            if gg.is_some() && !is_allowed_gg(gg.unwrap()) {
                return Err(
                    "Not allowed --games-gen option, for list of allowed options use --help",
                );
            }
        }

        pokemon.render(should_render_moves, gg).await.unwrap();
    }

    Ok(())
}
