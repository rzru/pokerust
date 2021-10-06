mod ability;
mod generations;
mod held_item;
mod http;
mod moves;
mod named_api_resource;
mod pk_type;
mod pokemon;
mod sprites;
mod version_game_index;

#[cfg(test)]
#[macro_use]
extern crate yup_hyper_mock as hyper_mock;

use crate::http::Http;
use crate::pokemon::Pokemon;

use crate::generations::game_entry_by_game;
use clap::{load_yaml, App, ArgMatches};

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
        return try_process_pokemon(pokemon_name, &matches, &client).await;
    }

    Ok(())
}

async fn try_process_pokemon<'a>(
    pokemon_name: &str,
    matches: &ArgMatches,
    client: &Http,
) -> Result<(), &'a str> {
    let pokemon = Pokemon::new(client, pokemon_name)
        .await
        .ok_or("Pokemon not found")?;
    let should_render_moves = matches.is_present("moves");
    let game = matches.value_of("game");
    let gg = game_entry_by_game(game.unwrap());

    if should_render_moves {
        if game.is_none() {
            return Err("You should use --moves flag only with --game option specified");
        }
        if gg.is_none() {
            return Err("Not allowed --game option, for list of allowed options use --help");
        }
    }

    if let Ok(_) = pokemon.render(should_render_moves, gg).await {
        return Ok(());
    }

    Err("Error rendering pokedex entry")
}
