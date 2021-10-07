mod abilities;
mod api_resource;
mod fetch_external;
mod flavor_text_entry;
mod generations;
mod held_item;
mod http;
mod moves;
mod named_api_resource;
mod pk_type;
mod pokemon;
mod pokemon_species;
mod pokemon_stat;
mod sprites;
mod version_game_index;

pub use fetch_external::fetch_external;

use crate::http::Http;
use crate::pokemon::Pokemon;

use crate::generations::game_entry_by_game;
use clap::{load_yaml, App, ArgMatches};
use std::collections::HashMap;

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

    let mut render_details = HashMap::new();
    render_details.insert("moves", matches.is_present("moves"));
    render_details.insert("abilities", matches.is_present("abilities"));
    render_details.insert("ext", matches.is_present("ext"));

    let mut gg = None;
    let game = matches.value_of("game");

    if let Some(game) = game {
        gg = game_entry_by_game(game)
    }

    if *render_details.get("moves").unwrap() {
        if game.is_none() {
            return Err("You should use --moves flag only with --game option specified");
        }
        if gg.is_none() {
            return Err("Not allowed --game option, for list of allowed options use --help");
        }
    }

    if let Ok(_) = pokemon
        .render(render_details, gg.unwrap_or(String::new()))
        .await
    {
        return Ok(());
    }

    Err("Error rendering pokedex entry")
}
