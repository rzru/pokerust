# PokéRust - Terminal PokéDex built in Rust

## Run in dev mode
``cargo run -- [OPTIONS]``
## Build
``cargo build --release``
## Usage info
```shell
USAGE:
    pokerust [FLAGS] [OPTIONS]

FLAGS:
    -a, --abilities    Show extende info about Pokemon abilities
    -h, --help         Print help information
    -m, --moves        Show Pokemon moves info (works only with games-gen specified)
    -V, --version      Print version information

OPTIONS:
    -g, --game <game>          Specify concrete game from which you need information. Available
                               values: x, y, omega-ruby, alpha-sapphire, sun, moon, ultra-sun,
                               ultra-moon, black-2, white-2, black, white, heartgold, soulsilver,
                               platinum, diamond, pearl, emerald, ruby, sapphire, xd, colosseum,
                               gold, silver, crystal, red, blue, yellow, lets-go, sword, shield,
                               firered, leafgreen
```
## Example usage
### Builded
```shell
pokerust -p bulbasaur # show bulbasaur info
pokerust -p bulbasaur --moves --game=x # show bulbasaur info with additional information about moves in concrete game
pokerust -p bulbasaur --abilities # show bulbasaur info with additional information about abilities
```