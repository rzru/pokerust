## PokéRust - Terminal PokéDex built in Rust

### Run in dev mode
``cargo run -- [OPTIONS]``

### Build
``cargo build --release``

### Usage info
```shell
USAGE:
    pokerust [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -m, --moves      Show Pokemon moves info
    -V, --version    Print version information

OPTIONS:
    -p, --pokemon <pokemon>    Specify pokemon name or id
```

### Example usage

#### Builded
```shell
pokerust -p bulbasaur # show bulbasaur info
pokerust -p bulbasaur -m # show bulbasaur info with additional information about moves
```