# PokéRust - Terminal PokéDex built in Rust
## Run in dev mode
``cargo run --features color -- [OPTIONS] [FLAGS]``
## Build
``cargo build --features color --release``
## Usage info
```shell
USAGE:
    pokerust [FLAGS] [OPTIONS]

FLAGS:
    -a, --abilities    Show extended info about pokemon abilities
    -h, --help         Print help information
    -m, --moves        Show Pokemon moves info (works only with --game specified)
    -V, --version      Print version information

OPTIONS:
    -g, --game <game>          Specify concrete game from which you need information. Available
                               values: x, y, omega-ruby, alpha-sapphire, sun, moon, ultra-sun,
                               ultra-moon, black-2, white-2, black, white, heartgold, soulsilver,
                               platinum, diamond, pearl, emerald, ruby, sapphire, xd, colosseum,
                               gold, silver, crystal, red, blue, yellow, lets-go, sword, shield,
                               firered, leafgreen
    -p, --pokemon <pokemon>    Specify pokemon name or id
```
## Example usage
### Builded
```shell
pokerust -p bulbasaur # show bulbasaur info
pokerust -p bulbasaur --moves --game=x # show bulbasaur info with additional information about moves in concrete game
pokerust -p bulbasaur --abilities # show bulbasaur info with additional information about abilities
```
