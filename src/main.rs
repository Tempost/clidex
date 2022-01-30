use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Pokemon {
    // fill me with a TON of Pokemon info....
    abilities:  Vec<Abilities>,
    base_exp:   u32,
    // forms
    height:     u32,
    held_items: Vec<String>,
    id:         u32,
    // moves
    name: String,
    // species
    // sprites
    // stats
    // types
    weight:     u32
}

#[derive(Serialize, Deserialize)]
struct Abilities {
    ability:   AbilityInfo,
    is_hidden: bool,
    slot:      u8
}

#[derive(Serialize, Deserialize)]
struct AbilityInfo {
    name: String,
    url:  String
}

#[derive(Serialize, Deserialize)]
struct Move {
    name: String,
    url:  String
}

#[derive(Serialize, Deserialize)]
struct Sprite {
    back_default:       Option<String>,
    back_female:        Option<String>,
    back_shiny:         Option<String>,
    back_shiny_female:  Option<String>,
    front_default:      Option<String>,
    front_female:       Option<String>,
    front_shiny:        Option<String>,
    front_shiny_female: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Species {
    name: String,
    url:  String
}

#[derive(Serialize, Deserialize)]
struct Stats {
    base_stat: u32,
    effort:    u32,
    stat:      StatInfo 
} 

#[derive(Serialize, Deserialize)]
struct StatInfo {
    name: String,
    url:  String
}


/* 
*
* TODO:
* Take user input -> pokemonname "--info" "--sprite" -- "only a pokemon name will show data"
* 
* Send request to https://pokeapi.co/api/v2/pokemon/"name"
* 
* Derialize json response, sends back ALOT of data so maybe only include a select few and slowly
* update it. No need to bother with include strong type saftey
*
* Use the Deserialized data to display pokemons info/sprite/locations to stdout
*
* NOTE: try and include proper error handling, show informative messages in case something goes
* wrong 
*
* NOTE: Use w3m to display the image to the terminal
*/

fn main() {
}
