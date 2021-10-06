use std::collections::HashMap;

pub fn game_entry_by_game(game: &str) -> Option<String> {
    let game_entry_xy = "x-y";
    let game_entry_oras = "omega-ruby-alpha-sapphire";
    let game_entry_sm = "sun-moon";
    let game_entry_usum = "ultra-sun-ultra-moon";
    let game_entry_b2w2 = "black-2-white-2";
    let game_entry_bw = "black-white";
    let game_entry_hgss = "heartgold-soulsilver";
    let game_entry_pl = "platinum";
    let game_entry_dp = "diamond-pearl";
    let game_entry_em = "emerald";
    let game_entry_rs = "ruby-sapphire";
    let game_entry_xd = "xd";
    let game_entry_colos = "colosseum";
    let game_entry_gs = "gold-silver";
    let game_entry_crys = "crystal";
    let game_entry_rb = "red-blue";
    let game_entry_yel = "yellow";
    let game_entry_go = "lets-go";
    let game_entry_ss = "sword-shield";
    let game_entry_frlf = "firered-leafgreen";

    let mut games_to_generations = HashMap::new();

    games_to_generations.insert("x", game_entry_xy);
    games_to_generations.insert("y", game_entry_xy);
    games_to_generations.insert("omega-ruby", game_entry_oras);
    games_to_generations.insert("omega ruby", game_entry_oras);
    games_to_generations.insert("alpha sapphire", game_entry_oras);
    games_to_generations.insert("alpha-sapphire", game_entry_oras);
    games_to_generations.insert("sun", game_entry_sm);
    games_to_generations.insert("moon", game_entry_sm);
    games_to_generations.insert("ultra-sun", game_entry_usum);
    games_to_generations.insert("ultra sun", game_entry_usum);
    games_to_generations.insert("ultra-moon", game_entry_usum);
    games_to_generations.insert("black 2", game_entry_b2w2);
    games_to_generations.insert("black-2", game_entry_b2w2);
    games_to_generations.insert("white-2", game_entry_b2w2);
    games_to_generations.insert("white 2", game_entry_b2w2);
    games_to_generations.insert("black", game_entry_bw);
    games_to_generations.insert("white", game_entry_bw);
    games_to_generations.insert("heartgold", game_entry_hgss);
    games_to_generations.insert("heart-gold", game_entry_hgss);
    games_to_generations.insert("heart gold", game_entry_hgss);
    games_to_generations.insert("soulsilver", game_entry_hgss);
    games_to_generations.insert("soul silver", game_entry_hgss);
    games_to_generations.insert("soul-silver", game_entry_hgss);
    games_to_generations.insert("platinum", game_entry_pl);
    games_to_generations.insert("diamond", game_entry_dp);
    games_to_generations.insert("pearl", game_entry_dp);
    games_to_generations.insert("emerald", game_entry_em);
    games_to_generations.insert("ruby", game_entry_rs);
    games_to_generations.insert("sapphire", game_entry_rs);
    games_to_generations.insert("xd", game_entry_xd);
    games_to_generations.insert("colosseum", game_entry_colos);
    games_to_generations.insert("gold", game_entry_gs);
    games_to_generations.insert("silver", game_entry_gs);
    games_to_generations.insert("crystal", game_entry_crys);
    games_to_generations.insert("red", game_entry_rb);
    games_to_generations.insert("blue", game_entry_rb);
    games_to_generations.insert("yellow", game_entry_yel);
    games_to_generations.insert("lets-go", game_entry_go);
    games_to_generations.insert("lets go", game_entry_go);
    games_to_generations.insert("go", game_entry_go);
    games_to_generations.insert("sword", game_entry_ss);
    games_to_generations.insert("shield", game_entry_ss);
    games_to_generations.insert("firered", game_entry_frlf);
    games_to_generations.insert("fire red", game_entry_frlf);
    games_to_generations.insert("fire-red", game_entry_frlf);
    games_to_generations.insert("leaf-green", game_entry_frlf);
    games_to_generations.insert("leaf green", game_entry_frlf);
    games_to_generations.insert("leafgreen", game_entry_frlf);
    if let Some(value) = games_to_generations.get(game) {
        return Some(value.to_string());
    }

    None
}
