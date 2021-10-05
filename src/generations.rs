pub fn is_allowed_gg(gg: &str) -> bool {
    let allowed = vec![
        "x-y",
        "omega-ruby-alpha-sapphire",
        "sun-moon",
        "ultra-sun-ultra-moon",
        "black-2-white-2",
        "black-white",
        "heartgold-soulsilver",
        "platinum",
        "diamond-pearl",
        "emerald",
        "ruby-sapphire",
        "xd",
        "colosseum",
        "gold-silver",
        "crystal",
        "red-blue",
        "yellow",
        "lets-go",
        "sword-shield",
        "firered-leafgreen",
    ];

    allowed.contains(&gg)
}
