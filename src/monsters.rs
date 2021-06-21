use crate::types::{PokemonType, TypeData};
use crate::{Pokemon, Stats};

pub const ABOMASNOW: Pokemon = Pokemon {
    name: "Abomasnow",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 90,
        attack: 92,
        defense: 75,
        special_attack: 92,
        special_defense: 85,
        speed: 60,
    },
};
pub const ABRA: Pokemon = Pokemon {
    name: "Abra",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 25,
        attack: 20,
        defense: 15,
        special_attack: 105,
        special_defense: 55,
        speed: 90,
    },
};
pub const ABSOL: Pokemon = Pokemon {
    name: "Absol",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 130,
        defense: 60,
        special_attack: 75,
        special_defense: 60,
        speed: 75,
    },
};
pub const ACCELGOR: Pokemon = Pokemon {
    name: "Accelgor",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 70,
        defense: 40,
        special_attack: 100,
        special_defense: 60,
        speed: 145,
    },
};
pub const AEGISLASH: Pokemon = Pokemon {
    name: "Aegislash",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 140,
        special_attack: 50,
        special_defense: 140,
        speed: 60,
    },
};
pub const AERODACTYL: Pokemon = Pokemon {
    name: "Aerodactyl",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 80,
        attack: 105,
        defense: 65,
        special_attack: 60,
        special_defense: 75,
        speed: 130,
    },
};
pub const AGGRON: Pokemon = Pokemon {
    name: "Aggron",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 70,
        attack: 110,
        defense: 180,
        special_attack: 60,
        special_defense: 60,
        speed: 50,
    },
};
pub const AIPOM: Pokemon = Pokemon {
    name: "Aipom",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 70,
        defense: 55,
        special_attack: 40,
        special_defense: 55,
        speed: 85,
    },
};
pub const ALAKAZAM: Pokemon = Pokemon {
    name: "Alakazam",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 50,
        defense: 45,
        special_attack: 135,
        special_defense: 95,
        speed: 120,
    },
};
pub const ALCREMIE: Pokemon = Pokemon {
    name: "Alcremie",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 60,
        defense: 75,
        special_attack: 110,
        special_defense: 121,
        speed: 64,
    },
};
pub const ALOMOMOLA: Pokemon = Pokemon {
    name: "Alomomola",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 165,
        attack: 75,
        defense: 80,
        special_attack: 40,
        special_defense: 45,
        speed: 65,
    },
};
pub const ALTARIA: Pokemon = Pokemon {
    name: "Altaria",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 75,
        attack: 70,
        defense: 90,
        special_attack: 70,
        special_defense: 105,
        speed: 80,
    },
};
pub const AMAURA: Pokemon = Pokemon {
    name: "Amaura",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 77,
        attack: 59,
        defense: 50,
        special_attack: 67,
        special_defense: 63,
        speed: 46,
    },
};
pub const AMBIPOM: Pokemon = Pokemon {
    name: "Ambipom",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 100,
        defense: 66,
        special_attack: 60,
        special_defense: 66,
        speed: 115,
    },
};
pub const AMOONGUSS: Pokemon = Pokemon {
    name: "Amoonguss",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 114,
        attack: 85,
        defense: 70,
        special_attack: 85,
        special_defense: 80,
        speed: 30,
    },
};
pub const AMPHAROS: Pokemon = Pokemon {
    name: "Ampharos",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 75,
        defense: 85,
        special_attack: 115,
        special_defense: 90,
        speed: 55,
    },
};
pub const ANORITH: Pokemon = Pokemon {
    name: "Anorith",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 45,
        attack: 95,
        defense: 50,
        special_attack: 40,
        special_defense: 50,
        speed: 75,
    },
};
pub const APPLETUN: Pokemon = Pokemon {
    name: "Appletun",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 110,
        attack: 85,
        defense: 80,
        special_attack: 100,
        special_defense: 80,
        speed: 30,
    },
};
pub const APPLIN: Pokemon = Pokemon {
    name: "Applin",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 40,
        attack: 40,
        defense: 80,
        special_attack: 40,
        special_defense: 40,
        speed: 20,
    },
};
pub const ARAQUANID: Pokemon = Pokemon {
    name: "Araquanid",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 68,
        attack: 70,
        defense: 92,
        special_attack: 50,
        special_defense: 132,
        speed: 42,
    },
};
pub const ARBOK: Pokemon = Pokemon {
    name: "Arbok",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 95,
        defense: 69,
        special_attack: 65,
        special_defense: 79,
        speed: 80,
    },
};
pub const ARCANINE: Pokemon = Pokemon {
    name: "Arcanine",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 110,
        defense: 80,
        special_attack: 100,
        special_defense: 80,
        speed: 95,
    },
};
pub const ARCEUS: Pokemon = Pokemon {
    name: "Arceus",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 120,
        attack: 120,
        defense: 120,
        special_attack: 120,
        special_defense: 120,
        speed: 120,
    },
};
pub const ARCHEN: Pokemon = Pokemon {
    name: "Archen",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 112,
        defense: 45,
        special_attack: 74,
        special_defense: 45,
        speed: 70,
    },
};
pub const ARCHEOPS: Pokemon = Pokemon {
    name: "Archeops",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 75,
        attack: 140,
        defense: 65,
        special_attack: 112,
        special_defense: 65,
        speed: 110,
    },
};
pub const ARCTOVISH: Pokemon = Pokemon {
    name: "Arctovish",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 90,
        attack: 90,
        defense: 100,
        special_attack: 80,
        special_defense: 90,
        speed: 55,
    },
};
pub const ARCTOZOLT: Pokemon = Pokemon {
    name: "Arctozolt",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 90,
        attack: 100,
        defense: 90,
        special_attack: 90,
        special_defense: 80,
        speed: 55,
    },
};
pub const ARIADOS: Pokemon = Pokemon {
    name: "Ariados",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 70,
        attack: 90,
        defense: 70,
        special_attack: 60,
        special_defense: 70,
        speed: 40,
    },
};
pub const ARMALDO: Pokemon = Pokemon {
    name: "Armaldo",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 75,
        attack: 125,
        defense: 100,
        special_attack: 70,
        special_defense: 80,
        speed: 45,
    },
};
pub const AROMATISSE: Pokemon = Pokemon {
    name: "Aromatisse",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 101,
        attack: 72,
        defense: 72,
        special_attack: 99,
        special_defense: 89,
        speed: 29,
    },
};
pub const ARON: Pokemon = Pokemon {
    name: "Aron",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 50,
        attack: 70,
        defense: 100,
        special_attack: 40,
        special_defense: 40,
        speed: 30,
    },
};
pub const ARROKUDA: Pokemon = Pokemon {
    name: "Arrokuda",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 41,
        attack: 63,
        defense: 40,
        special_attack: 40,
        special_defense: 30,
        speed: 66,
    },
};
pub const ARTICUNO: Pokemon = Pokemon {
    name: "Articuno",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 90,
        attack: 85,
        defense: 100,
        special_attack: 95,
        special_defense: 125,
        speed: 85,
    },
};
pub const AUDINO: Pokemon = Pokemon {
    name: "Audino",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 103,
        attack: 60,
        defense: 86,
        special_attack: 60,
        special_defense: 86,
        speed: 50,
    },
};
pub const AURORUS: Pokemon = Pokemon {
    name: "Aurorus",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 123,
        attack: 77,
        defense: 72,
        special_attack: 99,
        special_defense: 92,
        speed: 58,
    },
};
pub const AVALUGG: Pokemon = Pokemon {
    name: "Avalugg",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 117,
        defense: 184,
        special_attack: 44,
        special_defense: 46,
        speed: 28,
    },
};
pub const AXEW: Pokemon = Pokemon {
    name: "Axew",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 46,
        attack: 87,
        defense: 60,
        special_attack: 30,
        special_defense: 40,
        speed: 57,
    },
};
pub const AZELF: Pokemon = Pokemon {
    name: "Azelf",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 125,
        defense: 70,
        special_attack: 125,
        special_defense: 70,
        speed: 115,
    },
};
pub const AZUMARILL: Pokemon = Pokemon {
    name: "Azumarill",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 100,
        attack: 50,
        defense: 80,
        special_attack: 60,
        special_defense: 80,
        speed: 50,
    },
};
pub const AZUMARILL_GEN2: Pokemon = Pokemon {
    name: "Azumarill",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 50,
        defense: 80,
        special_attack: 60,
        special_defense: 80,
        speed: 50,
    },
};
pub const AZURILL: Pokemon = Pokemon {
    name: "Azurill",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 50,
        attack: 20,
        defense: 40,
        special_attack: 20,
        special_defense: 40,
        speed: 20,
    },
};
pub const BAGON: Pokemon = Pokemon {
    name: "Bagon",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 75,
        defense: 60,
        special_attack: 40,
        special_defense: 30,
        speed: 50,
    },
};
pub const BALTOY: Pokemon = Pokemon {
    name: "Baltoy",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 40,
        attack: 40,
        defense: 55,
        special_attack: 40,
        special_defense: 70,
        speed: 55,
    },
};
pub const BANETTE: Pokemon = Pokemon {
    name: "Banette",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 64,
        attack: 115,
        defense: 65,
        special_attack: 83,
        special_defense: 63,
        speed: 65,
    },
};
pub const BARBARACLE: Pokemon = Pokemon {
    name: "Barbaracle",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 72,
        attack: 105,
        defense: 115,
        special_attack: 54,
        special_defense: 86,
        speed: 68,
    },
};
pub const BARBOACH: Pokemon = Pokemon {
    name: "Barboach",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 50,
        attack: 48,
        defense: 43,
        special_attack: 46,
        special_defense: 41,
        speed: 60,
    },
};
pub const BARRASKEWDA: Pokemon = Pokemon {
    name: "Barraskewda",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 61,
        attack: 123,
        defense: 60,
        special_attack: 60,
        special_defense: 50,
        speed: 136,
    },
};
pub const BASCULIN: Pokemon = Pokemon {
    name: "Basculin",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 92,
        defense: 65,
        special_attack: 80,
        special_defense: 55,
        speed: 98,
    },
};
pub const BASTIODON: Pokemon = Pokemon {
    name: "Bastiodon",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 60,
        attack: 52,
        defense: 168,
        special_attack: 47,
        special_defense: 138,
        speed: 30,
    },
};
pub const BAYLEEF: Pokemon = Pokemon {
    name: "Bayleef",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 62,
        defense: 80,
        special_attack: 63,
        special_defense: 80,
        speed: 60,
    },
};
pub const BEARTIC: Pokemon = Pokemon {
    name: "Beartic",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 130,
        defense: 80,
        special_attack: 70,
        special_defense: 80,
        speed: 50,
    },
};
pub const BEAUTIFLY: Pokemon = Pokemon {
    name: "Beautifly",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 70,
        defense: 50,
        special_attack: 100,
        special_defense: 50,
        speed: 65,
    },
};
pub const BEEDRILL: Pokemon = Pokemon {
    name: "Beedrill",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 65,
        attack: 90,
        defense: 40,
        special_attack: 45,
        special_defense: 80,
        speed: 75,
    },
};
pub const BEHEEYEM: Pokemon = Pokemon {
    name: "Beheeyem",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 75,
        defense: 75,
        special_attack: 125,
        special_defense: 95,
        speed: 40,
    },
};
pub const BELDUM: Pokemon = Pokemon {
    name: "Beldum",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 40,
        attack: 55,
        defense: 80,
        special_attack: 35,
        special_defense: 60,
        speed: 30,
    },
};
pub const BELLOSSOM: Pokemon = Pokemon {
    name: "Bellossom",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 80,
        defense: 95,
        special_attack: 90,
        special_defense: 100,
        speed: 50,
    },
};
pub const BELLSPROUT: Pokemon = Pokemon {
    name: "Bellsprout",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 50,
        attack: 75,
        defense: 35,
        special_attack: 70,
        special_defense: 30,
        speed: 40,
    },
};
pub const BERGMITE: Pokemon = Pokemon {
    name: "Bergmite",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 69,
        defense: 85,
        special_attack: 32,
        special_defense: 35,
        speed: 28,
    },
};
pub const BEWEAR: Pokemon = Pokemon {
    name: "Bewear",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 120,
        attack: 125,
        defense: 80,
        special_attack: 55,
        special_defense: 60,
        speed: 60,
    },
};
pub const BIBAREL: Pokemon = Pokemon {
    name: "Bibarel",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 79,
        attack: 85,
        defense: 60,
        special_attack: 55,
        special_defense: 60,
        speed: 71,
    },
};
pub const BIDOOF: Pokemon = Pokemon {
    name: "Bidoof",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 59,
        attack: 45,
        defense: 40,
        special_attack: 35,
        special_defense: 40,
        speed: 31,
    },
};
pub const BINACLE: Pokemon = Pokemon {
    name: "Binacle",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 42,
        attack: 52,
        defense: 67,
        special_attack: 39,
        special_defense: 56,
        speed: 50,
    },
};
pub const BISHARP: Pokemon = Pokemon {
    name: "Bisharp",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 65,
        attack: 125,
        defense: 100,
        special_attack: 60,
        special_defense: 70,
        speed: 70,
    },
};
pub const BLACEPHALON: Pokemon = Pokemon {
    name: "Blacephalon",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 53,
        attack: 127,
        defense: 53,
        special_attack: 151,
        special_defense: 79,
        speed: 107,
    },
};
pub const BLASTOISE: Pokemon = Pokemon {
    name: "Blastoise",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 79,
        attack: 83,
        defense: 100,
        special_attack: 85,
        special_defense: 105,
        speed: 78,
    },
};
pub const BLAZIKEN: Pokemon = Pokemon {
    name: "Blaziken",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 80,
        attack: 120,
        defense: 70,
        special_attack: 110,
        special_defense: 70,
        speed: 80,
    },
};
pub const BLIPBUG: Pokemon = Pokemon {
    name: "Blipbug",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 25,
        attack: 20,
        defense: 20,
        special_attack: 25,
        special_defense: 45,
        speed: 45,
    },
};
pub const BLISSEY: Pokemon = Pokemon {
    name: "Blissey",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 255,
        attack: 10,
        defense: 10,
        special_attack: 75,
        special_defense: 135,
        speed: 55,
    },
};
pub const BLITZLE: Pokemon = Pokemon {
    name: "Blitzle",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 60,
        defense: 32,
        special_attack: 50,
        special_defense: 32,
        speed: 76,
    },
};
pub const BOLDORE: Pokemon = Pokemon {
    name: "Boldore",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 105,
        defense: 105,
        special_attack: 50,
        special_defense: 40,
        speed: 20,
    },
};
pub const BOLTUND: Pokemon = Pokemon {
    name: "Boltund",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 69,
        attack: 90,
        defense: 60,
        special_attack: 90,
        special_defense: 60,
        speed: 121,
    },
};
pub const BONSLY: Pokemon = Pokemon {
    name: "Bonsly",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 80,
        defense: 95,
        special_attack: 10,
        special_defense: 45,
        speed: 10,
    },
};
pub const BOUFFALANT: Pokemon = Pokemon {
    name: "Bouffalant",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 110,
        defense: 95,
        special_attack: 40,
        special_defense: 95,
        speed: 55,
    },
};
pub const BOUNSWEET: Pokemon = Pokemon {
    name: "Bounsweet",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 42,
        attack: 30,
        defense: 38,
        special_attack: 30,
        special_defense: 38,
        speed: 32,
    },
};
pub const BRAIXEN: Pokemon = Pokemon {
    name: "Braixen",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 59,
        attack: 59,
        defense: 58,
        special_attack: 90,
        special_defense: 70,
        speed: 73,
    },
};
pub const BRAVIARY: Pokemon = Pokemon {
    name: "Braviary",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 100,
        attack: 123,
        defense: 75,
        special_attack: 57,
        special_defense: 75,
        speed: 80,
    },
};
pub const BRELOOM: Pokemon = Pokemon {
    name: "Breloom",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 60,
        attack: 130,
        defense: 80,
        special_attack: 60,
        special_defense: 60,
        speed: 70,
    },
};
pub const BRIONNE: Pokemon = Pokemon {
    name: "Brionne",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 69,
        defense: 69,
        special_attack: 91,
        special_defense: 81,
        speed: 50,
    },
};
pub const BRONZONG: Pokemon = Pokemon {
    name: "Bronzong",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 67,
        attack: 89,
        defense: 116,
        special_attack: 79,
        special_defense: 116,
        speed: 33,
    },
};
pub const BRONZOR: Pokemon = Pokemon {
    name: "Bronzor",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 57,
        attack: 24,
        defense: 86,
        special_attack: 24,
        special_defense: 86,
        speed: 23,
    },
};
pub const BRUXISH: Pokemon = Pokemon {
    name: "Bruxish",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 68,
        attack: 105,
        defense: 70,
        special_attack: 70,
        special_defense: 70,
        speed: 92,
    },
};
pub const BUDEW: Pokemon = Pokemon {
    name: "Budew",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 40,
        attack: 30,
        defense: 35,
        special_attack: 50,
        special_defense: 70,
        speed: 55,
    },
};
pub const BUIZEL: Pokemon = Pokemon {
    name: "Buizel",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 65,
        defense: 35,
        special_attack: 60,
        special_defense: 30,
        speed: 85,
    },
};
pub const BULBASAUR: Pokemon = Pokemon {
    name: "Bulbasaur",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 45,
        attack: 49,
        defense: 49,
        special_attack: 65,
        special_defense: 65,
        speed: 45,
    },
};
pub const BUNEARY: Pokemon = Pokemon {
    name: "Buneary",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 66,
        defense: 44,
        special_attack: 44,
        special_defense: 56,
        speed: 85,
    },
};
pub const BUNNELBY: Pokemon = Pokemon {
    name: "Bunnelby",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 38,
        attack: 36,
        defense: 38,
        special_attack: 32,
        special_defense: 36,
        speed: 57,
    },
};
pub const BURMY: Pokemon = Pokemon {
    name: "Burmy",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 29,
        defense: 45,
        special_attack: 29,
        special_defense: 45,
        speed: 36,
    },
};
pub const BUTTERFREE: Pokemon = Pokemon {
    name: "Butterfree",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 45,
        defense: 50,
        special_attack: 90,
        special_defense: 80,
        speed: 70,
    },
};
pub const BUZZWOLE: Pokemon = Pokemon {
    name: "Buzzwole",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 107,
        attack: 139,
        defense: 139,
        special_attack: 53,
        special_defense: 53,
        speed: 79,
    },
};
pub const CACNEA: Pokemon = Pokemon {
    name: "Cacnea",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 85,
        defense: 40,
        special_attack: 85,
        special_defense: 40,
        speed: 35,
    },
};
pub const CACTURNE: Pokemon = Pokemon {
    name: "Cacturne",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 70,
        attack: 115,
        defense: 60,
        special_attack: 115,
        special_defense: 60,
        speed: 55,
    },
};
pub const CALYREX: Pokemon = Pokemon {
    name: "Calyrex",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 100,
        attack: 80,
        defense: 80,
        special_attack: 80,
        special_defense: 80,
        speed: 80,
    },
};
pub const CAMERUPT: Pokemon = Pokemon {
    name: "Camerupt",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 70,
        attack: 100,
        defense: 70,
        special_attack: 105,
        special_defense: 75,
        speed: 40,
    },
};
pub const CARBINK: Pokemon = Pokemon {
    name: "Carbink",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 50,
        attack: 50,
        defense: 150,
        special_attack: 50,
        special_defense: 150,
        speed: 50,
    },
};
pub const CARKOL: Pokemon = Pokemon {
    name: "Carkol",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 80,
        attack: 60,
        defense: 90,
        special_attack: 60,
        special_defense: 70,
        speed: 50,
    },
};
pub const CARNIVINE: Pokemon = Pokemon {
    name: "Carnivine",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 74,
        attack: 100,
        defense: 72,
        special_attack: 90,
        special_defense: 72,
        speed: 46,
    },
};
pub const CARRACOSTA: Pokemon = Pokemon {
    name: "Carracosta",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 74,
        attack: 108,
        defense: 133,
        special_attack: 83,
        special_defense: 65,
        speed: 32,
    },
};
pub const CARVANHA: Pokemon = Pokemon {
    name: "Carvanha",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 45,
        attack: 90,
        defense: 20,
        special_attack: 65,
        special_defense: 20,
        speed: 65,
    },
};
pub const CASCOON: Pokemon = Pokemon {
    name: "Cascoon",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 35,
        defense: 55,
        special_attack: 25,
        special_defense: 25,
        speed: 15,
    },
};
pub const CASTFORM: Pokemon = Pokemon {
    name: "Castform",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 70,
        defense: 70,
        special_attack: 70,
        special_defense: 70,
        speed: 70,
    },
};
pub const CATERPIE: Pokemon = Pokemon {
    name: "Caterpie",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 30,
        defense: 35,
        special_attack: 20,
        special_defense: 20,
        speed: 45,
    },
};
pub const CELEBI: Pokemon = Pokemon {
    name: "Celebi",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
    },
};
pub const CELESTEELA: Pokemon = Pokemon {
    name: "Celesteela",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 97,
        attack: 101,
        defense: 103,
        special_attack: 107,
        special_defense: 101,
        speed: 61,
    },
};
pub const CENTISKORCH: Pokemon = Pokemon {
    name: "Centiskorch",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 100,
        attack: 115,
        defense: 65,
        special_attack: 90,
        special_defense: 90,
        speed: 65,
    },
};
pub const CHANDELURE: Pokemon = Pokemon {
    name: "Chandelure",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 60,
        attack: 55,
        defense: 90,
        special_attack: 145,
        special_defense: 90,
        speed: 80,
    },
};
pub const CHANSEY: Pokemon = Pokemon {
    name: "Chansey",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 250,
        attack: 5,
        defense: 5,
        special_attack: 35,
        special_defense: 105,
        speed: 50,
    },
};
pub const CHARIZARD: Pokemon = Pokemon {
    name: "Charizard",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 78,
        attack: 84,
        defense: 78,
        special_attack: 109,
        special_defense: 85,
        speed: 100,
    },
};
pub const CHARJABUG: Pokemon = Pokemon {
    name: "Charjabug",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 57,
        attack: 82,
        defense: 95,
        special_attack: 55,
        special_defense: 75,
        speed: 36,
    },
};
pub const CHARMANDER: Pokemon = Pokemon {
    name: "Charmander",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 39,
        attack: 52,
        defense: 43,
        special_attack: 60,
        special_defense: 50,
        speed: 65,
    },
};
pub const CHARMELEON: Pokemon = Pokemon {
    name: "Charmeleon",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 58,
        attack: 64,
        defense: 58,
        special_attack: 80,
        special_defense: 65,
        speed: 80,
    },
};
pub const CHATOT: Pokemon = Pokemon {
    name: "Chatot",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 76,
        attack: 65,
        defense: 45,
        special_attack: 92,
        special_defense: 42,
        speed: 91,
    },
};
pub const CHERRIM: Pokemon = Pokemon {
    name: "Cherrim",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 60,
        defense: 70,
        special_attack: 87,
        special_defense: 78,
        speed: 85,
    },
};
pub const CHERUBI: Pokemon = Pokemon {
    name: "Cherubi",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 35,
        defense: 45,
        special_attack: 62,
        special_defense: 53,
        speed: 35,
    },
};
pub const CHESNAUGHT: Pokemon = Pokemon {
    name: "Chesnaught",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 88,
        attack: 107,
        defense: 122,
        special_attack: 74,
        special_defense: 75,
        speed: 64,
    },
};
pub const CHESPIN: Pokemon = Pokemon {
    name: "Chespin",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 56,
        attack: 61,
        defense: 65,
        special_attack: 48,
        special_defense: 45,
        speed: 38,
    },
};
pub const CHEWTLE: Pokemon = Pokemon {
    name: "Chewtle",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 64,
        defense: 50,
        special_attack: 38,
        special_defense: 38,
        speed: 44,
    },
};
pub const CHIKORITA: Pokemon = Pokemon {
    name: "Chikorita",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 49,
        defense: 65,
        special_attack: 49,
        special_defense: 65,
        speed: 45,
    },
};
pub const CHIMCHAR: Pokemon = Pokemon {
    name: "Chimchar",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 44,
        attack: 58,
        defense: 44,
        special_attack: 58,
        special_defense: 44,
        speed: 61,
    },
};
pub const CHIMECHO: Pokemon = Pokemon {
    name: "Chimecho",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 50,
        defense: 80,
        special_attack: 95,
        special_defense: 90,
        speed: 65,
    },
};
pub const CHINCHOU: Pokemon = Pokemon {
    name: "Chinchou",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 75,
        attack: 38,
        defense: 38,
        special_attack: 56,
        special_defense: 56,
        speed: 67,
    },
};
pub const CHINGLING: Pokemon = Pokemon {
    name: "Chingling",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 30,
        defense: 50,
        special_attack: 65,
        special_defense: 50,
        speed: 45,
    },
};
pub const CINCCINO: Pokemon = Pokemon {
    name: "Cinccino",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 95,
        defense: 60,
        special_attack: 65,
        special_defense: 60,
        speed: 115,
    },
};
pub const CINDERACE: Pokemon = Pokemon {
    name: "Cinderace",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 116,
        defense: 75,
        special_attack: 65,
        special_defense: 75,
        speed: 119,
    },
};
pub const CLAMPERL: Pokemon = Pokemon {
    name: "Clamperl",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 64,
        defense: 85,
        special_attack: 74,
        special_defense: 55,
        speed: 32,
    },
};
pub const CLAUNCHER: Pokemon = Pokemon {
    name: "Clauncher",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 53,
        defense: 62,
        special_attack: 58,
        special_defense: 63,
        speed: 44,
    },
};
pub const CLAWITZER: Pokemon = Pokemon {
    name: "Clawitzer",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 71,
        attack: 73,
        defense: 88,
        special_attack: 120,
        special_defense: 89,
        speed: 59,
    },
};
pub const CLAYDOL: Pokemon = Pokemon {
    name: "Claydol",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 60,
        attack: 70,
        defense: 105,
        special_attack: 70,
        special_defense: 120,
        speed: 75,
    },
};
pub const CLEFABLE: Pokemon = Pokemon {
    name: "Clefable",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 70,
        defense: 73,
        special_attack: 95,
        special_defense: 90,
        speed: 60,
    },
};
pub const CLEFABLE_GEN1: Pokemon = Pokemon {
    name: "Clefable",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 70,
        defense: 73,
        special_attack: 95,
        special_defense: 90,
        speed: 60,
    },
};
pub const CLEFAIRY: Pokemon = Pokemon {
    name: "Clefairy",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 45,
        defense: 48,
        special_attack: 60,
        special_defense: 65,
        speed: 35,
    },
};
pub const CLEFAIRY_GEN1: Pokemon = Pokemon {
    name: "Clefairy",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 45,
        defense: 48,
        special_attack: 60,
        special_defense: 65,
        speed: 35,
    },
};
pub const CLEFFA: Pokemon = Pokemon {
    name: "Cleffa",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 25,
        defense: 28,
        special_attack: 45,
        special_defense: 55,
        speed: 15,
    },
};
pub const CLEFFA_GEN2: Pokemon = Pokemon {
    name: "Cleffa",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 25,
        defense: 28,
        special_attack: 45,
        special_defense: 55,
        speed: 15,
    },
};
pub const CLOBBOPUS: Pokemon = Pokemon {
    name: "Clobbopus",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 68,
        defense: 60,
        special_attack: 50,
        special_defense: 50,
        speed: 32,
    },
};
pub const CLOYSTER: Pokemon = Pokemon {
    name: "Cloyster",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 50,
        attack: 95,
        defense: 180,
        special_attack: 85,
        special_defense: 45,
        speed: 70,
    },
};
pub const COALOSSAL: Pokemon = Pokemon {
    name: "Coalossal",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 110,
        attack: 80,
        defense: 120,
        special_attack: 80,
        special_defense: 90,
        speed: 30,
    },
};
pub const COBALION: Pokemon = Pokemon {
    name: "Cobalion",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 91,
        attack: 90,
        defense: 129,
        special_attack: 90,
        special_defense: 72,
        speed: 108,
    },
};
pub const COFAGRIGUS: Pokemon = Pokemon {
    name: "Cofagrigus",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 58,
        attack: 50,
        defense: 145,
        special_attack: 95,
        special_defense: 105,
        speed: 30,
    },
};
pub const COMBEE: Pokemon = Pokemon {
    name: "Combee",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 30,
        attack: 30,
        defense: 42,
        special_attack: 30,
        special_defense: 42,
        speed: 70,
    },
};
pub const COMBUSKEN: Pokemon = Pokemon {
    name: "Combusken",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 60,
        attack: 85,
        defense: 60,
        special_attack: 85,
        special_defense: 60,
        speed: 55,
    },
};
pub const COMFEY: Pokemon = Pokemon {
    name: "Comfey",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 51,
        attack: 52,
        defense: 90,
        special_attack: 82,
        special_defense: 110,
        speed: 100,
    },
};
pub const CONKELDURR: Pokemon = Pokemon {
    name: "Conkeldurr",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 105,
        attack: 140,
        defense: 95,
        special_attack: 55,
        special_defense: 65,
        speed: 45,
    },
};
pub const COPPERAJAH: Pokemon = Pokemon {
    name: "Copperajah",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 122,
        attack: 130,
        defense: 69,
        special_attack: 80,
        special_defense: 69,
        speed: 30,
    },
};
pub const CORPHISH: Pokemon = Pokemon {
    name: "Corphish",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 43,
        attack: 80,
        defense: 65,
        special_attack: 50,
        special_defense: 35,
        speed: 35,
    },
};
pub const CORSOLA: Pokemon = Pokemon {
    name: "Corsola",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 65,
        attack: 55,
        defense: 95,
        special_attack: 65,
        special_defense: 95,
        speed: 35,
    },
};
pub const CORVIKNIGHT: Pokemon = Pokemon {
    name: "Corviknight",
    typeset: TypeData {
        primary: PokemonType::Flying,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 98,
        attack: 87,
        defense: 105,
        special_attack: 53,
        special_defense: 85,
        speed: 67,
    },
};
pub const CORVISQUIRE: Pokemon = Pokemon {
    name: "Corvisquire",
    typeset: TypeData {
        primary: PokemonType::Flying,
        secondary: None,
    },
    stats: Stats {
        hp: 68,
        attack: 67,
        defense: 55,
        special_attack: 43,
        special_defense: 55,
        speed: 77,
    },
};
pub const COSMOEM: Pokemon = Pokemon {
    name: "Cosmoem",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 43,
        attack: 29,
        defense: 131,
        special_attack: 29,
        special_defense: 131,
        speed: 37,
    },
};
pub const COSMOG: Pokemon = Pokemon {
    name: "Cosmog",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 43,
        attack: 29,
        defense: 31,
        special_attack: 29,
        special_defense: 31,
        speed: 37,
    },
};
pub const COTTONEE: Pokemon = Pokemon {
    name: "Cottonee",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 40,
        attack: 27,
        defense: 60,
        special_attack: 37,
        special_defense: 50,
        speed: 66,
    },
};
pub const CRABOMINABLE: Pokemon = Pokemon {
    name: "Crabominable",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 97,
        attack: 132,
        defense: 77,
        special_attack: 62,
        special_defense: 67,
        speed: 43,
    },
};
pub const CRABRAWLER: Pokemon = Pokemon {
    name: "Crabrawler",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 47,
        attack: 82,
        defense: 57,
        special_attack: 42,
        special_defense: 47,
        speed: 63,
    },
};
pub const CRADILY: Pokemon = Pokemon {
    name: "Cradily",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 86,
        attack: 81,
        defense: 97,
        special_attack: 81,
        special_defense: 107,
        speed: 43,
    },
};
pub const CRAMORANT: Pokemon = Pokemon {
    name: "Cramorant",
    typeset: TypeData {
        primary: PokemonType::Flying,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 70,
        attack: 85,
        defense: 55,
        special_attack: 85,
        special_defense: 95,
        speed: 85,
    },
};
pub const CRANIDOS: Pokemon = Pokemon {
    name: "Cranidos",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 67,
        attack: 125,
        defense: 40,
        special_attack: 30,
        special_defense: 30,
        speed: 58,
    },
};
pub const CRAWDAUNT: Pokemon = Pokemon {
    name: "Crawdaunt",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 63,
        attack: 120,
        defense: 85,
        special_attack: 90,
        special_defense: 55,
        speed: 55,
    },
};
pub const CRESSELIA: Pokemon = Pokemon {
    name: "Cresselia",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 120,
        attack: 70,
        defense: 120,
        special_attack: 75,
        special_defense: 130,
        speed: 85,
    },
};
pub const CROAGUNK: Pokemon = Pokemon {
    name: "Croagunk",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 48,
        attack: 61,
        defense: 40,
        special_attack: 61,
        special_defense: 40,
        speed: 50,
    },
};
pub const CROBAT: Pokemon = Pokemon {
    name: "Crobat",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 85,
        attack: 90,
        defense: 80,
        special_attack: 70,
        special_defense: 80,
        speed: 130,
    },
};
pub const CROCONAW: Pokemon = Pokemon {
    name: "Croconaw",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 80,
        defense: 80,
        special_attack: 59,
        special_defense: 63,
        speed: 58,
    },
};
pub const CRUSTLE: Pokemon = Pokemon {
    name: "Crustle",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 70,
        attack: 105,
        defense: 125,
        special_attack: 65,
        special_defense: 75,
        speed: 45,
    },
};
pub const CRYOGONAL: Pokemon = Pokemon {
    name: "Cryogonal",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 50,
        defense: 50,
        special_attack: 95,
        special_defense: 135,
        speed: 105,
    },
};
pub const CUBCHOO: Pokemon = Pokemon {
    name: "Cubchoo",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 70,
        defense: 40,
        special_attack: 60,
        special_defense: 40,
        speed: 40,
    },
};
pub const CUBONE: Pokemon = Pokemon {
    name: "Cubone",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 50,
        defense: 95,
        special_attack: 40,
        special_defense: 50,
        speed: 35,
    },
};
pub const CUFANT: Pokemon = Pokemon {
    name: "Cufant",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 72,
        attack: 80,
        defense: 49,
        special_attack: 40,
        special_defense: 49,
        speed: 40,
    },
};
pub const CURSOLA: Pokemon = Pokemon {
    name: "Cursola",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 95,
        defense: 50,
        special_attack: 145,
        special_defense: 130,
        speed: 30,
    },
};
pub const CUTIEFLY: Pokemon = Pokemon {
    name: "Cutiefly",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 40,
        special_attack: 55,
        special_defense: 40,
        speed: 84,
    },
};
pub const CYNDAQUIL: Pokemon = Pokemon {
    name: "Cyndaquil",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 39,
        attack: 52,
        defense: 43,
        special_attack: 60,
        special_defense: 50,
        speed: 65,
    },
};
pub const DARKRAI: Pokemon = Pokemon {
    name: "Darkrai",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 90,
        defense: 90,
        special_attack: 135,
        special_defense: 90,
        speed: 125,
    },
};
pub const DARMANITAN: Pokemon = Pokemon {
    name: "Darmanitan",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 105,
        attack: 140,
        defense: 55,
        special_attack: 30,
        special_defense: 55,
        speed: 95,
    },
};
pub const DARTRIX: Pokemon = Pokemon {
    name: "Dartrix",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 78,
        attack: 75,
        defense: 75,
        special_attack: 70,
        special_defense: 70,
        speed: 52,
    },
};
pub const DARUMAKA: Pokemon = Pokemon {
    name: "Darumaka",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 90,
        defense: 45,
        special_attack: 15,
        special_defense: 45,
        speed: 50,
    },
};
pub const DECIDUEYE: Pokemon = Pokemon {
    name: "Decidueye",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 78,
        attack: 107,
        defense: 75,
        special_attack: 100,
        special_defense: 100,
        speed: 70,
    },
};
pub const DEDENNE: Pokemon = Pokemon {
    name: "Dedenne",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 67,
        attack: 58,
        defense: 57,
        special_attack: 81,
        special_defense: 67,
        speed: 101,
    },
};
pub const DEERLING: Pokemon = Pokemon {
    name: "Deerling",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 50,
        special_attack: 40,
        special_defense: 50,
        speed: 75,
    },
};
pub const DEINO: Pokemon = Pokemon {
    name: "Deino",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 52,
        attack: 65,
        defense: 50,
        special_attack: 45,
        special_defense: 50,
        speed: 38,
    },
};
pub const DELCATTY: Pokemon = Pokemon {
    name: "Delcatty",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 65,
        defense: 65,
        special_attack: 55,
        special_defense: 55,
        speed: 90,
    },
};
pub const DELIBIRD: Pokemon = Pokemon {
    name: "Delibird",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 45,
        attack: 55,
        defense: 45,
        special_attack: 65,
        special_defense: 45,
        speed: 75,
    },
};
pub const DELPHOX: Pokemon = Pokemon {
    name: "Delphox",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 75,
        attack: 69,
        defense: 72,
        special_attack: 114,
        special_defense: 100,
        speed: 104,
    },
};
pub const DEOXYS: Pokemon = Pokemon {
    name: "Deoxys",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 150,
        defense: 50,
        special_attack: 150,
        special_defense: 50,
        speed: 150,
    },
};
pub const DEWGONG: Pokemon = Pokemon {
    name: "Dewgong",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 90,
        attack: 70,
        defense: 80,
        special_attack: 70,
        special_defense: 95,
        speed: 70,
    },
};
pub const DEWOTT: Pokemon = Pokemon {
    name: "Dewott",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 75,
        defense: 60,
        special_attack: 83,
        special_defense: 60,
        speed: 60,
    },
};
pub const DEWPIDER: Pokemon = Pokemon {
    name: "Dewpider",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 38,
        attack: 40,
        defense: 52,
        special_attack: 40,
        special_defense: 72,
        speed: 27,
    },
};
pub const DHELMISE: Pokemon = Pokemon {
    name: "Dhelmise",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 70,
        attack: 131,
        defense: 100,
        special_attack: 86,
        special_defense: 90,
        speed: 40,
    },
};
pub const DIALGA: Pokemon = Pokemon {
    name: "Dialga",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 100,
        attack: 120,
        defense: 120,
        special_attack: 150,
        special_defense: 100,
        speed: 90,
    },
};
pub const DIANCIE: Pokemon = Pokemon {
    name: "Diancie",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 50,
        attack: 100,
        defense: 150,
        special_attack: 100,
        special_defense: 150,
        speed: 50,
    },
};
pub const DIGGERSBY: Pokemon = Pokemon {
    name: "Diggersby",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 85,
        attack: 56,
        defense: 77,
        special_attack: 50,
        special_defense: 77,
        speed: 78,
    },
};
pub const DIGLETT: Pokemon = Pokemon {
    name: "Diglett",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 10,
        attack: 55,
        defense: 25,
        special_attack: 35,
        special_defense: 45,
        speed: 95,
    },
};
pub const DITTO: Pokemon = Pokemon {
    name: "Ditto",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 48,
        attack: 48,
        defense: 48,
        special_attack: 48,
        special_defense: 48,
        speed: 48,
    },
};
pub const DODRIO: Pokemon = Pokemon {
    name: "Dodrio",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 110,
        defense: 70,
        special_attack: 60,
        special_defense: 60,
        speed: 110,
    },
};
pub const DODUO: Pokemon = Pokemon {
    name: "Doduo",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 35,
        attack: 85,
        defense: 45,
        special_attack: 35,
        special_defense: 35,
        speed: 75,
    },
};
pub const DONPHAN: Pokemon = Pokemon {
    name: "Donphan",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 120,
        defense: 120,
        special_attack: 60,
        special_defense: 60,
        speed: 50,
    },
};
pub const DOTTLER: Pokemon = Pokemon {
    name: "Dottler",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 50,
        attack: 35,
        defense: 80,
        special_attack: 50,
        special_defense: 90,
        speed: 30,
    },
};
pub const DOUBLADE: Pokemon = Pokemon {
    name: "Doublade",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 59,
        attack: 110,
        defense: 150,
        special_attack: 45,
        special_defense: 49,
        speed: 35,
    },
};
pub const DRACOVISH: Pokemon = Pokemon {
    name: "Dracovish",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 90,
        attack: 90,
        defense: 100,
        special_attack: 70,
        special_defense: 80,
        speed: 75,
    },
};
pub const DRACOZOLT: Pokemon = Pokemon {
    name: "Dracozolt",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 90,
        attack: 100,
        defense: 90,
        special_attack: 80,
        special_defense: 70,
        speed: 75,
    },
};
pub const DRAGALGE: Pokemon = Pokemon {
    name: "Dragalge",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 65,
        attack: 75,
        defense: 90,
        special_attack: 97,
        special_defense: 123,
        speed: 44,
    },
};
pub const DRAGAPULT: Pokemon = Pokemon {
    name: "Dragapult",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 88,
        attack: 120,
        defense: 75,
        special_attack: 100,
        special_defense: 75,
        speed: 142,
    },
};
pub const DRAGONAIR: Pokemon = Pokemon {
    name: "Dragonair",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 61,
        attack: 84,
        defense: 65,
        special_attack: 70,
        special_defense: 70,
        speed: 70,
    },
};
pub const DRAGONITE: Pokemon = Pokemon {
    name: "Dragonite",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 91,
        attack: 134,
        defense: 95,
        special_attack: 100,
        special_defense: 100,
        speed: 80,
    },
};
pub const DRAKLOAK: Pokemon = Pokemon {
    name: "Drakloak",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 68,
        attack: 80,
        defense: 50,
        special_attack: 60,
        special_defense: 50,
        speed: 102,
    },
};
pub const DRAMPA: Pokemon = Pokemon {
    name: "Drampa",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 78,
        attack: 60,
        defense: 85,
        special_attack: 135,
        special_defense: 91,
        speed: 36,
    },
};
pub const DRAPION: Pokemon = Pokemon {
    name: "Drapion",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 70,
        attack: 90,
        defense: 110,
        special_attack: 60,
        special_defense: 75,
        speed: 95,
    },
};
pub const DRATINI: Pokemon = Pokemon {
    name: "Dratini",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 41,
        attack: 64,
        defense: 45,
        special_attack: 50,
        special_defense: 50,
        speed: 50,
    },
};
pub const DREDNAW: Pokemon = Pokemon {
    name: "Drednaw",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 90,
        attack: 115,
        defense: 90,
        special_attack: 48,
        special_defense: 68,
        speed: 74,
    },
};
pub const DREEPY: Pokemon = Pokemon {
    name: "Dreepy",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 28,
        attack: 60,
        defense: 30,
        special_attack: 40,
        special_defense: 30,
        speed: 82,
    },
};
pub const DRIFBLIM: Pokemon = Pokemon {
    name: "Drifblim",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 150,
        attack: 80,
        defense: 44,
        special_attack: 90,
        special_defense: 54,
        speed: 80,
    },
};
pub const DRIFLOON: Pokemon = Pokemon {
    name: "Drifloon",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 90,
        attack: 50,
        defense: 34,
        special_attack: 60,
        special_defense: 44,
        speed: 70,
    },
};
pub const DRILBUR: Pokemon = Pokemon {
    name: "Drilbur",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 85,
        defense: 40,
        special_attack: 30,
        special_defense: 45,
        speed: 68,
    },
};
pub const DRIZZILE: Pokemon = Pokemon {
    name: "Drizzile",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 60,
        defense: 55,
        special_attack: 95,
        special_defense: 55,
        speed: 90,
    },
};
pub const DROWZEE: Pokemon = Pokemon {
    name: "Drowzee",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 48,
        defense: 45,
        special_attack: 43,
        special_defense: 90,
        speed: 42,
    },
};
pub const DRUDDIGON: Pokemon = Pokemon {
    name: "Druddigon",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 77,
        attack: 120,
        defense: 90,
        special_attack: 60,
        special_defense: 90,
        speed: 48,
    },
};
pub const DUBWOOL: Pokemon = Pokemon {
    name: "Dubwool",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 72,
        attack: 80,
        defense: 100,
        special_attack: 60,
        special_defense: 90,
        speed: 88,
    },
};
pub const DUCKLETT: Pokemon = Pokemon {
    name: "Ducklett",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 62,
        attack: 44,
        defense: 50,
        special_attack: 44,
        special_defense: 50,
        speed: 55,
    },
};
pub const DUGTRIO: Pokemon = Pokemon {
    name: "Dugtrio",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 100,
        defense: 50,
        special_attack: 50,
        special_defense: 70,
        speed: 120,
    },
};
pub const DUNSPARCE: Pokemon = Pokemon {
    name: "Dunsparce",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 70,
        defense: 70,
        special_attack: 65,
        special_defense: 65,
        speed: 45,
    },
};
pub const DUOSION: Pokemon = Pokemon {
    name: "Duosion",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 40,
        defense: 50,
        special_attack: 125,
        special_defense: 60,
        speed: 30,
    },
};
pub const DURALUDON: Pokemon = Pokemon {
    name: "Duraludon",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 70,
        attack: 95,
        defense: 115,
        special_attack: 120,
        special_defense: 50,
        speed: 85,
    },
};
pub const DURANT: Pokemon = Pokemon {
    name: "Durant",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 58,
        attack: 109,
        defense: 112,
        special_attack: 48,
        special_defense: 48,
        speed: 109,
    },
};
pub const DUSCLOPS: Pokemon = Pokemon {
    name: "Dusclops",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 70,
        defense: 130,
        special_attack: 60,
        special_defense: 130,
        speed: 25,
    },
};
pub const DUSKNOIR: Pokemon = Pokemon {
    name: "Dusknoir",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 100,
        defense: 135,
        special_attack: 65,
        special_defense: 135,
        speed: 45,
    },
};
pub const DUSKULL: Pokemon = Pokemon {
    name: "Duskull",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 20,
        attack: 40,
        defense: 90,
        special_attack: 30,
        special_defense: 90,
        speed: 25,
    },
};
pub const DUSTOX: Pokemon = Pokemon {
    name: "Dustox",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 70,
        special_attack: 50,
        special_defense: 90,
        speed: 65,
    },
};
pub const DWEBBLE: Pokemon = Pokemon {
    name: "Dwebble",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 50,
        attack: 65,
        defense: 85,
        special_attack: 35,
        special_defense: 35,
        speed: 55,
    },
};
pub const EELEKTRIK: Pokemon = Pokemon {
    name: "Eelektrik",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 85,
        defense: 70,
        special_attack: 75,
        special_defense: 70,
        speed: 40,
    },
};
pub const EELEKTROSS: Pokemon = Pokemon {
    name: "Eelektross",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 115,
        defense: 80,
        special_attack: 105,
        special_defense: 80,
        speed: 50,
    },
};
pub const EEVEE: Pokemon = Pokemon {
    name: "Eevee",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 55,
        defense: 50,
        special_attack: 45,
        special_defense: 65,
        speed: 55,
    },
};
pub const EISCUE: Pokemon = Pokemon {
    name: "Eiscue",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 80,
        defense: 110,
        special_attack: 65,
        special_defense: 90,
        speed: 50,
    },
};
pub const EKANS: Pokemon = Pokemon {
    name: "Ekans",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 60,
        defense: 44,
        special_attack: 40,
        special_defense: 54,
        speed: 55,
    },
};
pub const ELDEGOSS: Pokemon = Pokemon {
    name: "Eldegoss",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 90,
        special_attack: 80,
        special_defense: 120,
        speed: 60,
    },
};
pub const ELECTABUZZ: Pokemon = Pokemon {
    name: "Electabuzz",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 83,
        defense: 57,
        special_attack: 95,
        special_defense: 85,
        speed: 105,
    },
};
pub const ELECTIVIRE: Pokemon = Pokemon {
    name: "Electivire",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 123,
        defense: 67,
        special_attack: 95,
        special_defense: 85,
        speed: 95,
    },
};
pub const ELECTRIKE: Pokemon = Pokemon {
    name: "Electrike",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 40,
        special_attack: 65,
        special_defense: 40,
        speed: 65,
    },
};
pub const ELECTRODE: Pokemon = Pokemon {
    name: "Electrode",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 70,
        special_attack: 80,
        special_defense: 80,
        speed: 150,
    },
};
pub const ELEKID: Pokemon = Pokemon {
    name: "Elekid",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 63,
        defense: 37,
        special_attack: 65,
        special_defense: 55,
        speed: 95,
    },
};
pub const ELGYEM: Pokemon = Pokemon {
    name: "Elgyem",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 55,
        defense: 55,
        special_attack: 85,
        special_defense: 55,
        speed: 30,
    },
};
pub const EMBOAR: Pokemon = Pokemon {
    name: "Emboar",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 110,
        attack: 123,
        defense: 65,
        special_attack: 100,
        special_defense: 65,
        speed: 65,
    },
};
pub const EMOLGA: Pokemon = Pokemon {
    name: "Emolga",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 75,
        defense: 60,
        special_attack: 75,
        special_defense: 60,
        speed: 103,
    },
};
pub const EMPOLEON: Pokemon = Pokemon {
    name: "Empoleon",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 84,
        attack: 86,
        defense: 88,
        special_attack: 111,
        special_defense: 101,
        speed: 60,
    },
};
pub const ENTEI: Pokemon = Pokemon {
    name: "Entei",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 115,
        attack: 115,
        defense: 85,
        special_attack: 90,
        special_defense: 75,
        speed: 100,
    },
};
pub const ESCAVALIER: Pokemon = Pokemon {
    name: "Escavalier",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 70,
        attack: 135,
        defense: 105,
        special_attack: 60,
        special_defense: 105,
        speed: 20,
    },
};
pub const ESPEON: Pokemon = Pokemon {
    name: "Espeon",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 65,
        defense: 60,
        special_attack: 130,
        special_defense: 95,
        speed: 110,
    },
};
pub const ESPURR: Pokemon = Pokemon {
    name: "Espurr",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 62,
        attack: 48,
        defense: 54,
        special_attack: 63,
        special_defense: 60,
        speed: 68,
    },
};
pub const ETERNATUS: Pokemon = Pokemon {
    name: "Eternatus",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 140,
        attack: 85,
        defense: 95,
        special_attack: 145,
        special_defense: 95,
        speed: 130,
    },
};
pub const EXCADRILL: Pokemon = Pokemon {
    name: "Excadrill",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 110,
        attack: 135,
        defense: 60,
        special_attack: 50,
        special_defense: 65,
        speed: 88,
    },
};
pub const EXEGGCUTE: Pokemon = Pokemon {
    name: "Exeggcute",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 60,
        attack: 40,
        defense: 80,
        special_attack: 60,
        special_defense: 45,
        speed: 40,
    },
};
pub const EXEGGUTOR: Pokemon = Pokemon {
    name: "Exeggutor",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 95,
        attack: 95,
        defense: 85,
        special_attack: 125,
        special_defense: 75,
        speed: 55,
    },
};
pub const EXPLOUD: Pokemon = Pokemon {
    name: "Exploud",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 104,
        attack: 91,
        defense: 63,
        special_attack: 91,
        special_defense: 73,
        speed: 68,
    },
};
pub const FALINKS: Pokemon = Pokemon {
    name: "Falinks",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 100,
        defense: 100,
        special_attack: 70,
        special_defense: 60,
        speed: 75,
    },
};
pub const FARFETCHD: Pokemon = Pokemon {
    name: "Farfetch'd",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 52,
        attack: 90,
        defense: 55,
        special_attack: 58,
        special_defense: 62,
        speed: 60,
    },
};
pub const FEAROW: Pokemon = Pokemon {
    name: "Fearow",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 65,
        attack: 90,
        defense: 65,
        special_attack: 61,
        special_defense: 61,
        speed: 100,
    },
};
pub const FEEBAS: Pokemon = Pokemon {
    name: "Feebas",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 20,
        attack: 15,
        defense: 20,
        special_attack: 10,
        special_defense: 55,
        speed: 80,
    },
};
pub const FENNEKIN: Pokemon = Pokemon {
    name: "Fennekin",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 40,
        special_attack: 62,
        special_defense: 60,
        speed: 60,
    },
};
pub const FERALIGATR: Pokemon = Pokemon {
    name: "Feraligatr",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 105,
        defense: 100,
        special_attack: 79,
        special_defense: 83,
        speed: 78,
    },
};
pub const FERROSEED: Pokemon = Pokemon {
    name: "Ferroseed",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 44,
        attack: 50,
        defense: 91,
        special_attack: 24,
        special_defense: 86,
        speed: 10,
    },
};
pub const FERROTHORN: Pokemon = Pokemon {
    name: "Ferrothorn",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 74,
        attack: 94,
        defense: 131,
        special_attack: 54,
        special_defense: 116,
        speed: 20,
    },
};
pub const FINNEON: Pokemon = Pokemon {
    name: "Finneon",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 49,
        attack: 49,
        defense: 56,
        special_attack: 49,
        special_defense: 61,
        speed: 66,
    },
};
pub const FLAAFFY: Pokemon = Pokemon {
    name: "Flaaffy",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 55,
        defense: 55,
        special_attack: 80,
        special_defense: 60,
        speed: 45,
    },
};
pub const FLABEBE: Pokemon = Pokemon {
    name: "Flabebe",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 44,
        attack: 38,
        defense: 39,
        special_attack: 61,
        special_defense: 79,
        speed: 42,
    },
};
pub const FLAPPLE: Pokemon = Pokemon {
    name: "Flapple",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 70,
        attack: 110,
        defense: 80,
        special_attack: 95,
        special_defense: 60,
        speed: 70,
    },
};
pub const FLAREON: Pokemon = Pokemon {
    name: "Flareon",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 130,
        defense: 60,
        special_attack: 95,
        special_defense: 110,
        speed: 65,
    },
};
pub const FLETCHINDER: Pokemon = Pokemon {
    name: "Fletchinder",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 62,
        attack: 73,
        defense: 55,
        special_attack: 56,
        special_defense: 52,
        speed: 84,
    },
};
pub const FLETCHLING: Pokemon = Pokemon {
    name: "Fletchling",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 45,
        attack: 50,
        defense: 43,
        special_attack: 40,
        special_defense: 38,
        speed: 62,
    },
};
pub const FLOATZEL: Pokemon = Pokemon {
    name: "Floatzel",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 105,
        defense: 55,
        special_attack: 85,
        special_defense: 50,
        speed: 115,
    },
};
pub const FLOETTE: Pokemon = Pokemon {
    name: "Floette",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 54,
        attack: 45,
        defense: 47,
        special_attack: 75,
        special_defense: 98,
        speed: 52,
    },
};
pub const FLORGES: Pokemon = Pokemon {
    name: "Florges",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 78,
        attack: 65,
        defense: 68,
        special_attack: 112,
        special_defense: 154,
        speed: 75,
    },
};
pub const FLYGON: Pokemon = Pokemon {
    name: "Flygon",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 80,
        attack: 100,
        defense: 80,
        special_attack: 80,
        special_defense: 80,
        speed: 100,
    },
};
pub const FOMANTIS: Pokemon = Pokemon {
    name: "Fomantis",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 55,
        defense: 35,
        special_attack: 50,
        special_defense: 35,
        speed: 35,
    },
};
pub const FOONGUS: Pokemon = Pokemon {
    name: "Foongus",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 69,
        attack: 55,
        defense: 45,
        special_attack: 55,
        special_defense: 55,
        speed: 15,
    },
};
pub const FORRETRESS: Pokemon = Pokemon {
    name: "Forretress",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 75,
        attack: 90,
        defense: 140,
        special_attack: 60,
        special_defense: 60,
        speed: 40,
    },
};
pub const FRAXURE: Pokemon = Pokemon {
    name: "Fraxure",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 66,
        attack: 117,
        defense: 70,
        special_attack: 40,
        special_defense: 50,
        speed: 67,
    },
};
pub const FRILLISH: Pokemon = Pokemon {
    name: "Frillish",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 55,
        attack: 40,
        defense: 50,
        special_attack: 65,
        special_defense: 85,
        speed: 40,
    },
};
pub const FROAKIE: Pokemon = Pokemon {
    name: "Froakie",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 41,
        attack: 56,
        defense: 40,
        special_attack: 62,
        special_defense: 44,
        speed: 71,
    },
};
pub const FROGADIER: Pokemon = Pokemon {
    name: "Frogadier",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 54,
        attack: 63,
        defense: 52,
        special_attack: 83,
        special_defense: 56,
        speed: 97,
    },
};
pub const FROSLASS: Pokemon = Pokemon {
    name: "Froslass",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 70,
        attack: 80,
        defense: 70,
        special_attack: 80,
        special_defense: 70,
        speed: 110,
    },
};
pub const FROSMOTH: Pokemon = Pokemon {
    name: "Frosmoth",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 70,
        attack: 65,
        defense: 60,
        special_attack: 125,
        special_defense: 90,
        speed: 65,
    },
};
pub const FURFROU: Pokemon = Pokemon {
    name: "Furfrou",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 80,
        defense: 60,
        special_attack: 65,
        special_defense: 90,
        speed: 102,
    },
};
pub const FURRET: Pokemon = Pokemon {
    name: "Furret",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 76,
        defense: 64,
        special_attack: 45,
        special_defense: 55,
        speed: 90,
    },
};
pub const GABITE: Pokemon = Pokemon {
    name: "Gabite",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 68,
        attack: 90,
        defense: 65,
        special_attack: 50,
        special_defense: 55,
        speed: 82,
    },
};
pub const GALLADE: Pokemon = Pokemon {
    name: "Gallade",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 68,
        attack: 125,
        defense: 65,
        special_attack: 65,
        special_defense: 115,
        speed: 80,
    },
};
pub const GALVANTULA: Pokemon = Pokemon {
    name: "Galvantula",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 70,
        attack: 77,
        defense: 60,
        special_attack: 97,
        special_defense: 60,
        speed: 108,
    },
};
pub const GARBODOR: Pokemon = Pokemon {
    name: "Garbodor",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 95,
        defense: 82,
        special_attack: 60,
        special_defense: 82,
        speed: 75,
    },
};
pub const GARCHOMP: Pokemon = Pokemon {
    name: "Garchomp",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 108,
        attack: 130,
        defense: 95,
        special_attack: 80,
        special_defense: 85,
        speed: 102,
    },
};
pub const GARDEVOIR: Pokemon = Pokemon {
    name: "Gardevoir",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 68,
        attack: 65,
        defense: 65,
        special_attack: 125,
        special_defense: 115,
        speed: 80,
    },
};
pub const GASTLY: Pokemon = Pokemon {
    name: "Gastly",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 30,
        attack: 35,
        defense: 30,
        special_attack: 100,
        special_defense: 35,
        speed: 80,
    },
};
pub const GASTRODON: Pokemon = Pokemon {
    name: "Gastrodon",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 111,
        attack: 83,
        defense: 68,
        special_attack: 92,
        special_defense: 82,
        speed: 39,
    },
};
pub const GENESECT: Pokemon = Pokemon {
    name: "Genesect",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 71,
        attack: 120,
        defense: 95,
        special_attack: 120,
        special_defense: 95,
        speed: 99,
    },
};
pub const GENGAR: Pokemon = Pokemon {
    name: "Gengar",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 60,
        attack: 65,
        defense: 60,
        special_attack: 130,
        special_defense: 75,
        speed: 110,
    },
};
pub const GEODUDE: Pokemon = Pokemon {
    name: "Geodude",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 40,
        attack: 80,
        defense: 100,
        special_attack: 30,
        special_defense: 30,
        speed: 20,
    },
};
pub const GIBLE: Pokemon = Pokemon {
    name: "Gible",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 58,
        attack: 70,
        defense: 45,
        special_attack: 40,
        special_defense: 45,
        speed: 42,
    },
};
pub const GIGALITH: Pokemon = Pokemon {
    name: "Gigalith",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 135,
        defense: 130,
        special_attack: 60,
        special_defense: 80,
        speed: 25,
    },
};
pub const GIRAFARIG: Pokemon = Pokemon {
    name: "Girafarig",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 70,
        attack: 80,
        defense: 65,
        special_attack: 90,
        special_defense: 65,
        speed: 85,
    },
};
pub const GIRATINA: Pokemon = Pokemon {
    name: "Giratina",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 150,
        attack: 100,
        defense: 120,
        special_attack: 100,
        special_defense: 120,
        speed: 90,
    },
};
pub const GLACEON: Pokemon = Pokemon {
    name: "Glaceon",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 60,
        defense: 110,
        special_attack: 130,
        special_defense: 95,
        speed: 65,
    },
};
pub const GLALIE: Pokemon = Pokemon {
    name: "Glalie",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 80,
        defense: 80,
        special_attack: 80,
        special_defense: 80,
        speed: 80,
    },
};
pub const GLAMEOW: Pokemon = Pokemon {
    name: "Glameow",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 49,
        attack: 55,
        defense: 42,
        special_attack: 42,
        special_defense: 37,
        speed: 85,
    },
};
pub const GLASTRIER: Pokemon = Pokemon {
    name: "Glastrier",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 145,
        defense: 130,
        special_attack: 65,
        special_defense: 110,
        speed: 30,
    },
};
pub const GLIGAR: Pokemon = Pokemon {
    name: "Gligar",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 65,
        attack: 75,
        defense: 105,
        special_attack: 35,
        special_defense: 65,
        speed: 85,
    },
};
pub const GLISCOR: Pokemon = Pokemon {
    name: "Gliscor",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 75,
        attack: 95,
        defense: 125,
        special_attack: 45,
        special_defense: 75,
        speed: 95,
    },
};
pub const GLOOM: Pokemon = Pokemon {
    name: "Gloom",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 60,
        attack: 65,
        defense: 70,
        special_attack: 85,
        special_defense: 75,
        speed: 40,
    },
};
pub const GOGOAT: Pokemon = Pokemon {
    name: "Gogoat",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 123,
        attack: 100,
        defense: 62,
        special_attack: 97,
        special_defense: 81,
        speed: 68,
    },
};
pub const GOLBAT: Pokemon = Pokemon {
    name: "Golbat",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 75,
        attack: 80,
        defense: 70,
        special_attack: 65,
        special_defense: 75,
        speed: 90,
    },
};
pub const GOLDEEN: Pokemon = Pokemon {
    name: "Goldeen",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 67,
        defense: 60,
        special_attack: 35,
        special_defense: 50,
        speed: 63,
    },
};
pub const GOLDUCK: Pokemon = Pokemon {
    name: "Golduck",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 82,
        defense: 78,
        special_attack: 95,
        special_defense: 80,
        speed: 85,
    },
};
pub const GOLEM: Pokemon = Pokemon {
    name: "Golem",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 80,
        attack: 120,
        defense: 130,
        special_attack: 55,
        special_defense: 65,
        speed: 45,
    },
};
pub const GOLETT: Pokemon = Pokemon {
    name: "Golett",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 59,
        attack: 74,
        defense: 50,
        special_attack: 35,
        special_defense: 50,
        speed: 35,
    },
};
pub const GOLISOPOD: Pokemon = Pokemon {
    name: "Golisopod",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 75,
        attack: 125,
        defense: 140,
        special_attack: 60,
        special_defense: 90,
        speed: 40,
    },
};
pub const GOLURK: Pokemon = Pokemon {
    name: "Golurk",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 89,
        attack: 124,
        defense: 80,
        special_attack: 55,
        special_defense: 80,
        speed: 55,
    },
};
pub const GOODRA: Pokemon = Pokemon {
    name: "Goodra",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 100,
        defense: 70,
        special_attack: 110,
        special_defense: 150,
        speed: 80,
    },
};
pub const GOOMY: Pokemon = Pokemon {
    name: "Goomy",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 50,
        defense: 35,
        special_attack: 55,
        special_defense: 75,
        speed: 40,
    },
};
pub const GOREBYSS: Pokemon = Pokemon {
    name: "Gorebyss",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 84,
        defense: 105,
        special_attack: 114,
        special_defense: 75,
        speed: 52,
    },
};
pub const GOSSIFLEUR: Pokemon = Pokemon {
    name: "Gossifleur",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 40,
        defense: 60,
        special_attack: 40,
        special_defense: 60,
        speed: 10,
    },
};
pub const GOTHITA: Pokemon = Pokemon {
    name: "Gothita",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 30,
        defense: 50,
        special_attack: 55,
        special_defense: 65,
        speed: 45,
    },
};
pub const GOTHITELLE: Pokemon = Pokemon {
    name: "Gothitelle",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 55,
        defense: 95,
        special_attack: 95,
        special_defense: 110,
        speed: 65,
    },
};
pub const GOTHORITA: Pokemon = Pokemon {
    name: "Gothorita",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 45,
        defense: 70,
        special_attack: 75,
        special_defense: 85,
        speed: 55,
    },
};
pub const GOURGEIST: Pokemon = Pokemon {
    name: "Gourgeist",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 65,
        attack: 90,
        defense: 122,
        special_attack: 58,
        special_defense: 75,
        speed: 84,
    },
};
pub const GRANBULL: Pokemon = Pokemon {
    name: "Granbull",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 120,
        defense: 75,
        special_attack: 60,
        special_defense: 60,
        speed: 45,
    },
};
pub const GRANBULL_GEN2: Pokemon = Pokemon {
    name: "Granbull",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 120,
        defense: 75,
        special_attack: 60,
        special_defense: 60,
        speed: 45,
    },
};
pub const GRAPPLOCT: Pokemon = Pokemon {
    name: "Grapploct",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 118,
        defense: 90,
        special_attack: 70,
        special_defense: 80,
        speed: 42,
    },
};
pub const GRAVELER: Pokemon = Pokemon {
    name: "Graveler",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 55,
        attack: 95,
        defense: 115,
        special_attack: 45,
        special_defense: 45,
        speed: 35,
    },
};
pub const GREEDENT: Pokemon = Pokemon {
    name: "Greedent",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 120,
        attack: 95,
        defense: 95,
        special_attack: 55,
        special_defense: 75,
        speed: 20,
    },
};
pub const GRENINJA: Pokemon = Pokemon {
    name: "Greninja",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 72,
        attack: 95,
        defense: 67,
        special_attack: 103,
        special_defense: 71,
        speed: 122,
    },
};
pub const GRIMER: Pokemon = Pokemon {
    name: "Grimer",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 80,
        defense: 50,
        special_attack: 40,
        special_defense: 50,
        speed: 25,
    },
};
pub const GRIMMSNARL: Pokemon = Pokemon {
    name: "Grimmsnarl",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 95,
        attack: 120,
        defense: 65,
        special_attack: 95,
        special_defense: 75,
        speed: 60,
    },
};
pub const GROOKEY: Pokemon = Pokemon {
    name: "Grookey",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 65,
        defense: 50,
        special_attack: 40,
        special_defense: 40,
        speed: 65,
    },
};
pub const GROTLE: Pokemon = Pokemon {
    name: "Grotle",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 89,
        defense: 85,
        special_attack: 55,
        special_defense: 65,
        speed: 36,
    },
};
pub const GROUDON: Pokemon = Pokemon {
    name: "Groudon",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 150,
        defense: 140,
        special_attack: 100,
        special_defense: 90,
        speed: 90,
    },
};
pub const GROVYLE: Pokemon = Pokemon {
    name: "Grovyle",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 65,
        defense: 45,
        special_attack: 85,
        special_defense: 65,
        speed: 95,
    },
};
pub const GROWLITHE: Pokemon = Pokemon {
    name: "Growlithe",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 70,
        defense: 45,
        special_attack: 70,
        special_defense: 50,
        speed: 60,
    },
};
pub const GRUBBIN: Pokemon = Pokemon {
    name: "Grubbin",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 47,
        attack: 62,
        defense: 45,
        special_attack: 55,
        special_defense: 45,
        speed: 46,
    },
};
pub const GRUMPIG: Pokemon = Pokemon {
    name: "Grumpig",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 45,
        defense: 65,
        special_attack: 90,
        special_defense: 110,
        speed: 80,
    },
};
pub const GULPIN: Pokemon = Pokemon {
    name: "Gulpin",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 43,
        defense: 53,
        special_attack: 43,
        special_defense: 53,
        speed: 40,
    },
};
pub const GUMSHOOS: Pokemon = Pokemon {
    name: "Gumshoos",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 88,
        attack: 110,
        defense: 60,
        special_attack: 55,
        special_defense: 60,
        speed: 45,
    },
};
pub const GURDURR: Pokemon = Pokemon {
    name: "Gurdurr",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 105,
        defense: 85,
        special_attack: 40,
        special_defense: 50,
        speed: 40,
    },
};
pub const GUZZLORD: Pokemon = Pokemon {
    name: "Guzzlord",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 223,
        attack: 101,
        defense: 53,
        special_attack: 97,
        special_defense: 53,
        speed: 43,
    },
};
pub const GYARADOS: Pokemon = Pokemon {
    name: "Gyarados",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 95,
        attack: 125,
        defense: 79,
        special_attack: 60,
        special_defense: 100,
        speed: 81,
    },
};
pub const HAKAMO_O: Pokemon = Pokemon {
    name: "Hakamo-o",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 55,
        attack: 75,
        defense: 90,
        special_attack: 65,
        special_defense: 70,
        speed: 65,
    },
};
pub const HAPPINY: Pokemon = Pokemon {
    name: "Happiny",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 5,
        defense: 5,
        special_attack: 15,
        special_defense: 65,
        speed: 30,
    },
};
pub const HARIYAMA: Pokemon = Pokemon {
    name: "Hariyama",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 144,
        attack: 120,
        defense: 60,
        special_attack: 40,
        special_defense: 60,
        speed: 50,
    },
};
pub const HATENNA: Pokemon = Pokemon {
    name: "Hatenna",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 42,
        attack: 30,
        defense: 45,
        special_attack: 56,
        special_defense: 53,
        speed: 39,
    },
};
pub const HATTERENE: Pokemon = Pokemon {
    name: "Hatterene",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 57,
        attack: 90,
        defense: 95,
        special_attack: 136,
        special_defense: 103,
        speed: 29,
    },
};
pub const HATTREM: Pokemon = Pokemon {
    name: "Hattrem",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 57,
        attack: 40,
        defense: 65,
        special_attack: 86,
        special_defense: 73,
        speed: 49,
    },
};
pub const HAUNTER: Pokemon = Pokemon {
    name: "Haunter",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 45,
        attack: 50,
        defense: 45,
        special_attack: 115,
        special_defense: 55,
        speed: 95,
    },
};
pub const HAWLUCHA: Pokemon = Pokemon {
    name: "Hawlucha",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 78,
        attack: 92,
        defense: 75,
        special_attack: 74,
        special_defense: 63,
        speed: 118,
    },
};
pub const HAXORUS: Pokemon = Pokemon {
    name: "Haxorus",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 76,
        attack: 147,
        defense: 90,
        special_attack: 60,
        special_defense: 70,
        speed: 97,
    },
};
pub const HEATMOR: Pokemon = Pokemon {
    name: "Heatmor",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 97,
        defense: 66,
        special_attack: 105,
        special_defense: 66,
        speed: 65,
    },
};
pub const HEATRAN: Pokemon = Pokemon {
    name: "Heatran",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 91,
        attack: 90,
        defense: 106,
        special_attack: 130,
        special_defense: 106,
        speed: 77,
    },
};
pub const HELIOLISK: Pokemon = Pokemon {
    name: "Heliolisk",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Normal),
    },
    stats: Stats {
        hp: 62,
        attack: 55,
        defense: 52,
        special_attack: 109,
        special_defense: 94,
        speed: 109,
    },
};
pub const HELIOPTILE: Pokemon = Pokemon {
    name: "Helioptile",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Normal),
    },
    stats: Stats {
        hp: 44,
        attack: 38,
        defense: 33,
        special_attack: 61,
        special_defense: 43,
        speed: 70,
    },
};
pub const HERACROSS: Pokemon = Pokemon {
    name: "Heracross",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 80,
        attack: 125,
        defense: 75,
        special_attack: 40,
        special_defense: 95,
        speed: 85,
    },
};
pub const HERDIER: Pokemon = Pokemon {
    name: "Herdier",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 80,
        defense: 65,
        special_attack: 35,
        special_defense: 65,
        speed: 60,
    },
};
pub const HIPPOPOTAS: Pokemon = Pokemon {
    name: "Hippopotas",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 68,
        attack: 72,
        defense: 78,
        special_attack: 38,
        special_defense: 42,
        speed: 32,
    },
};
pub const HIPPOWDON: Pokemon = Pokemon {
    name: "Hippowdon",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 108,
        attack: 112,
        defense: 118,
        special_attack: 68,
        special_defense: 72,
        speed: 47,
    },
};
pub const HITMONCHAN: Pokemon = Pokemon {
    name: "Hitmonchan",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 105,
        defense: 79,
        special_attack: 35,
        special_defense: 110,
        speed: 76,
    },
};
pub const HITMONLEE: Pokemon = Pokemon {
    name: "Hitmonlee",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 120,
        defense: 53,
        special_attack: 35,
        special_defense: 110,
        speed: 87,
    },
};
pub const HITMONTOP: Pokemon = Pokemon {
    name: "Hitmontop",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 95,
        defense: 95,
        special_attack: 35,
        special_defense: 110,
        speed: 70,
    },
};
pub const HO_OH: Pokemon = Pokemon {
    name: "Ho-oh",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 106,
        attack: 130,
        defense: 90,
        special_attack: 110,
        special_defense: 154,
        speed: 90,
    },
};
pub const HONCHKROW: Pokemon = Pokemon {
    name: "Honchkrow",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 100,
        attack: 125,
        defense: 52,
        special_attack: 105,
        special_defense: 52,
        speed: 71,
    },
};
pub const HONEDGE: Pokemon = Pokemon {
    name: "Honedge",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 45,
        attack: 80,
        defense: 100,
        special_attack: 35,
        special_defense: 37,
        speed: 28,
    },
};
pub const HOOPA: Pokemon = Pokemon {
    name: "Hoopa",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 80,
        attack: 110,
        defense: 60,
        special_attack: 150,
        special_defense: 130,
        speed: 70,
    },
};
pub const HOOTHOOT: Pokemon = Pokemon {
    name: "Hoothoot",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 30,
        defense: 30,
        special_attack: 36,
        special_defense: 56,
        speed: 50,
    },
};
pub const HOPPIP: Pokemon = Pokemon {
    name: "Hoppip",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 35,
        attack: 35,
        defense: 40,
        special_attack: 35,
        special_defense: 55,
        speed: 50,
    },
};
pub const HORSEA: Pokemon = Pokemon {
    name: "Horsea",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 40,
        defense: 70,
        special_attack: 70,
        special_defense: 25,
        speed: 60,
    },
};
pub const HOUNDOOM: Pokemon = Pokemon {
    name: "Houndoom",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 75,
        attack: 90,
        defense: 50,
        special_attack: 110,
        special_defense: 80,
        speed: 95,
    },
};
pub const HOUNDOUR: Pokemon = Pokemon {
    name: "Houndour",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 45,
        attack: 60,
        defense: 30,
        special_attack: 80,
        special_defense: 50,
        speed: 65,
    },
};
pub const HUNTAIL: Pokemon = Pokemon {
    name: "Huntail",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 104,
        defense: 105,
        special_attack: 94,
        special_defense: 75,
        speed: 52,
    },
};
pub const HYDREIGON: Pokemon = Pokemon {
    name: "Hydreigon",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 92,
        attack: 105,
        defense: 90,
        special_attack: 125,
        special_defense: 90,
        speed: 98,
    },
};
pub const HYPNO: Pokemon = Pokemon {
    name: "Hypno",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 73,
        defense: 70,
        special_attack: 73,
        special_defense: 115,
        speed: 67,
    },
};
pub const IGGLYBUFF: Pokemon = Pokemon {
    name: "Igglybuff",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 90,
        attack: 30,
        defense: 15,
        special_attack: 40,
        special_defense: 20,
        speed: 15,
    },
};
pub const IGGLYBUFF_GEN2: Pokemon = Pokemon {
    name: "Igglybuff",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 30,
        defense: 15,
        special_attack: 40,
        special_defense: 20,
        speed: 15,
    },
};
pub const ILLUMISE: Pokemon = Pokemon {
    name: "Illumise",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 47,
        defense: 75,
        special_attack: 73,
        special_defense: 85,
        speed: 85,
    },
};
pub const IMPIDIMP: Pokemon = Pokemon {
    name: "Impidimp",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 45,
        attack: 45,
        defense: 30,
        special_attack: 55,
        special_defense: 40,
        speed: 50,
    },
};
pub const INCINEROAR: Pokemon = Pokemon {
    name: "Incineroar",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 95,
        attack: 115,
        defense: 90,
        special_attack: 80,
        special_defense: 90,
        speed: 60,
    },
};
pub const INDEEDEE: Pokemon = Pokemon {
    name: "Indeedee",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Normal),
    },
    stats: Stats {
        hp: 60,
        attack: 65,
        defense: 55,
        special_attack: 105,
        special_defense: 95,
        speed: 95,
    },
};
pub const INFERNAPE: Pokemon = Pokemon {
    name: "Infernape",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 76,
        attack: 104,
        defense: 71,
        special_attack: 104,
        special_defense: 71,
        speed: 108,
    },
};
pub const INKAY: Pokemon = Pokemon {
    name: "Inkay",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 53,
        attack: 54,
        defense: 53,
        special_attack: 37,
        special_defense: 46,
        speed: 45,
    },
};
pub const INTELEON: Pokemon = Pokemon {
    name: "Inteleon",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 85,
        defense: 65,
        special_attack: 125,
        special_defense: 65,
        speed: 120,
    },
};
pub const IVYSAUR: Pokemon = Pokemon {
    name: "Ivysaur",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 60,
        attack: 62,
        defense: 63,
        special_attack: 80,
        special_defense: 80,
        speed: 60,
    },
};
pub const JANGMO_O: Pokemon = Pokemon {
    name: "Jangmo-o",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 55,
        defense: 65,
        special_attack: 45,
        special_defense: 45,
        speed: 45,
    },
};
pub const JELLICENT: Pokemon = Pokemon {
    name: "Jellicent",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 100,
        attack: 60,
        defense: 70,
        special_attack: 85,
        special_defense: 105,
        speed: 60,
    },
};
pub const JIGGLYPUFF: Pokemon = Pokemon {
    name: "Jigglypuff",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 115,
        attack: 45,
        defense: 20,
        special_attack: 45,
        special_defense: 25,
        speed: 20,
    },
};
pub const JIGGLYPUFF_GEN1: Pokemon = Pokemon {
    name: "Jigglypuff",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 115,
        attack: 45,
        defense: 20,
        special_attack: 45,
        special_defense: 25,
        speed: 20,
    },
};
pub const JIRACHI: Pokemon = Pokemon {
    name: "Jirachi",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
    },
};
pub const JOLTEON: Pokemon = Pokemon {
    name: "Jolteon",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 65,
        defense: 60,
        special_attack: 110,
        special_defense: 95,
        speed: 130,
    },
};
pub const JOLTIK: Pokemon = Pokemon {
    name: "Joltik",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 50,
        attack: 47,
        defense: 50,
        special_attack: 57,
        special_defense: 50,
        speed: 65,
    },
};
pub const JUMPLUFF: Pokemon = Pokemon {
    name: "Jumpluff",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 75,
        attack: 55,
        defense: 70,
        special_attack: 55,
        special_defense: 95,
        speed: 110,
    },
};
pub const JYNX: Pokemon = Pokemon {
    name: "Jynx",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 65,
        attack: 50,
        defense: 35,
        special_attack: 115,
        special_defense: 95,
        speed: 95,
    },
};
pub const KABUTO: Pokemon = Pokemon {
    name: "Kabuto",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 30,
        attack: 80,
        defense: 90,
        special_attack: 55,
        special_defense: 45,
        speed: 55,
    },
};
pub const KABUTOPS: Pokemon = Pokemon {
    name: "Kabutops",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 60,
        attack: 115,
        defense: 105,
        special_attack: 65,
        special_defense: 70,
        speed: 80,
    },
};
pub const KADABRA: Pokemon = Pokemon {
    name: "Kadabra",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 35,
        defense: 30,
        special_attack: 120,
        special_defense: 70,
        speed: 105,
    },
};
pub const KAKUNA: Pokemon = Pokemon {
    name: "Kakuna",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 45,
        attack: 25,
        defense: 50,
        special_attack: 25,
        special_defense: 25,
        speed: 35,
    },
};
pub const KANGASKHAN: Pokemon = Pokemon {
    name: "Kangaskhan",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 105,
        attack: 95,
        defense: 80,
        special_attack: 40,
        special_defense: 80,
        speed: 90,
    },
};
pub const KARRABLAST: Pokemon = Pokemon {
    name: "Karrablast",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 75,
        defense: 45,
        special_attack: 40,
        special_defense: 45,
        speed: 60,
    },
};
pub const KARTANA: Pokemon = Pokemon {
    name: "Kartana",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 59,
        attack: 181,
        defense: 131,
        special_attack: 59,
        special_defense: 31,
        speed: 109,
    },
};
pub const KECLEON: Pokemon = Pokemon {
    name: "Kecleon",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 90,
        defense: 70,
        special_attack: 60,
        special_defense: 120,
        speed: 40,
    },
};
pub const KELDEO: Pokemon = Pokemon {
    name: "Keldeo",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 91,
        attack: 72,
        defense: 90,
        special_attack: 129,
        special_defense: 90,
        speed: 108,
    },
};
pub const KINGDRA: Pokemon = Pokemon {
    name: "Kingdra",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 75,
        attack: 95,
        defense: 95,
        special_attack: 95,
        special_defense: 95,
        speed: 85,
    },
};
pub const KINGLER: Pokemon = Pokemon {
    name: "Kingler",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 130,
        defense: 115,
        special_attack: 50,
        special_defense: 50,
        speed: 75,
    },
};
pub const KIRLIA: Pokemon = Pokemon {
    name: "Kirlia",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 38,
        attack: 35,
        defense: 35,
        special_attack: 65,
        special_defense: 55,
        speed: 50,
    },
};
pub const KLANG: Pokemon = Pokemon {
    name: "Klang",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 80,
        defense: 95,
        special_attack: 70,
        special_defense: 85,
        speed: 50,
    },
};
pub const KLEFKI: Pokemon = Pokemon {
    name: "Klefki",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 57,
        attack: 80,
        defense: 91,
        special_attack: 80,
        special_defense: 87,
        speed: 75,
    },
};
pub const KLINK: Pokemon = Pokemon {
    name: "Klink",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 55,
        defense: 70,
        special_attack: 45,
        special_defense: 60,
        speed: 30,
    },
};
pub const KLINKLANG: Pokemon = Pokemon {
    name: "Klinklang",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 100,
        defense: 115,
        special_attack: 70,
        special_defense: 85,
        speed: 90,
    },
};
pub const KOFFING: Pokemon = Pokemon {
    name: "Koffing",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 65,
        defense: 95,
        special_attack: 60,
        special_defense: 45,
        speed: 35,
    },
};
pub const KOMALA: Pokemon = Pokemon {
    name: "Komala",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 115,
        defense: 65,
        special_attack: 75,
        special_defense: 95,
        speed: 65,
    },
};
pub const KOMMO_O: Pokemon = Pokemon {
    name: "Kommo-o",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 75,
        attack: 110,
        defense: 125,
        special_attack: 100,
        special_defense: 105,
        speed: 85,
    },
};
pub const KRABBY: Pokemon = Pokemon {
    name: "Krabby",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 105,
        defense: 90,
        special_attack: 25,
        special_defense: 25,
        speed: 50,
    },
};
pub const KRICKETOT: Pokemon = Pokemon {
    name: "Kricketot",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 37,
        attack: 25,
        defense: 41,
        special_attack: 25,
        special_defense: 41,
        speed: 25,
    },
};
pub const KRICKETUNE: Pokemon = Pokemon {
    name: "Kricketune",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 77,
        attack: 85,
        defense: 51,
        special_attack: 55,
        special_defense: 51,
        speed: 65,
    },
};
pub const KROKOROK: Pokemon = Pokemon {
    name: "Krokorok",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 60,
        attack: 82,
        defense: 45,
        special_attack: 45,
        special_defense: 45,
        speed: 74,
    },
};
pub const KROOKODILE: Pokemon = Pokemon {
    name: "Krookodile",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 95,
        attack: 117,
        defense: 80,
        special_attack: 65,
        special_defense: 70,
        speed: 92,
    },
};
pub const KUBFU: Pokemon = Pokemon {
    name: "Kubfu",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 90,
        defense: 60,
        special_attack: 53,
        special_defense: 50,
        speed: 72,
    },
};
pub const KYOGRE: Pokemon = Pokemon {
    name: "Kyogre",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 90,
        special_attack: 150,
        special_defense: 140,
        speed: 90,
    },
};
pub const KYUREM: Pokemon = Pokemon {
    name: "Kyurem",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 125,
        attack: 130,
        defense: 90,
        special_attack: 130,
        special_defense: 90,
        speed: 95,
    },
};
pub const LAIRON: Pokemon = Pokemon {
    name: "Lairon",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 60,
        attack: 90,
        defense: 140,
        special_attack: 50,
        special_defense: 50,
        speed: 40,
    },
};
pub const LAMPENT: Pokemon = Pokemon {
    name: "Lampent",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 60,
        attack: 40,
        defense: 60,
        special_attack: 95,
        special_defense: 60,
        speed: 55,
    },
};
pub const LANDORUS: Pokemon = Pokemon {
    name: "Landorus",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 89,
        attack: 125,
        defense: 90,
        special_attack: 115,
        special_defense: 80,
        speed: 101,
    },
};
pub const LANTURN: Pokemon = Pokemon {
    name: "Lanturn",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 125,
        attack: 58,
        defense: 58,
        special_attack: 76,
        special_defense: 76,
        speed: 67,
    },
};
pub const LAPRAS: Pokemon = Pokemon {
    name: "Lapras",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 130,
        attack: 85,
        defense: 80,
        special_attack: 85,
        special_defense: 95,
        speed: 60,
    },
};
pub const LARVESTA: Pokemon = Pokemon {
    name: "Larvesta",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 55,
        attack: 85,
        defense: 55,
        special_attack: 50,
        special_defense: 55,
        speed: 60,
    },
};
pub const LARVITAR: Pokemon = Pokemon {
    name: "Larvitar",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 50,
        attack: 64,
        defense: 50,
        special_attack: 45,
        special_defense: 50,
        speed: 41,
    },
};
pub const LATIAS: Pokemon = Pokemon {
    name: "Latias",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 80,
        attack: 80,
        defense: 90,
        special_attack: 110,
        special_defense: 130,
        speed: 110,
    },
};
pub const LATIOS: Pokemon = Pokemon {
    name: "Latios",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 80,
        attack: 90,
        defense: 80,
        special_attack: 130,
        special_defense: 110,
        speed: 110,
    },
};
pub const LEAFEON: Pokemon = Pokemon {
    name: "Leafeon",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 110,
        defense: 130,
        special_attack: 60,
        special_defense: 65,
        speed: 95,
    },
};
pub const LEAVANNY: Pokemon = Pokemon {
    name: "Leavanny",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 75,
        attack: 103,
        defense: 80,
        special_attack: 70,
        special_defense: 80,
        speed: 92,
    },
};
pub const LEDIAN: Pokemon = Pokemon {
    name: "Ledian",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 35,
        defense: 50,
        special_attack: 55,
        special_defense: 110,
        speed: 85,
    },
};
pub const LEDYBA: Pokemon = Pokemon {
    name: "Ledyba",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 20,
        defense: 30,
        special_attack: 40,
        special_defense: 80,
        speed: 55,
    },
};
pub const LICKILICKY: Pokemon = Pokemon {
    name: "Lickilicky",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 110,
        attack: 85,
        defense: 95,
        special_attack: 80,
        special_defense: 95,
        speed: 50,
    },
};
pub const LICKITUNG: Pokemon = Pokemon {
    name: "Lickitung",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 55,
        defense: 75,
        special_attack: 60,
        special_defense: 75,
        speed: 30,
    },
};
pub const LIEPARD: Pokemon = Pokemon {
    name: "Liepard",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 64,
        attack: 88,
        defense: 50,
        special_attack: 88,
        special_defense: 50,
        speed: 106,
    },
};
pub const LILEEP: Pokemon = Pokemon {
    name: "Lileep",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 66,
        attack: 41,
        defense: 77,
        special_attack: 61,
        special_defense: 87,
        speed: 23,
    },
};
pub const LILLIGANT: Pokemon = Pokemon {
    name: "Lilligant",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 60,
        defense: 75,
        special_attack: 110,
        special_defense: 75,
        speed: 90,
    },
};
pub const LILLIPUP: Pokemon = Pokemon {
    name: "Lillipup",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 60,
        defense: 45,
        special_attack: 25,
        special_defense: 45,
        speed: 55,
    },
};
pub const LINOONE: Pokemon = Pokemon {
    name: "Linoone",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 78,
        attack: 70,
        defense: 61,
        special_attack: 50,
        special_defense: 61,
        speed: 100,
    },
};
pub const LITLEO: Pokemon = Pokemon {
    name: "Litleo",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Normal),
    },
    stats: Stats {
        hp: 62,
        attack: 50,
        defense: 58,
        special_attack: 73,
        special_defense: 54,
        speed: 72,
    },
};
pub const LITTEN: Pokemon = Pokemon {
    name: "Litten",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 65,
        defense: 40,
        special_attack: 60,
        special_defense: 40,
        speed: 70,
    },
};
pub const LITWICK: Pokemon = Pokemon {
    name: "Litwick",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 50,
        attack: 30,
        defense: 55,
        special_attack: 65,
        special_defense: 55,
        speed: 20,
    },
};
pub const LOMBRE: Pokemon = Pokemon {
    name: "Lombre",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 50,
        special_attack: 60,
        special_defense: 70,
        speed: 50,
    },
};
pub const LOPUNNY: Pokemon = Pokemon {
    name: "Lopunny",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 76,
        defense: 84,
        special_attack: 54,
        special_defense: 96,
        speed: 105,
    },
};
pub const LOTAD: Pokemon = Pokemon {
    name: "Lotad",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 40,
        attack: 30,
        defense: 30,
        special_attack: 40,
        special_defense: 50,
        speed: 30,
    },
};
pub const LOUDRED: Pokemon = Pokemon {
    name: "Loudred",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 84,
        attack: 71,
        defense: 43,
        special_attack: 71,
        special_defense: 43,
        speed: 48,
    },
};
pub const LUCARIO: Pokemon = Pokemon {
    name: "Lucario",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 70,
        attack: 110,
        defense: 70,
        special_attack: 115,
        special_defense: 70,
        speed: 90,
    },
};
pub const LUDICOLO: Pokemon = Pokemon {
    name: "Ludicolo",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 80,
        attack: 70,
        defense: 70,
        special_attack: 90,
        special_defense: 100,
        speed: 70,
    },
};
pub const LUGIA: Pokemon = Pokemon {
    name: "Lugia",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 106,
        attack: 90,
        defense: 130,
        special_attack: 90,
        special_defense: 154,
        speed: 110,
    },
};
pub const LUMINEON: Pokemon = Pokemon {
    name: "Lumineon",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 69,
        attack: 69,
        defense: 76,
        special_attack: 69,
        special_defense: 86,
        speed: 91,
    },
};
pub const LUNALA: Pokemon = Pokemon {
    name: "Lunala",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 137,
        attack: 113,
        defense: 89,
        special_attack: 137,
        special_defense: 107,
        speed: 97,
    },
};
pub const LUNATONE: Pokemon = Pokemon {
    name: "Lunatone",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 90,
        attack: 55,
        defense: 65,
        special_attack: 95,
        special_defense: 85,
        speed: 70,
    },
};
pub const LURANTIS: Pokemon = Pokemon {
    name: "Lurantis",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 105,
        defense: 90,
        special_attack: 80,
        special_defense: 90,
        speed: 45,
    },
};
pub const LUVDISC: Pokemon = Pokemon {
    name: "Luvdisc",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 43,
        attack: 30,
        defense: 55,
        special_attack: 40,
        special_defense: 65,
        speed: 97,
    },
};
pub const LUXIO: Pokemon = Pokemon {
    name: "Luxio",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 85,
        defense: 49,
        special_attack: 60,
        special_defense: 49,
        speed: 60,
    },
};
pub const LUXRAY: Pokemon = Pokemon {
    name: "Luxray",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 120,
        defense: 79,
        special_attack: 95,
        special_defense: 79,
        speed: 70,
    },
};
pub const LYCANROC: Pokemon = Pokemon {
    name: "Lycanroc",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 115,
        defense: 65,
        special_attack: 55,
        special_defense: 65,
        speed: 112,
    },
};
pub const MACHAMP: Pokemon = Pokemon {
    name: "Machamp",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 130,
        defense: 80,
        special_attack: 65,
        special_defense: 85,
        speed: 55,
    },
};
pub const MACHOKE: Pokemon = Pokemon {
    name: "Machoke",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 100,
        defense: 70,
        special_attack: 50,
        special_defense: 60,
        speed: 45,
    },
};
pub const MACHOP: Pokemon = Pokemon {
    name: "Machop",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 80,
        defense: 50,
        special_attack: 35,
        special_defense: 35,
        speed: 35,
    },
};
pub const MAGBY: Pokemon = Pokemon {
    name: "Magby",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 75,
        defense: 37,
        special_attack: 70,
        special_defense: 55,
        speed: 83,
    },
};
pub const MAGCARGO: Pokemon = Pokemon {
    name: "Magcargo",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 120,
        special_attack: 90,
        special_defense: 80,
        speed: 30,
    },
};
pub const MAGEARNA: Pokemon = Pokemon {
    name: "Magearna",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 80,
        attack: 95,
        defense: 115,
        special_attack: 130,
        special_defense: 115,
        speed: 65,
    },
};
pub const MAGIKARP: Pokemon = Pokemon {
    name: "Magikarp",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 20,
        attack: 10,
        defense: 55,
        special_attack: 15,
        special_defense: 20,
        speed: 80,
    },
};
pub const MAGMAR: Pokemon = Pokemon {
    name: "Magmar",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 95,
        defense: 57,
        special_attack: 100,
        special_defense: 85,
        speed: 93,
    },
};
pub const MAGMORTAR: Pokemon = Pokemon {
    name: "Magmortar",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 95,
        defense: 67,
        special_attack: 125,
        special_defense: 95,
        speed: 83,
    },
};
pub const MAGNEMITE: Pokemon = Pokemon {
    name: "Magnemite",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 25,
        attack: 35,
        defense: 70,
        special_attack: 95,
        special_defense: 55,
        speed: 45,
    },
};
pub const MAGNEMITE_GEN1: Pokemon = Pokemon {
    name: "Magnemite",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 25,
        attack: 35,
        defense: 70,
        special_attack: 95,
        special_defense: 55,
        speed: 45,
    },
};
pub const MAGNETON: Pokemon = Pokemon {
    name: "Magneton",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 50,
        attack: 60,
        defense: 95,
        special_attack: 120,
        special_defense: 70,
        speed: 70,
    },
};
pub const MAGNETON_GEN1: Pokemon = Pokemon {
    name: "Magneton",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 60,
        defense: 95,
        special_attack: 120,
        special_defense: 70,
        speed: 70,
    },
};
pub const MAGNEZONE: Pokemon = Pokemon {
    name: "Magnezone",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 70,
        attack: 70,
        defense: 115,
        special_attack: 130,
        special_defense: 90,
        speed: 60,
    },
};
pub const MAKUHITA: Pokemon = Pokemon {
    name: "Makuhita",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 72,
        attack: 60,
        defense: 30,
        special_attack: 20,
        special_defense: 30,
        speed: 25,
    },
};
pub const MALAMAR: Pokemon = Pokemon {
    name: "Malamar",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 86,
        attack: 92,
        defense: 88,
        special_attack: 68,
        special_defense: 75,
        speed: 73,
    },
};
pub const MAMOSWINE: Pokemon = Pokemon {
    name: "Mamoswine",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 110,
        attack: 130,
        defense: 80,
        special_attack: 70,
        special_defense: 60,
        speed: 80,
    },
};
pub const MANAPHY: Pokemon = Pokemon {
    name: "Manaphy",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
    },
};
pub const MANDIBUZZ: Pokemon = Pokemon {
    name: "Mandibuzz",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 110,
        attack: 65,
        defense: 105,
        special_attack: 55,
        special_defense: 95,
        speed: 80,
    },
};
pub const MANECTRIC: Pokemon = Pokemon {
    name: "Manectric",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 75,
        defense: 60,
        special_attack: 105,
        special_defense: 60,
        speed: 105,
    },
};
pub const MANKEY: Pokemon = Pokemon {
    name: "Mankey",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 80,
        defense: 35,
        special_attack: 35,
        special_defense: 45,
        speed: 70,
    },
};
pub const MANTINE: Pokemon = Pokemon {
    name: "Mantine",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 85,
        attack: 40,
        defense: 70,
        special_attack: 80,
        special_defense: 140,
        speed: 70,
    },
};
pub const MANTYKE: Pokemon = Pokemon {
    name: "Mantyke",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 45,
        attack: 20,
        defense: 50,
        special_attack: 60,
        special_defense: 120,
        speed: 50,
    },
};
pub const MARACTUS: Pokemon = Pokemon {
    name: "Maractus",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 86,
        defense: 67,
        special_attack: 106,
        special_defense: 67,
        speed: 60,
    },
};
pub const MAREANIE: Pokemon = Pokemon {
    name: "Mareanie",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 50,
        attack: 53,
        defense: 62,
        special_attack: 43,
        special_defense: 52,
        speed: 45,
    },
};
pub const MAREEP: Pokemon = Pokemon {
    name: "Mareep",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 40,
        defense: 40,
        special_attack: 65,
        special_defense: 45,
        speed: 35,
    },
};
pub const MARILL: Pokemon = Pokemon {
    name: "Marill",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 70,
        attack: 20,
        defense: 50,
        special_attack: 20,
        special_defense: 50,
        speed: 40,
    },
};
pub const MARILL_GEN2: Pokemon = Pokemon {
    name: "Marill",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 20,
        defense: 50,
        special_attack: 20,
        special_defense: 50,
        speed: 40,
    },
};
pub const MAROWAK: Pokemon = Pokemon {
    name: "Marowak",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 80,
        defense: 110,
        special_attack: 50,
        special_defense: 80,
        speed: 45,
    },
};
pub const MARSHADOW: Pokemon = Pokemon {
    name: "Marshadow",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 90,
        attack: 125,
        defense: 80,
        special_attack: 90,
        special_defense: 90,
        speed: 125,
    },
};
pub const MARSHTOMP: Pokemon = Pokemon {
    name: "Marshtomp",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 70,
        attack: 85,
        defense: 70,
        special_attack: 60,
        special_defense: 70,
        speed: 50,
    },
};
pub const MASQUERAIN: Pokemon = Pokemon {
    name: "Masquerain",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 70,
        attack: 60,
        defense: 62,
        special_attack: 100,
        special_defense: 82,
        speed: 80,
    },
};
pub const MAWILE: Pokemon = Pokemon {
    name: "Mawile",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 50,
        attack: 85,
        defense: 85,
        special_attack: 55,
        special_defense: 55,
        speed: 50,
    },
};
pub const MEDICHAM: Pokemon = Pokemon {
    name: "Medicham",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 75,
        special_attack: 60,
        special_defense: 75,
        speed: 80,
    },
};
pub const MEDITITE: Pokemon = Pokemon {
    name: "Meditite",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 30,
        attack: 40,
        defense: 55,
        special_attack: 40,
        special_defense: 55,
        speed: 60,
    },
};
pub const MEGANIUM: Pokemon = Pokemon {
    name: "Meganium",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 82,
        defense: 100,
        special_attack: 83,
        special_defense: 100,
        speed: 80,
    },
};
pub const MELMETAL: Pokemon = Pokemon {
    name: "Melmetal",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 135,
        attack: 143,
        defense: 143,
        special_attack: 80,
        special_defense: 65,
        speed: 34,
    },
};
pub const MELOETTA: Pokemon = Pokemon {
    name: "Meloetta",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 100,
        attack: 77,
        defense: 77,
        special_attack: 128,
        special_defense: 128,
        speed: 90,
    },
};
pub const MELTAN: Pokemon = Pokemon {
    name: "Meltan",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 46,
        attack: 65,
        defense: 65,
        special_attack: 55,
        special_defense: 35,
        speed: 34,
    },
};
pub const MEOWSTIC: Pokemon = Pokemon {
    name: "Meowstic",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 74,
        attack: 48,
        defense: 76,
        special_attack: 83,
        special_defense: 81,
        speed: 104,
    },
};
pub const MEOWTH: Pokemon = Pokemon {
    name: "Meowth",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 35,
        special_attack: 40,
        special_defense: 40,
        speed: 90,
    },
};
pub const MESPRIT: Pokemon = Pokemon {
    name: "Mesprit",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 105,
        defense: 105,
        special_attack: 105,
        special_defense: 105,
        speed: 80,
    },
};
pub const METAGROSS: Pokemon = Pokemon {
    name: "Metagross",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 80,
        attack: 135,
        defense: 130,
        special_attack: 95,
        special_defense: 90,
        speed: 70,
    },
};
pub const METANG: Pokemon = Pokemon {
    name: "Metang",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 60,
        attack: 75,
        defense: 100,
        special_attack: 55,
        special_defense: 80,
        speed: 50,
    },
};
pub const METAPOD: Pokemon = Pokemon {
    name: "Metapod",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 20,
        defense: 55,
        special_attack: 25,
        special_defense: 25,
        speed: 30,
    },
};
pub const MEW: Pokemon = Pokemon {
    name: "Mew",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
    },
};
pub const MEWTWO: Pokemon = Pokemon {
    name: "Mewtwo",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 106,
        attack: 110,
        defense: 90,
        special_attack: 154,
        special_defense: 90,
        speed: 130,
    },
};
pub const MIENFOO: Pokemon = Pokemon {
    name: "Mienfoo",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 85,
        defense: 50,
        special_attack: 55,
        special_defense: 50,
        speed: 65,
    },
};
pub const MIENSHAO: Pokemon = Pokemon {
    name: "Mienshao",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 125,
        defense: 60,
        special_attack: 95,
        special_defense: 60,
        speed: 105,
    },
};
pub const MIGHTYENA: Pokemon = Pokemon {
    name: "Mightyena",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 90,
        defense: 70,
        special_attack: 60,
        special_defense: 60,
        speed: 70,
    },
};
pub const MILCERY: Pokemon = Pokemon {
    name: "Milcery",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 40,
        defense: 40,
        special_attack: 50,
        special_defense: 61,
        speed: 34,
    },
};
pub const MILOTIC: Pokemon = Pokemon {
    name: "Milotic",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 60,
        defense: 79,
        special_attack: 100,
        special_defense: 125,
        speed: 81,
    },
};
pub const MILTANK: Pokemon = Pokemon {
    name: "Miltank",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 80,
        defense: 105,
        special_attack: 40,
        special_defense: 70,
        speed: 100,
    },
};
pub const MIME_JR: Pokemon = Pokemon {
    name: "Mime Jr.",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 20,
        attack: 25,
        defense: 45,
        special_attack: 70,
        special_defense: 90,
        speed: 60,
    },
};
pub const MIME_JR_GEN2: Pokemon = Pokemon {
    name: "Mime Jr.",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 20,
        attack: 25,
        defense: 45,
        special_attack: 70,
        special_defense: 90,
        speed: 60,
    },
};
pub const MIMIKYU: Pokemon = Pokemon {
    name: "Mimikyu",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 55,
        attack: 90,
        defense: 80,
        special_attack: 50,
        special_defense: 105,
        speed: 96,
    },
};
pub const MINCCINO: Pokemon = Pokemon {
    name: "Minccino",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 50,
        defense: 40,
        special_attack: 40,
        special_defense: 40,
        speed: 75,
    },
};
pub const MINIOR: Pokemon = Pokemon {
    name: "Minior",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 100,
        special_attack: 60,
        special_defense: 100,
        speed: 60,
    },
};
pub const MINUN: Pokemon = Pokemon {
    name: "Minun",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 40,
        defense: 50,
        special_attack: 75,
        special_defense: 85,
        speed: 95,
    },
};
pub const MISDREAVUS: Pokemon = Pokemon {
    name: "Misdreavus",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 60,
        special_attack: 85,
        special_defense: 85,
        speed: 85,
    },
};
pub const MISMAGIUS: Pokemon = Pokemon {
    name: "Mismagius",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 60,
        special_attack: 105,
        special_defense: 105,
        speed: 105,
    },
};
pub const MOLTRES: Pokemon = Pokemon {
    name: "Moltres",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 90,
        attack: 100,
        defense: 90,
        special_attack: 125,
        special_defense: 85,
        speed: 90,
    },
};
pub const MONFERNO: Pokemon = Pokemon {
    name: "Monferno",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 64,
        attack: 78,
        defense: 52,
        special_attack: 78,
        special_defense: 52,
        speed: 81,
    },
};
pub const MORELULL: Pokemon = Pokemon {
    name: "Morelull",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 40,
        attack: 35,
        defense: 55,
        special_attack: 65,
        special_defense: 75,
        speed: 15,
    },
};
pub const MORGREM: Pokemon = Pokemon {
    name: "Morgrem",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 65,
        attack: 60,
        defense: 45,
        special_attack: 75,
        special_defense: 55,
        speed: 70,
    },
};
pub const MORPEKO: Pokemon = Pokemon {
    name: "Morpeko",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 58,
        attack: 95,
        defense: 58,
        special_attack: 70,
        special_defense: 58,
        speed: 97,
    },
};
pub const MOTHIM: Pokemon = Pokemon {
    name: "Mothim",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 70,
        attack: 94,
        defense: 50,
        special_attack: 94,
        special_defense: 50,
        speed: 66,
    },
};
pub const MR_MIME: Pokemon = Pokemon {
    name: "Mr. Mime",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 65,
        special_attack: 100,
        special_defense: 120,
        speed: 90,
    },
};
pub const MR_MIME_GEN1: Pokemon = Pokemon {
    name: "Mr. Mime",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 65,
        special_attack: 100,
        special_defense: 120,
        speed: 90,
    },
};
pub const MR_RIME: Pokemon = Pokemon {
    name: "Mr. Rime",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 80,
        attack: 85,
        defense: 75,
        special_attack: 110,
        special_defense: 100,
        speed: 70,
    },
};
pub const MUDBRAY: Pokemon = Pokemon {
    name: "Mudbray",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 100,
        defense: 70,
        special_attack: 45,
        special_defense: 55,
        speed: 45,
    },
};
pub const MUDKIP: Pokemon = Pokemon {
    name: "Mudkip",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 70,
        defense: 50,
        special_attack: 50,
        special_defense: 50,
        speed: 40,
    },
};
pub const MUDSDALE: Pokemon = Pokemon {
    name: "Mudsdale",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 125,
        defense: 100,
        special_attack: 55,
        special_defense: 85,
        speed: 35,
    },
};
pub const MUK: Pokemon = Pokemon {
    name: "Muk",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 105,
        attack: 105,
        defense: 75,
        special_attack: 65,
        special_defense: 100,
        speed: 50,
    },
};
pub const MUNCHLAX: Pokemon = Pokemon {
    name: "Munchlax",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 135,
        attack: 85,
        defense: 40,
        special_attack: 40,
        special_defense: 85,
        speed: 5,
    },
};
pub const MUNNA: Pokemon = Pokemon {
    name: "Munna",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 76,
        attack: 25,
        defense: 45,
        special_attack: 67,
        special_defense: 55,
        speed: 24,
    },
};
pub const MURKROW: Pokemon = Pokemon {
    name: "Murkrow",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 85,
        defense: 42,
        special_attack: 85,
        special_defense: 42,
        speed: 91,
    },
};
pub const MUSHARNA: Pokemon = Pokemon {
    name: "Musharna",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 116,
        attack: 55,
        defense: 85,
        special_attack: 107,
        special_defense: 95,
        speed: 29,
    },
};
pub const NAGANADEL: Pokemon = Pokemon {
    name: "Naganadel",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 73,
        attack: 73,
        defense: 73,
        special_attack: 127,
        special_defense: 73,
        speed: 121,
    },
};
pub const NATU: Pokemon = Pokemon {
    name: "Natu",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 50,
        defense: 45,
        special_attack: 70,
        special_defense: 45,
        speed: 70,
    },
};
pub const NECROZMA: Pokemon = Pokemon {
    name: "Necrozma",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 97,
        attack: 107,
        defense: 101,
        special_attack: 127,
        special_defense: 89,
        speed: 79,
    },
};
pub const NICKIT: Pokemon = Pokemon {
    name: "Nickit",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 28,
        defense: 28,
        special_attack: 47,
        special_defense: 52,
        speed: 50,
    },
};
pub const NIDOKING: Pokemon = Pokemon {
    name: "Nidoking",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 81,
        attack: 102,
        defense: 77,
        special_attack: 85,
        special_defense: 75,
        speed: 85,
    },
};
pub const NIDOQUEEN: Pokemon = Pokemon {
    name: "Nidoqueen",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 90,
        attack: 92,
        defense: 87,
        special_attack: 75,
        special_defense: 85,
        speed: 76,
    },
};
pub const NIDORAN_F: Pokemon = Pokemon {
    name: "Nidoran (female)",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 47,
        defense: 52,
        special_attack: 40,
        special_defense: 40,
        speed: 41,
    },
};
pub const NIDORAN_M: Pokemon = Pokemon {
    name: "Nidoran (male)",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 46,
        attack: 57,
        defense: 40,
        special_attack: 40,
        special_defense: 40,
        speed: 50,
    },
};
pub const NIDORINA: Pokemon = Pokemon {
    name: "Nidorina",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 62,
        defense: 67,
        special_attack: 55,
        special_defense: 55,
        speed: 56,
    },
};
pub const NIDORINO: Pokemon = Pokemon {
    name: "Nidorino",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 61,
        attack: 72,
        defense: 57,
        special_attack: 55,
        special_defense: 55,
        speed: 65,
    },
};
pub const NIHILEGO: Pokemon = Pokemon {
    name: "Nihilego",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 109,
        attack: 53,
        defense: 47,
        special_attack: 127,
        special_defense: 131,
        speed: 103,
    },
};
pub const NINCADA: Pokemon = Pokemon {
    name: "Nincada",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 31,
        attack: 45,
        defense: 90,
        special_attack: 30,
        special_defense: 30,
        speed: 40,
    },
};
pub const NINETALES: Pokemon = Pokemon {
    name: "Ninetales",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 73,
        attack: 76,
        defense: 75,
        special_attack: 81,
        special_defense: 100,
        speed: 100,
    },
};
pub const NINJASK: Pokemon = Pokemon {
    name: "Ninjask",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 61,
        attack: 90,
        defense: 45,
        special_attack: 50,
        special_defense: 50,
        speed: 160,
    },
};
pub const NOCTOWL: Pokemon = Pokemon {
    name: "Noctowl",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 100,
        attack: 50,
        defense: 50,
        special_attack: 86,
        special_defense: 96,
        speed: 70,
    },
};
pub const NOIBAT: Pokemon = Pokemon {
    name: "Noibat",
    typeset: TypeData {
        primary: PokemonType::Flying,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 40,
        attack: 30,
        defense: 35,
        special_attack: 45,
        special_defense: 40,
        speed: 55,
    },
};
pub const NOIVERN: Pokemon = Pokemon {
    name: "Noivern",
    typeset: TypeData {
        primary: PokemonType::Flying,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 85,
        attack: 70,
        defense: 80,
        special_attack: 97,
        special_defense: 80,
        speed: 123,
    },
};
pub const NOSEPASS: Pokemon = Pokemon {
    name: "Nosepass",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 45,
        defense: 135,
        special_attack: 45,
        special_defense: 90,
        speed: 30,
    },
};
pub const NUMEL: Pokemon = Pokemon {
    name: "Numel",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 40,
        special_attack: 65,
        special_defense: 45,
        speed: 35,
    },
};
pub const NUZLEAF: Pokemon = Pokemon {
    name: "Nuzleaf",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 70,
        attack: 70,
        defense: 40,
        special_attack: 60,
        special_defense: 40,
        speed: 60,
    },
};
pub const OBSTAGOON: Pokemon = Pokemon {
    name: "Obstagoon",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Normal),
    },
    stats: Stats {
        hp: 93,
        attack: 90,
        defense: 101,
        special_attack: 60,
        special_defense: 81,
        speed: 95,
    },
};
pub const OCTILLERY: Pokemon = Pokemon {
    name: "Octillery",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 105,
        defense: 75,
        special_attack: 105,
        special_defense: 75,
        speed: 45,
    },
};
pub const ODDISH: Pokemon = Pokemon {
    name: "Oddish",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 45,
        attack: 50,
        defense: 55,
        special_attack: 75,
        special_defense: 65,
        speed: 30,
    },
};
pub const OMANYTE: Pokemon = Pokemon {
    name: "Omanyte",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 35,
        attack: 40,
        defense: 100,
        special_attack: 90,
        special_defense: 55,
        speed: 35,
    },
};
pub const OMASTAR: Pokemon = Pokemon {
    name: "Omastar",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 70,
        attack: 60,
        defense: 125,
        special_attack: 115,
        special_defense: 70,
        speed: 55,
    },
};
pub const ONIX: Pokemon = Pokemon {
    name: "Onix",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 35,
        attack: 45,
        defense: 160,
        special_attack: 30,
        special_defense: 45,
        speed: 70,
    },
};
pub const ORANGURU: Pokemon = Pokemon {
    name: "Oranguru",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 90,
        attack: 60,
        defense: 80,
        special_attack: 90,
        special_defense: 110,
        speed: 60,
    },
};
pub const ORBEETLE: Pokemon = Pokemon {
    name: "Orbeetle",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 60,
        attack: 45,
        defense: 110,
        special_attack: 80,
        special_defense: 120,
        speed: 90,
    },
};
pub const ORICORIO: Pokemon = Pokemon {
    name: "Oricorio",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 75,
        attack: 70,
        defense: 70,
        special_attack: 98,
        special_defense: 70,
        speed: 93,
    },
};
pub const OSHAWOTT: Pokemon = Pokemon {
    name: "Oshawott",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 55,
        defense: 45,
        special_attack: 63,
        special_defense: 45,
        speed: 45,
    },
};
pub const PACHIRISU: Pokemon = Pokemon {
    name: "Pachirisu",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 45,
        defense: 70,
        special_attack: 45,
        special_defense: 90,
        speed: 95,
    },
};
pub const PALKIA: Pokemon = Pokemon {
    name: "Palkia",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 90,
        attack: 120,
        defense: 100,
        special_attack: 150,
        special_defense: 120,
        speed: 100,
    },
};
pub const PALOSSAND: Pokemon = Pokemon {
    name: "Palossand",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 85,
        attack: 75,
        defense: 110,
        special_attack: 100,
        special_defense: 75,
        speed: 35,
    },
};
pub const PALPITOAD: Pokemon = Pokemon {
    name: "Palpitoad",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 75,
        attack: 65,
        defense: 55,
        special_attack: 65,
        special_defense: 55,
        speed: 69,
    },
};
pub const PANCHAM: Pokemon = Pokemon {
    name: "Pancham",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 67,
        attack: 82,
        defense: 62,
        special_attack: 46,
        special_defense: 48,
        speed: 43,
    },
};
pub const PANGORO: Pokemon = Pokemon {
    name: "Pangoro",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 95,
        attack: 124,
        defense: 78,
        special_attack: 69,
        special_defense: 71,
        speed: 58,
    },
};
pub const PANPOUR: Pokemon = Pokemon {
    name: "Panpour",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 53,
        defense: 48,
        special_attack: 53,
        special_defense: 48,
        speed: 64,
    },
};
pub const PANSAGE: Pokemon = Pokemon {
    name: "Pansage",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 53,
        defense: 48,
        special_attack: 53,
        special_defense: 48,
        speed: 64,
    },
};
pub const PANSEAR: Pokemon = Pokemon {
    name: "Pansear",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 53,
        defense: 48,
        special_attack: 53,
        special_defense: 48,
        speed: 64,
    },
};
pub const PARAS: Pokemon = Pokemon {
    name: "Paras",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 35,
        attack: 70,
        defense: 55,
        special_attack: 45,
        special_defense: 55,
        speed: 25,
    },
};
pub const PARASECT: Pokemon = Pokemon {
    name: "Parasect",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 60,
        attack: 95,
        defense: 80,
        special_attack: 60,
        special_defense: 80,
        speed: 30,
    },
};
pub const PASSIMIAN: Pokemon = Pokemon {
    name: "Passimian",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 120,
        defense: 90,
        special_attack: 40,
        special_defense: 60,
        speed: 80,
    },
};
pub const PATRAT: Pokemon = Pokemon {
    name: "Patrat",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 55,
        defense: 39,
        special_attack: 35,
        special_defense: 39,
        speed: 42,
    },
};
pub const PAWNIARD: Pokemon = Pokemon {
    name: "Pawniard",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 45,
        attack: 85,
        defense: 70,
        special_attack: 40,
        special_defense: 40,
        speed: 60,
    },
};
pub const PELIPPER: Pokemon = Pokemon {
    name: "Pelipper",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 100,
        special_attack: 95,
        special_defense: 70,
        speed: 65,
    },
};
pub const PERRSERKER: Pokemon = Pokemon {
    name: "Perrserker",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 110,
        defense: 100,
        special_attack: 50,
        special_defense: 60,
        speed: 50,
    },
};
pub const PERSIAN: Pokemon = Pokemon {
    name: "Persian",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 70,
        defense: 60,
        special_attack: 65,
        special_defense: 65,
        speed: 115,
    },
};
pub const PETILIL: Pokemon = Pokemon {
    name: "Petilil",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 35,
        defense: 50,
        special_attack: 70,
        special_defense: 50,
        speed: 30,
    },
};
pub const PHANPY: Pokemon = Pokemon {
    name: "Phanpy",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 60,
        defense: 60,
        special_attack: 40,
        special_defense: 40,
        speed: 40,
    },
};
pub const PHANTUMP: Pokemon = Pokemon {
    name: "Phantump",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 43,
        attack: 70,
        defense: 48,
        special_attack: 50,
        special_defense: 60,
        speed: 38,
    },
};
pub const PHEROMOSA: Pokemon = Pokemon {
    name: "Pheromosa",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 71,
        attack: 137,
        defense: 37,
        special_attack: 137,
        special_defense: 37,
        speed: 151,
    },
};
pub const PHIONE: Pokemon = Pokemon {
    name: "Phione",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 80,
        defense: 80,
        special_attack: 80,
        special_defense: 80,
        speed: 80,
    },
};
pub const PICHU: Pokemon = Pokemon {
    name: "Pichu",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 20,
        attack: 40,
        defense: 15,
        special_attack: 35,
        special_defense: 35,
        speed: 60,
    },
};
pub const PIDGEOT: Pokemon = Pokemon {
    name: "Pidgeot",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 83,
        attack: 80,
        defense: 75,
        special_attack: 70,
        special_defense: 70,
        speed: 101,
    },
};
pub const PIDGEOTTO: Pokemon = Pokemon {
    name: "Pidgeotto",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 63,
        attack: 60,
        defense: 55,
        special_attack: 50,
        special_defense: 50,
        speed: 71,
    },
};
pub const PIDGEY: Pokemon = Pokemon {
    name: "Pidgey",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 40,
        special_attack: 35,
        special_defense: 35,
        speed: 56,
    },
};
pub const PIDOVE: Pokemon = Pokemon {
    name: "Pidove",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 50,
        attack: 55,
        defense: 50,
        special_attack: 36,
        special_defense: 30,
        speed: 43,
    },
};
pub const PIGNITE: Pokemon = Pokemon {
    name: "Pignite",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 90,
        attack: 93,
        defense: 55,
        special_attack: 70,
        special_defense: 55,
        speed: 55,
    },
};
pub const PIKACHU: Pokemon = Pokemon {
    name: "Pikachu",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 55,
        defense: 40,
        special_attack: 50,
        special_defense: 50,
        speed: 90,
    },
};
pub const PIKIPEK: Pokemon = Pokemon {
    name: "Pikipek",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 35,
        attack: 75,
        defense: 30,
        special_attack: 30,
        special_defense: 30,
        speed: 65,
    },
};
pub const PILOSWINE: Pokemon = Pokemon {
    name: "Piloswine",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 80,
        special_attack: 60,
        special_defense: 60,
        speed: 50,
    },
};
pub const PINCURCHIN: Pokemon = Pokemon {
    name: "Pincurchin",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 48,
        attack: 101,
        defense: 95,
        special_attack: 91,
        special_defense: 85,
        speed: 15,
    },
};
pub const PINECO: Pokemon = Pokemon {
    name: "Pineco",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 65,
        defense: 90,
        special_attack: 35,
        special_defense: 35,
        speed: 15,
    },
};
pub const PINSIR: Pokemon = Pokemon {
    name: "Pinsir",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 125,
        defense: 100,
        special_attack: 55,
        special_defense: 70,
        speed: 85,
    },
};
pub const PIPLUP: Pokemon = Pokemon {
    name: "Piplup",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 53,
        attack: 51,
        defense: 53,
        special_attack: 61,
        special_defense: 56,
        speed: 40,
    },
};
pub const PLUSLE: Pokemon = Pokemon {
    name: "Plusle",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 50,
        defense: 40,
        special_attack: 85,
        special_defense: 75,
        speed: 95,
    },
};
pub const POIPOLE: Pokemon = Pokemon {
    name: "Poipole",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 67,
        attack: 73,
        defense: 67,
        special_attack: 73,
        special_defense: 67,
        speed: 73,
    },
};
pub const POLITOED: Pokemon = Pokemon {
    name: "Politoed",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 75,
        defense: 75,
        special_attack: 90,
        special_defense: 100,
        speed: 70,
    },
};
pub const POLIWAG: Pokemon = Pokemon {
    name: "Poliwag",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 50,
        defense: 40,
        special_attack: 40,
        special_defense: 40,
        speed: 90,
    },
};
pub const POLIWHIRL: Pokemon = Pokemon {
    name: "Poliwhirl",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 65,
        defense: 65,
        special_attack: 50,
        special_defense: 50,
        speed: 90,
    },
};
pub const POLIWRATH: Pokemon = Pokemon {
    name: "Poliwrath",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 90,
        attack: 95,
        defense: 95,
        special_attack: 70,
        special_defense: 90,
        speed: 70,
    },
};
pub const POLTEAGEIST: Pokemon = Pokemon {
    name: "Polteageist",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 65,
        defense: 65,
        special_attack: 134,
        special_defense: 114,
        speed: 70,
    },
};
pub const PONYTA: Pokemon = Pokemon {
    name: "Ponyta",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 85,
        defense: 55,
        special_attack: 65,
        special_defense: 65,
        speed: 90,
    },
};
pub const POOCHYENA: Pokemon = Pokemon {
    name: "Poochyena",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 55,
        defense: 35,
        special_attack: 30,
        special_defense: 30,
        speed: 35,
    },
};
pub const POPPLIO: Pokemon = Pokemon {
    name: "Popplio",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 54,
        defense: 54,
        special_attack: 66,
        special_defense: 56,
        speed: 40,
    },
};
pub const PORYGON: Pokemon = Pokemon {
    name: "Porygon",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 60,
        defense: 70,
        special_attack: 85,
        special_defense: 75,
        speed: 40,
    },
};
pub const PORYGON2: Pokemon = Pokemon {
    name: "Porygon2",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 80,
        defense: 90,
        special_attack: 105,
        special_defense: 95,
        speed: 60,
    },
};
pub const PORYGON_Z: Pokemon = Pokemon {
    name: "Porygon-Z",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 80,
        defense: 70,
        special_attack: 135,
        special_defense: 75,
        speed: 90,
    },
};
pub const PRIMARINA: Pokemon = Pokemon {
    name: "Primarina",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 80,
        attack: 74,
        defense: 74,
        special_attack: 126,
        special_defense: 116,
        speed: 60,
    },
};
pub const PRIMEAPE: Pokemon = Pokemon {
    name: "Primeape",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 105,
        defense: 60,
        special_attack: 60,
        special_defense: 70,
        speed: 95,
    },
};
pub const PRINPLUP: Pokemon = Pokemon {
    name: "Prinplup",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 64,
        attack: 66,
        defense: 68,
        special_attack: 81,
        special_defense: 76,
        speed: 50,
    },
};
pub const PROBOPASS: Pokemon = Pokemon {
    name: "Probopass",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 60,
        attack: 55,
        defense: 145,
        special_attack: 75,
        special_defense: 150,
        speed: 40,
    },
};
pub const PSYDUCK: Pokemon = Pokemon {
    name: "Psyduck",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 52,
        defense: 48,
        special_attack: 65,
        special_defense: 50,
        speed: 55,
    },
};
pub const PUMPKABOO: Pokemon = Pokemon {
    name: "Pumpkaboo",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 49,
        attack: 66,
        defense: 70,
        special_attack: 44,
        special_defense: 55,
        speed: 51,
    },
};
pub const PUPITAR: Pokemon = Pokemon {
    name: "Pupitar",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 70,
        attack: 84,
        defense: 70,
        special_attack: 65,
        special_defense: 70,
        speed: 51,
    },
};
pub const PURRLOIN: Pokemon = Pokemon {
    name: "Purrloin",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 41,
        attack: 50,
        defense: 37,
        special_attack: 50,
        special_defense: 37,
        speed: 66,
    },
};
pub const PURUGLY: Pokemon = Pokemon {
    name: "Purugly",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 71,
        attack: 82,
        defense: 64,
        special_attack: 64,
        special_defense: 59,
        speed: 112,
    },
};
pub const PYROAR: Pokemon = Pokemon {
    name: "Pyroar",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Normal),
    },
    stats: Stats {
        hp: 86,
        attack: 68,
        defense: 72,
        special_attack: 109,
        special_defense: 66,
        speed: 106,
    },
};
pub const PYUKUMUKU: Pokemon = Pokemon {
    name: "Pyukumuku",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 60,
        defense: 130,
        special_attack: 30,
        special_defense: 130,
        speed: 5,
    },
};
pub const QUAGSIRE: Pokemon = Pokemon {
    name: "Quagsire",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 95,
        attack: 85,
        defense: 85,
        special_attack: 65,
        special_defense: 65,
        speed: 35,
    },
};
pub const QUILAVA: Pokemon = Pokemon {
    name: "Quilava",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 58,
        attack: 64,
        defense: 58,
        special_attack: 80,
        special_defense: 65,
        speed: 80,
    },
};
pub const QUILLADIN: Pokemon = Pokemon {
    name: "Quilladin",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 61,
        attack: 78,
        defense: 95,
        special_attack: 56,
        special_defense: 58,
        speed: 57,
    },
};
pub const QWILFISH: Pokemon = Pokemon {
    name: "Qwilfish",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 65,
        attack: 95,
        defense: 85,
        special_attack: 55,
        special_defense: 55,
        speed: 85,
    },
};
pub const RABOOT: Pokemon = Pokemon {
    name: "Raboot",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 86,
        defense: 60,
        special_attack: 55,
        special_defense: 60,
        speed: 94,
    },
};
pub const RAICHU: Pokemon = Pokemon {
    name: "Raichu",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 90,
        defense: 55,
        special_attack: 90,
        special_defense: 80,
        speed: 110,
    },
};
pub const RAIKOU: Pokemon = Pokemon {
    name: "Raikou",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 85,
        defense: 75,
        special_attack: 115,
        special_defense: 100,
        speed: 115,
    },
};
pub const RALTS: Pokemon = Pokemon {
    name: "Ralts",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 28,
        attack: 25,
        defense: 25,
        special_attack: 45,
        special_defense: 35,
        speed: 40,
    },
};
pub const RAMPARDOS: Pokemon = Pokemon {
    name: "Rampardos",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 97,
        attack: 165,
        defense: 60,
        special_attack: 65,
        special_defense: 50,
        speed: 58,
    },
};
pub const RAPIDASH: Pokemon = Pokemon {
    name: "Rapidash",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 100,
        defense: 70,
        special_attack: 80,
        special_defense: 80,
        speed: 105,
    },
};
pub const RATICATE: Pokemon = Pokemon {
    name: "Raticate",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 81,
        defense: 60,
        special_attack: 50,
        special_defense: 70,
        speed: 97,
    },
};
pub const RATTATA: Pokemon = Pokemon {
    name: "Rattata",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 56,
        defense: 35,
        special_attack: 25,
        special_defense: 35,
        speed: 72,
    },
};
pub const RAYQUAZA: Pokemon = Pokemon {
    name: "Rayquaza",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 105,
        attack: 150,
        defense: 90,
        special_attack: 150,
        special_defense: 90,
        speed: 95,
    },
};
pub const REGICE: Pokemon = Pokemon {
    name: "Regice",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 50,
        defense: 100,
        special_attack: 100,
        special_defense: 200,
        speed: 50,
    },
};
pub const REGIDRAGO: Pokemon = Pokemon {
    name: "Regidrago",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 200,
        attack: 100,
        defense: 50,
        special_attack: 100,
        special_defense: 50,
        speed: 80,
    },
};
pub const REGIELEKI: Pokemon = Pokemon {
    name: "Regieleki",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 100,
        defense: 50,
        special_attack: 100,
        special_defense: 50,
        speed: 200,
    },
};
pub const REGIGIGAS: Pokemon = Pokemon {
    name: "Regigigas",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 110,
        attack: 160,
        defense: 110,
        special_attack: 80,
        special_defense: 110,
        speed: 100,
    },
};
pub const REGIROCK: Pokemon = Pokemon {
    name: "Regirock",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 100,
        defense: 200,
        special_attack: 50,
        special_defense: 100,
        speed: 50,
    },
};
pub const REGISTEEL: Pokemon = Pokemon {
    name: "Registeel",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 75,
        defense: 150,
        special_attack: 75,
        special_defense: 150,
        speed: 50,
    },
};
pub const RELICANTH: Pokemon = Pokemon {
    name: "Relicanth",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 100,
        attack: 90,
        defense: 130,
        special_attack: 45,
        special_defense: 65,
        speed: 55,
    },
};
pub const REMORAID: Pokemon = Pokemon {
    name: "Remoraid",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 65,
        defense: 35,
        special_attack: 65,
        special_defense: 35,
        speed: 65,
    },
};
pub const RESHIRAM: Pokemon = Pokemon {
    name: "Reshiram",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 100,
        attack: 120,
        defense: 100,
        special_attack: 150,
        special_defense: 120,
        speed: 90,
    },
};
pub const REUNICLUS: Pokemon = Pokemon {
    name: "Reuniclus",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 110,
        attack: 65,
        defense: 75,
        special_attack: 125,
        special_defense: 85,
        speed: 30,
    },
};
pub const RHYDON: Pokemon = Pokemon {
    name: "Rhydon",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 105,
        attack: 130,
        defense: 120,
        special_attack: 45,
        special_defense: 45,
        speed: 40,
    },
};
pub const RHYHORN: Pokemon = Pokemon {
    name: "Rhyhorn",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 80,
        attack: 85,
        defense: 95,
        special_attack: 30,
        special_defense: 30,
        speed: 25,
    },
};
pub const RHYPERIOR: Pokemon = Pokemon {
    name: "Rhyperior",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 115,
        attack: 140,
        defense: 130,
        special_attack: 55,
        special_defense: 55,
        speed: 40,
    },
};
pub const RIBOMBEE: Pokemon = Pokemon {
    name: "Ribombee",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 60,
        attack: 55,
        defense: 60,
        special_attack: 95,
        special_defense: 70,
        speed: 124,
    },
};
pub const RILLABOOM: Pokemon = Pokemon {
    name: "Rillaboom",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 125,
        defense: 90,
        special_attack: 60,
        special_defense: 70,
        speed: 85,
    },
};
pub const RIOLU: Pokemon = Pokemon {
    name: "Riolu",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 70,
        defense: 40,
        special_attack: 35,
        special_defense: 40,
        speed: 60,
    },
};
pub const ROCKRUFF: Pokemon = Pokemon {
    name: "Rockruff",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 65,
        defense: 40,
        special_attack: 30,
        special_defense: 40,
        speed: 60,
    },
};
pub const ROGGENROLA: Pokemon = Pokemon {
    name: "Roggenrola",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 75,
        defense: 85,
        special_attack: 25,
        special_defense: 25,
        speed: 15,
    },
};
pub const ROLYCOLY: Pokemon = Pokemon {
    name: "Rolycoly",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 40,
        defense: 50,
        special_attack: 40,
        special_defense: 50,
        speed: 30,
    },
};
pub const ROOKIDEE: Pokemon = Pokemon {
    name: "Rookidee",
    typeset: TypeData {
        primary: PokemonType::Flying,
        secondary: None,
    },
    stats: Stats {
        hp: 38,
        attack: 47,
        defense: 35,
        special_attack: 33,
        special_defense: 35,
        speed: 57,
    },
};
pub const ROSELIA: Pokemon = Pokemon {
    name: "Roselia",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 50,
        attack: 60,
        defense: 45,
        special_attack: 100,
        special_defense: 80,
        speed: 65,
    },
};
pub const ROSERADE: Pokemon = Pokemon {
    name: "Roserade",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 60,
        attack: 70,
        defense: 65,
        special_attack: 125,
        special_defense: 105,
        speed: 90,
    },
};
pub const ROTOM: Pokemon = Pokemon {
    name: "Rotom",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 50,
        attack: 50,
        defense: 77,
        special_attack: 95,
        special_defense: 77,
        speed: 91,
    },
};
pub const ROWLET: Pokemon = Pokemon {
    name: "Rowlet",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 68,
        attack: 55,
        defense: 55,
        special_attack: 50,
        special_defense: 50,
        speed: 42,
    },
};
pub const RUFFLET: Pokemon = Pokemon {
    name: "Rufflet",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 70,
        attack: 83,
        defense: 50,
        special_attack: 37,
        special_defense: 50,
        speed: 60,
    },
};
pub const RUNERIGUS: Pokemon = Pokemon {
    name: "Runerigus",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 58,
        attack: 95,
        defense: 145,
        special_attack: 50,
        special_defense: 105,
        speed: 30,
    },
};
pub const SABLEYE: Pokemon = Pokemon {
    name: "Sableye",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 50,
        attack: 75,
        defense: 75,
        special_attack: 65,
        special_defense: 65,
        speed: 50,
    },
};
pub const SALAMENCE: Pokemon = Pokemon {
    name: "Salamence",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 95,
        attack: 135,
        defense: 80,
        special_attack: 110,
        special_defense: 80,
        speed: 100,
    },
};
pub const SALANDIT: Pokemon = Pokemon {
    name: "Salandit",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 48,
        attack: 44,
        defense: 40,
        special_attack: 71,
        special_defense: 40,
        speed: 77,
    },
};
pub const SALAZZLE: Pokemon = Pokemon {
    name: "Salazzle",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 68,
        attack: 64,
        defense: 60,
        special_attack: 111,
        special_defense: 60,
        speed: 117,
    },
};
pub const SAMUROTT: Pokemon = Pokemon {
    name: "Samurott",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 100,
        defense: 85,
        special_attack: 108,
        special_defense: 70,
        speed: 70,
    },
};
pub const SANDACONDA: Pokemon = Pokemon {
    name: "Sandaconda",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 72,
        attack: 107,
        defense: 125,
        special_attack: 65,
        special_defense: 70,
        speed: 71,
    },
};
pub const SANDILE: Pokemon = Pokemon {
    name: "Sandile",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 50,
        attack: 72,
        defense: 35,
        special_attack: 35,
        special_defense: 35,
        speed: 65,
    },
};
pub const SANDSHREW: Pokemon = Pokemon {
    name: "Sandshrew",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 75,
        defense: 85,
        special_attack: 20,
        special_defense: 30,
        speed: 40,
    },
};
pub const SANDSLASH: Pokemon = Pokemon {
    name: "Sandslash",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 100,
        defense: 110,
        special_attack: 45,
        special_defense: 55,
        speed: 65,
    },
};
pub const SANDYGAST: Pokemon = Pokemon {
    name: "Sandygast",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 55,
        attack: 55,
        defense: 80,
        special_attack: 70,
        special_defense: 45,
        speed: 15,
    },
};
pub const SAWK: Pokemon = Pokemon {
    name: "Sawk",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 125,
        defense: 75,
        special_attack: 30,
        special_defense: 75,
        speed: 85,
    },
};
pub const SAWSBUCK: Pokemon = Pokemon {
    name: "Sawsbuck",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 80,
        attack: 100,
        defense: 70,
        special_attack: 60,
        special_defense: 70,
        speed: 95,
    },
};
pub const SCATTERBUG: Pokemon = Pokemon {
    name: "Scatterbug",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 38,
        attack: 35,
        defense: 40,
        special_attack: 27,
        special_defense: 25,
        speed: 35,
    },
};
pub const SCEPTILE: Pokemon = Pokemon {
    name: "Sceptile",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 85,
        defense: 65,
        special_attack: 105,
        special_defense: 85,
        speed: 120,
    },
};
pub const SCIZOR: Pokemon = Pokemon {
    name: "Scizor",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 70,
        attack: 130,
        defense: 100,
        special_attack: 55,
        special_defense: 80,
        speed: 65,
    },
};
pub const SCOLIPEDE: Pokemon = Pokemon {
    name: "Scolipede",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 60,
        attack: 100,
        defense: 89,
        special_attack: 55,
        special_defense: 69,
        speed: 112,
    },
};
pub const SCORBUNNY: Pokemon = Pokemon {
    name: "Scorbunny",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 71,
        defense: 40,
        special_attack: 40,
        special_defense: 40,
        speed: 69,
    },
};
pub const SCRAFTY: Pokemon = Pokemon {
    name: "Scrafty",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 65,
        attack: 90,
        defense: 115,
        special_attack: 45,
        special_defense: 115,
        speed: 58,
    },
};
pub const SCRAGGY: Pokemon = Pokemon {
    name: "Scraggy",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 50,
        attack: 75,
        defense: 70,
        special_attack: 35,
        special_defense: 70,
        speed: 48,
    },
};
pub const SCYTHER: Pokemon = Pokemon {
    name: "Scyther",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 70,
        attack: 110,
        defense: 80,
        special_attack: 55,
        special_defense: 80,
        speed: 105,
    },
};
pub const SEADRA: Pokemon = Pokemon {
    name: "Seadra",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 65,
        defense: 95,
        special_attack: 95,
        special_defense: 45,
        speed: 85,
    },
};
pub const SEAKING: Pokemon = Pokemon {
    name: "Seaking",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 92,
        defense: 65,
        special_attack: 65,
        special_defense: 80,
        speed: 68,
    },
};
pub const SEALEO: Pokemon = Pokemon {
    name: "Sealeo",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 90,
        attack: 60,
        defense: 70,
        special_attack: 75,
        special_defense: 70,
        speed: 45,
    },
};
pub const SEEDOT: Pokemon = Pokemon {
    name: "Seedot",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 40,
        defense: 50,
        special_attack: 30,
        special_defense: 30,
        speed: 30,
    },
};
pub const SEEL: Pokemon = Pokemon {
    name: "Seel",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 45,
        defense: 55,
        special_attack: 45,
        special_defense: 70,
        speed: 45,
    },
};
pub const SEISMITOAD: Pokemon = Pokemon {
    name: "Seismitoad",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 105,
        attack: 95,
        defense: 75,
        special_attack: 85,
        special_defense: 75,
        speed: 74,
    },
};
pub const SENTRET: Pokemon = Pokemon {
    name: "Sentret",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 46,
        defense: 34,
        special_attack: 35,
        special_defense: 45,
        speed: 20,
    },
};
pub const SERPERIOR: Pokemon = Pokemon {
    name: "Serperior",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 75,
        defense: 95,
        special_attack: 75,
        special_defense: 95,
        speed: 113,
    },
};
pub const SERVINE: Pokemon = Pokemon {
    name: "Servine",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 75,
        special_attack: 60,
        special_defense: 75,
        speed: 83,
    },
};
pub const SEVIPER: Pokemon = Pokemon {
    name: "Seviper",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 73,
        attack: 100,
        defense: 60,
        special_attack: 100,
        special_defense: 60,
        speed: 65,
    },
};
pub const SEWADDLE: Pokemon = Pokemon {
    name: "Sewaddle",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 45,
        attack: 53,
        defense: 70,
        special_attack: 40,
        special_defense: 60,
        speed: 42,
    },
};
pub const SHARPEDO: Pokemon = Pokemon {
    name: "Sharpedo",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 70,
        attack: 120,
        defense: 40,
        special_attack: 95,
        special_defense: 40,
        speed: 95,
    },
};
pub const SHAYMIN: Pokemon = Pokemon {
    name: "Shaymin",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
    },
};
pub const SHEDINJA: Pokemon = Pokemon {
    name: "Shedinja",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Ghost),
    },
    stats: Stats {
        hp: 1,
        attack: 90,
        defense: 45,
        special_attack: 30,
        special_defense: 30,
        speed: 40,
    },
};
pub const SHELGON: Pokemon = Pokemon {
    name: "Shelgon",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 95,
        defense: 100,
        special_attack: 60,
        special_defense: 50,
        speed: 50,
    },
};
pub const SHELLDER: Pokemon = Pokemon {
    name: "Shellder",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 65,
        defense: 100,
        special_attack: 45,
        special_defense: 25,
        speed: 40,
    },
};
pub const SHELLOS: Pokemon = Pokemon {
    name: "Shellos",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 76,
        attack: 48,
        defense: 48,
        special_attack: 57,
        special_defense: 62,
        speed: 34,
    },
};
pub const SHELMET: Pokemon = Pokemon {
    name: "Shelmet",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 40,
        defense: 85,
        special_attack: 40,
        special_defense: 65,
        speed: 25,
    },
};
pub const SHIELDON: Pokemon = Pokemon {
    name: "Shieldon",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 30,
        attack: 42,
        defense: 118,
        special_attack: 42,
        special_defense: 88,
        speed: 30,
    },
};
pub const SHIFTRY: Pokemon = Pokemon {
    name: "Shiftry",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 90,
        attack: 100,
        defense: 60,
        special_attack: 90,
        special_defense: 60,
        speed: 80,
    },
};
pub const SHIINOTIC: Pokemon = Pokemon {
    name: "Shiinotic",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 60,
        attack: 45,
        defense: 80,
        special_attack: 90,
        special_defense: 100,
        speed: 30,
    },
};
pub const SHINX: Pokemon = Pokemon {
    name: "Shinx",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 65,
        defense: 34,
        special_attack: 40,
        special_defense: 34,
        speed: 45,
    },
};
pub const SHROOMISH: Pokemon = Pokemon {
    name: "Shroomish",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 40,
        defense: 60,
        special_attack: 40,
        special_defense: 60,
        speed: 35,
    },
};
pub const SHUCKLE: Pokemon = Pokemon {
    name: "Shuckle",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 20,
        attack: 10,
        defense: 230,
        special_attack: 10,
        special_defense: 230,
        speed: 5,
    },
};
pub const SHUPPET: Pokemon = Pokemon {
    name: "Shuppet",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 44,
        attack: 75,
        defense: 35,
        special_attack: 63,
        special_defense: 33,
        speed: 45,
    },
};
pub const SIGILYPH: Pokemon = Pokemon {
    name: "Sigilyph",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 72,
        attack: 58,
        defense: 80,
        special_attack: 103,
        special_defense: 80,
        speed: 97,
    },
};
pub const SILCOON: Pokemon = Pokemon {
    name: "Silcoon",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 35,
        defense: 55,
        special_attack: 25,
        special_defense: 25,
        speed: 15,
    },
};
pub const SILICOBRA: Pokemon = Pokemon {
    name: "Silicobra",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 52,
        attack: 57,
        defense: 75,
        special_attack: 35,
        special_defense: 50,
        speed: 46,
    },
};
pub const SILVALLY: Pokemon = Pokemon {
    name: "Silvally",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 95,
        defense: 95,
        special_attack: 95,
        special_defense: 95,
        speed: 95,
    },
};
pub const SIMIPOUR: Pokemon = Pokemon {
    name: "Simipour",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 98,
        defense: 63,
        special_attack: 98,
        special_defense: 63,
        speed: 101,
    },
};
pub const SIMISAGE: Pokemon = Pokemon {
    name: "Simisage",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 98,
        defense: 63,
        special_attack: 98,
        special_defense: 63,
        speed: 101,
    },
};
pub const SIMISEAR: Pokemon = Pokemon {
    name: "Simisear",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 98,
        defense: 63,
        special_attack: 98,
        special_defense: 63,
        speed: 101,
    },
};
pub const SINISTEA: Pokemon = Pokemon {
    name: "Sinistea",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 45,
        special_attack: 74,
        special_defense: 54,
        speed: 50,
    },
};
pub const SIRFETCHD: Pokemon = Pokemon {
    name: "Sirfetch'd",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 62,
        attack: 135,
        defense: 95,
        special_attack: 68,
        special_defense: 82,
        speed: 65,
    },
};
pub const SIZZLIPEDE: Pokemon = Pokemon {
    name: "Sizzlipede",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 50,
        attack: 65,
        defense: 45,
        special_attack: 50,
        special_defense: 50,
        speed: 45,
    },
};
pub const SKARMORY: Pokemon = Pokemon {
    name: "Skarmory",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 65,
        attack: 80,
        defense: 140,
        special_attack: 40,
        special_defense: 70,
        speed: 70,
    },
};
pub const SKIDDO: Pokemon = Pokemon {
    name: "Skiddo",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 66,
        attack: 65,
        defense: 48,
        special_attack: 62,
        special_defense: 57,
        speed: 52,
    },
};
pub const SKIPLOOM: Pokemon = Pokemon {
    name: "Skiploom",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 45,
        defense: 50,
        special_attack: 45,
        special_defense: 65,
        speed: 80,
    },
};
pub const SKITTY: Pokemon = Pokemon {
    name: "Skitty",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 45,
        defense: 45,
        special_attack: 35,
        special_defense: 35,
        speed: 50,
    },
};
pub const SKORUPI: Pokemon = Pokemon {
    name: "Skorupi",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 40,
        attack: 50,
        defense: 90,
        special_attack: 30,
        special_defense: 55,
        speed: 65,
    },
};
pub const SKRELP: Pokemon = Pokemon {
    name: "Skrelp",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 50,
        attack: 60,
        defense: 60,
        special_attack: 60,
        special_defense: 60,
        speed: 30,
    },
};
pub const SKUNTANK: Pokemon = Pokemon {
    name: "Skuntank",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 103,
        attack: 93,
        defense: 67,
        special_attack: 71,
        special_defense: 61,
        speed: 84,
    },
};
pub const SKWOVET: Pokemon = Pokemon {
    name: "Skwovet",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 55,
        defense: 55,
        special_attack: 35,
        special_defense: 35,
        speed: 25,
    },
};
pub const SLAKING: Pokemon = Pokemon {
    name: "Slaking",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 150,
        attack: 160,
        defense: 100,
        special_attack: 95,
        special_defense: 65,
        speed: 100,
    },
};
pub const SLAKOTH: Pokemon = Pokemon {
    name: "Slakoth",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 60,
        special_attack: 35,
        special_defense: 35,
        speed: 30,
    },
};
pub const SLIGGOO: Pokemon = Pokemon {
    name: "Sliggoo",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: None,
    },
    stats: Stats {
        hp: 68,
        attack: 75,
        defense: 53,
        special_attack: 83,
        special_defense: 113,
        speed: 60,
    },
};
pub const SLOWBRO: Pokemon = Pokemon {
    name: "Slowbro",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 95,
        attack: 75,
        defense: 110,
        special_attack: 100,
        special_defense: 80,
        speed: 30,
    },
};
pub const SLOWKING: Pokemon = Pokemon {
    name: "Slowking",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 95,
        attack: 75,
        defense: 80,
        special_attack: 100,
        special_defense: 110,
        speed: 30,
    },
};
pub const SLOWPOKE: Pokemon = Pokemon {
    name: "Slowpoke",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 90,
        attack: 65,
        defense: 65,
        special_attack: 40,
        special_defense: 40,
        speed: 15,
    },
};
pub const SLUGMA: Pokemon = Pokemon {
    name: "Slugma",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 40,
        defense: 40,
        special_attack: 70,
        special_defense: 40,
        speed: 20,
    },
};
pub const SLURPUFF: Pokemon = Pokemon {
    name: "Slurpuff",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 82,
        attack: 80,
        defense: 86,
        special_attack: 85,
        special_defense: 75,
        speed: 72,
    },
};
pub const SMEARGLE: Pokemon = Pokemon {
    name: "Smeargle",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 20,
        defense: 35,
        special_attack: 20,
        special_defense: 45,
        speed: 75,
    },
};
pub const SMOOCHUM: Pokemon = Pokemon {
    name: "Smoochum",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 45,
        attack: 30,
        defense: 15,
        special_attack: 85,
        special_defense: 65,
        speed: 65,
    },
};
pub const SNEASEL: Pokemon = Pokemon {
    name: "Sneasel",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 55,
        attack: 95,
        defense: 55,
        special_attack: 35,
        special_defense: 75,
        speed: 115,
    },
};
pub const SNIVY: Pokemon = Pokemon {
    name: "Snivy",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 45,
        defense: 55,
        special_attack: 45,
        special_defense: 55,
        speed: 63,
    },
};
pub const SNOM: Pokemon = Pokemon {
    name: "Snom",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Bug),
    },
    stats: Stats {
        hp: 30,
        attack: 25,
        defense: 35,
        special_attack: 45,
        special_defense: 30,
        speed: 20,
    },
};
pub const SNORLAX: Pokemon = Pokemon {
    name: "Snorlax",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 160,
        attack: 110,
        defense: 65,
        special_attack: 65,
        special_defense: 110,
        speed: 30,
    },
};
pub const SNORUNT: Pokemon = Pokemon {
    name: "Snorunt",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 50,
        defense: 50,
        special_attack: 50,
        special_defense: 50,
        speed: 50,
    },
};
pub const SNOVER: Pokemon = Pokemon {
    name: "Snover",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 60,
        attack: 62,
        defense: 50,
        special_attack: 62,
        special_defense: 60,
        speed: 40,
    },
};
pub const SNUBBULL: Pokemon = Pokemon {
    name: "Snubbull",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 80,
        defense: 50,
        special_attack: 40,
        special_defense: 40,
        speed: 30,
    },
};
pub const SNUBBULL_GEN2: Pokemon = Pokemon {
    name: "Snubbull",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 80,
        defense: 50,
        special_attack: 40,
        special_defense: 40,
        speed: 30,
    },
};
pub const SOBBLE: Pokemon = Pokemon {
    name: "Sobble",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 40,
        defense: 40,
        special_attack: 70,
        special_defense: 40,
        speed: 70,
    },
};
pub const SOLGALEO: Pokemon = Pokemon {
    name: "Solgaleo",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 137,
        attack: 137,
        defense: 107,
        special_attack: 113,
        special_defense: 89,
        speed: 97,
    },
};
pub const SOLOSIS: Pokemon = Pokemon {
    name: "Solosis",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 30,
        defense: 40,
        special_attack: 105,
        special_defense: 50,
        speed: 20,
    },
};
pub const SOLROCK: Pokemon = Pokemon {
    name: "Solrock",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 90,
        attack: 95,
        defense: 85,
        special_attack: 55,
        special_defense: 65,
        speed: 70,
    },
};
pub const SPEAROW: Pokemon = Pokemon {
    name: "Spearow",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 60,
        defense: 30,
        special_attack: 31,
        special_defense: 31,
        speed: 70,
    },
};
pub const SPECTRIER: Pokemon = Pokemon {
    name: "Spectrier",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 65,
        defense: 60,
        special_attack: 145,
        special_defense: 80,
        speed: 130,
    },
};
pub const SPEWPA: Pokemon = Pokemon {
    name: "Spewpa",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 22,
        defense: 60,
        special_attack: 27,
        special_defense: 30,
        speed: 29,
    },
};
pub const SPHEAL: Pokemon = Pokemon {
    name: "Spheal",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 70,
        attack: 40,
        defense: 50,
        special_attack: 55,
        special_defense: 50,
        speed: 25,
    },
};
pub const SPINARAK: Pokemon = Pokemon {
    name: "Spinarak",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 40,
        attack: 60,
        defense: 40,
        special_attack: 40,
        special_defense: 40,
        speed: 30,
    },
};
pub const SPINDA: Pokemon = Pokemon {
    name: "Spinda",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 60,
        defense: 60,
        special_attack: 60,
        special_defense: 60,
        speed: 60,
    },
};
pub const SPIRITOMB: Pokemon = Pokemon {
    name: "Spiritomb",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 50,
        attack: 92,
        defense: 108,
        special_attack: 92,
        special_defense: 108,
        speed: 35,
    },
};
pub const SPOINK: Pokemon = Pokemon {
    name: "Spoink",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 25,
        defense: 35,
        special_attack: 70,
        special_defense: 80,
        speed: 60,
    },
};
pub const SPRITZEE: Pokemon = Pokemon {
    name: "Spritzee",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 78,
        attack: 52,
        defense: 60,
        special_attack: 63,
        special_defense: 65,
        speed: 23,
    },
};
pub const SQUIRTLE: Pokemon = Pokemon {
    name: "Squirtle",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 44,
        attack: 48,
        defense: 65,
        special_attack: 50,
        special_defense: 64,
        speed: 43,
    },
};
pub const STAKATAKA: Pokemon = Pokemon {
    name: "Stakataka",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 61,
        attack: 131,
        defense: 211,
        special_attack: 53,
        special_defense: 101,
        speed: 13,
    },
};
pub const STANTLER: Pokemon = Pokemon {
    name: "Stantler",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 73,
        attack: 95,
        defense: 62,
        special_attack: 85,
        special_defense: 65,
        speed: 85,
    },
};
pub const STARAPTOR: Pokemon = Pokemon {
    name: "Staraptor",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 85,
        attack: 120,
        defense: 70,
        special_attack: 50,
        special_defense: 60,
        speed: 100,
    },
};
pub const STARAVIA: Pokemon = Pokemon {
    name: "Staravia",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 75,
        defense: 50,
        special_attack: 40,
        special_defense: 40,
        speed: 80,
    },
};
pub const STARLY: Pokemon = Pokemon {
    name: "Starly",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 55,
        defense: 30,
        special_attack: 30,
        special_defense: 30,
        speed: 60,
    },
};
pub const STARMIE: Pokemon = Pokemon {
    name: "Starmie",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Psychic),
    },
    stats: Stats {
        hp: 60,
        attack: 75,
        defense: 85,
        special_attack: 100,
        special_defense: 85,
        speed: 115,
    },
};
pub const STARYU: Pokemon = Pokemon {
    name: "Staryu",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 45,
        defense: 55,
        special_attack: 70,
        special_defense: 55,
        speed: 85,
    },
};
pub const STEELIX: Pokemon = Pokemon {
    name: "Steelix",
    typeset: TypeData {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 75,
        attack: 85,
        defense: 200,
        special_attack: 55,
        special_defense: 65,
        speed: 30,
    },
};
pub const STEENEE: Pokemon = Pokemon {
    name: "Steenee",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 52,
        attack: 40,
        defense: 48,
        special_attack: 40,
        special_defense: 48,
        speed: 62,
    },
};
pub const STONJOURNER: Pokemon = Pokemon {
    name: "Stonjourner",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 125,
        defense: 135,
        special_attack: 20,
        special_defense: 20,
        speed: 70,
    },
};
pub const STOUTLAND: Pokemon = Pokemon {
    name: "Stoutland",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 85,
        attack: 110,
        defense: 90,
        special_attack: 45,
        special_defense: 90,
        speed: 80,
    },
};
pub const STUFFUL: Pokemon = Pokemon {
    name: "Stufful",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 70,
        attack: 75,
        defense: 50,
        special_attack: 45,
        special_defense: 50,
        speed: 50,
    },
};
pub const STUNFISK: Pokemon = Pokemon {
    name: "Stunfisk",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 109,
        attack: 66,
        defense: 84,
        special_attack: 81,
        special_defense: 99,
        speed: 32,
    },
};
pub const STUNKY: Pokemon = Pokemon {
    name: "Stunky",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 63,
        attack: 63,
        defense: 47,
        special_attack: 41,
        special_defense: 41,
        speed: 74,
    },
};
pub const SUDOWOODO: Pokemon = Pokemon {
    name: "Sudowoodo",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 100,
        defense: 115,
        special_attack: 30,
        special_defense: 65,
        speed: 30,
    },
};
pub const SUICUNE: Pokemon = Pokemon {
    name: "Suicune",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 75,
        defense: 115,
        special_attack: 90,
        special_defense: 115,
        speed: 85,
    },
};
pub const SUNFLORA: Pokemon = Pokemon {
    name: "Sunflora",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 75,
        defense: 55,
        special_attack: 105,
        special_defense: 85,
        speed: 30,
    },
};
pub const SUNKERN: Pokemon = Pokemon {
    name: "Sunkern",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 30,
        attack: 30,
        defense: 30,
        special_attack: 30,
        special_defense: 30,
        speed: 30,
    },
};
pub const SURSKIT: Pokemon = Pokemon {
    name: "Surskit",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 40,
        attack: 30,
        defense: 32,
        special_attack: 50,
        special_defense: 52,
        speed: 65,
    },
};
pub const SWABLU: Pokemon = Pokemon {
    name: "Swablu",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 45,
        attack: 40,
        defense: 60,
        special_attack: 40,
        special_defense: 75,
        speed: 50,
    },
};
pub const SWADLOON: Pokemon = Pokemon {
    name: "Swadloon",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 55,
        attack: 63,
        defense: 90,
        special_attack: 50,
        special_defense: 80,
        speed: 42,
    },
};
pub const SWALOT: Pokemon = Pokemon {
    name: "Swalot",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 73,
        defense: 83,
        special_attack: 73,
        special_defense: 83,
        speed: 55,
    },
};
pub const SWAMPERT: Pokemon = Pokemon {
    name: "Swampert",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 100,
        attack: 110,
        defense: 90,
        special_attack: 85,
        special_defense: 90,
        speed: 70,
    },
};
pub const SWANNA: Pokemon = Pokemon {
    name: "Swanna",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 75,
        attack: 87,
        defense: 63,
        special_attack: 87,
        special_defense: 63,
        speed: 98,
    },
};
pub const SWELLOW: Pokemon = Pokemon {
    name: "Swellow",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 60,
        attack: 85,
        defense: 60,
        special_attack: 75,
        special_defense: 50,
        speed: 125,
    },
};
pub const SWINUB: Pokemon = Pokemon {
    name: "Swinub",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 50,
        attack: 50,
        defense: 40,
        special_attack: 30,
        special_defense: 30,
        speed: 50,
    },
};
pub const SWIRLIX: Pokemon = Pokemon {
    name: "Swirlix",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 62,
        attack: 48,
        defense: 66,
        special_attack: 59,
        special_defense: 57,
        speed: 49,
    },
};
pub const SWOOBAT: Pokemon = Pokemon {
    name: "Swoobat",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 67,
        attack: 57,
        defense: 55,
        special_attack: 77,
        special_defense: 55,
        speed: 114,
    },
};
pub const SYLVEON: Pokemon = Pokemon {
    name: "Sylveon",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 65,
        defense: 65,
        special_attack: 110,
        special_defense: 130,
        speed: 60,
    },
};
pub const TAILLOW: Pokemon = Pokemon {
    name: "Taillow",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 55,
        defense: 30,
        special_attack: 30,
        special_defense: 30,
        speed: 85,
    },
};
pub const TALONFLAME: Pokemon = Pokemon {
    name: "Talonflame",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 78,
        attack: 81,
        defense: 71,
        special_attack: 74,
        special_defense: 69,
        speed: 126,
    },
};
pub const TANGELA: Pokemon = Pokemon {
    name: "Tangela",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 55,
        defense: 115,
        special_attack: 100,
        special_defense: 40,
        speed: 60,
    },
};
pub const TANGROWTH: Pokemon = Pokemon {
    name: "Tangrowth",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 125,
        special_attack: 110,
        special_defense: 50,
        speed: 50,
    },
};
pub const TAPU_BULU: Pokemon = Pokemon {
    name: "Tapu Bulu",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 70,
        attack: 130,
        defense: 115,
        special_attack: 85,
        special_defense: 95,
        speed: 75,
    },
};
pub const TAPU_FINI: Pokemon = Pokemon {
    name: "Tapu Fini",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 70,
        attack: 75,
        defense: 115,
        special_attack: 95,
        special_defense: 130,
        speed: 85,
    },
};
pub const TAPU_KOKO: Pokemon = Pokemon {
    name: "Tapu Koko",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 70,
        attack: 115,
        defense: 85,
        special_attack: 95,
        special_defense: 75,
        speed: 130,
    },
};
pub const TAPU_LELE: Pokemon = Pokemon {
    name: "Tapu Lele",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 70,
        attack: 85,
        defense: 75,
        special_attack: 130,
        special_defense: 115,
        speed: 95,
    },
};
pub const TAUROS: Pokemon = Pokemon {
    name: "Tauros",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 100,
        defense: 95,
        special_attack: 40,
        special_defense: 70,
        speed: 110,
    },
};
pub const TEDDIURSA: Pokemon = Pokemon {
    name: "Teddiursa",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 80,
        defense: 50,
        special_attack: 50,
        special_defense: 50,
        speed: 40,
    },
};
pub const TENTACOOL: Pokemon = Pokemon {
    name: "Tentacool",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 40,
        attack: 40,
        defense: 35,
        special_attack: 50,
        special_defense: 100,
        speed: 70,
    },
};
pub const TENTACRUEL: Pokemon = Pokemon {
    name: "Tentacruel",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 80,
        attack: 70,
        defense: 65,
        special_attack: 80,
        special_defense: 120,
        speed: 100,
    },
};
pub const TEPIG: Pokemon = Pokemon {
    name: "Tepig",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 63,
        defense: 45,
        special_attack: 45,
        special_defense: 45,
        speed: 45,
    },
};
pub const TERRAKION: Pokemon = Pokemon {
    name: "Terrakion",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 91,
        attack: 129,
        defense: 90,
        special_attack: 72,
        special_defense: 90,
        speed: 108,
    },
};
pub const THIEVUL: Pokemon = Pokemon {
    name: "Thievul",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 58,
        defense: 58,
        special_attack: 87,
        special_defense: 92,
        speed: 90,
    },
};
pub const THROH: Pokemon = Pokemon {
    name: "Throh",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 120,
        attack: 100,
        defense: 85,
        special_attack: 30,
        special_defense: 85,
        speed: 45,
    },
};
pub const THUNDURUS: Pokemon = Pokemon {
    name: "Thundurus",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 79,
        attack: 115,
        defense: 70,
        special_attack: 125,
        special_defense: 80,
        speed: 111,
    },
};
pub const THWACKEY: Pokemon = Pokemon {
    name: "Thwackey",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 85,
        defense: 70,
        special_attack: 55,
        special_defense: 60,
        speed: 80,
    },
};
pub const TIMBURR: Pokemon = Pokemon {
    name: "Timburr",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 80,
        defense: 55,
        special_attack: 25,
        special_defense: 35,
        speed: 35,
    },
};
pub const TIRTOUGA: Pokemon = Pokemon {
    name: "Tirtouga",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Rock),
    },
    stats: Stats {
        hp: 54,
        attack: 78,
        defense: 103,
        special_attack: 53,
        special_defense: 45,
        speed: 22,
    },
};
pub const TOGEDEMARU: Pokemon = Pokemon {
    name: "Togedemaru",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 65,
        attack: 98,
        defense: 63,
        special_attack: 40,
        special_defense: 73,
        speed: 96,
    },
};
pub const TOGEKISS: Pokemon = Pokemon {
    name: "Togekiss",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 85,
        attack: 50,
        defense: 95,
        special_attack: 120,
        special_defense: 115,
        speed: 80,
    },
};
pub const TOGEPI: Pokemon = Pokemon {
    name: "Togepi",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 20,
        defense: 65,
        special_attack: 40,
        special_defense: 65,
        speed: 20,
    },
};
pub const TOGEPI_GEN2: Pokemon = Pokemon {
    name: "Togepi",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 20,
        defense: 65,
        special_attack: 40,
        special_defense: 65,
        speed: 20,
    },
};
pub const TOGETIC: Pokemon = Pokemon {
    name: "Togetic",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 40,
        defense: 85,
        special_attack: 80,
        special_defense: 105,
        speed: 40,
    },
};
pub const TOGETIC_GEN2: Pokemon = Pokemon {
    name: "Togetic",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 40,
        defense: 85,
        special_attack: 80,
        special_defense: 105,
        speed: 40,
    },
};
pub const TORCHIC: Pokemon = Pokemon {
    name: "Torchic",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 60,
        defense: 40,
        special_attack: 70,
        special_defense: 50,
        speed: 45,
    },
};
pub const TORKOAL: Pokemon = Pokemon {
    name: "Torkoal",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 70,
        attack: 85,
        defense: 140,
        special_attack: 85,
        special_defense: 70,
        speed: 20,
    },
};
pub const TORNADUS: Pokemon = Pokemon {
    name: "Tornadus",
    typeset: TypeData {
        primary: PokemonType::Flying,
        secondary: None,
    },
    stats: Stats {
        hp: 79,
        attack: 115,
        defense: 70,
        special_attack: 125,
        special_defense: 80,
        speed: 111,
    },
};
pub const TORRACAT: Pokemon = Pokemon {
    name: "Torracat",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 85,
        defense: 50,
        special_attack: 80,
        special_defense: 50,
        speed: 90,
    },
};
pub const TORTERRA: Pokemon = Pokemon {
    name: "Torterra",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 95,
        attack: 109,
        defense: 105,
        special_attack: 75,
        special_defense: 85,
        speed: 56,
    },
};
pub const TOTODILE: Pokemon = Pokemon {
    name: "Totodile",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 65,
        defense: 64,
        special_attack: 44,
        special_defense: 48,
        speed: 43,
    },
};
pub const TOUCANNON: Pokemon = Pokemon {
    name: "Toucannon",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 80,
        attack: 120,
        defense: 75,
        special_attack: 75,
        special_defense: 75,
        speed: 60,
    },
};
pub const TOXAPEX: Pokemon = Pokemon {
    name: "Toxapex",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 50,
        attack: 63,
        defense: 152,
        special_attack: 53,
        special_defense: 142,
        speed: 35,
    },
};
pub const TOXEL: Pokemon = Pokemon {
    name: "Toxel",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 40,
        attack: 38,
        defense: 35,
        special_attack: 54,
        special_defense: 35,
        speed: 40,
    },
};
pub const TOXICROAK: Pokemon = Pokemon {
    name: "Toxicroak",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 83,
        attack: 106,
        defense: 65,
        special_attack: 86,
        special_defense: 65,
        speed: 85,
    },
};
pub const TOXTRICITY: Pokemon = Pokemon {
    name: "Toxtricity",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 75,
        attack: 98,
        defense: 70,
        special_attack: 114,
        special_defense: 70,
        speed: 75,
    },
};
pub const TRANQUILL: Pokemon = Pokemon {
    name: "Tranquill",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 62,
        attack: 77,
        defense: 62,
        special_attack: 50,
        special_defense: 42,
        speed: 65,
    },
};
pub const TRAPINCH: Pokemon = Pokemon {
    name: "Trapinch",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 100,
        defense: 45,
        special_attack: 45,
        special_defense: 45,
        speed: 10,
    },
};
pub const TREECKO: Pokemon = Pokemon {
    name: "Treecko",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 35,
        special_attack: 65,
        special_defense: 55,
        speed: 70,
    },
};
pub const TREVENANT: Pokemon = Pokemon {
    name: "Trevenant",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 85,
        attack: 110,
        defense: 76,
        special_attack: 65,
        special_defense: 82,
        speed: 56,
    },
};
pub const TROPIUS: Pokemon = Pokemon {
    name: "Tropius",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 99,
        attack: 68,
        defense: 83,
        special_attack: 72,
        special_defense: 87,
        speed: 51,
    },
};
pub const TRUBBISH: Pokemon = Pokemon {
    name: "Trubbish",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 50,
        defense: 62,
        special_attack: 40,
        special_defense: 62,
        speed: 65,
    },
};
pub const TRUMBEAK: Pokemon = Pokemon {
    name: "Trumbeak",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 55,
        attack: 85,
        defense: 50,
        special_attack: 40,
        special_defense: 50,
        speed: 75,
    },
};
pub const TSAREENA: Pokemon = Pokemon {
    name: "Tsareena",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 72,
        attack: 120,
        defense: 98,
        special_attack: 50,
        special_defense: 98,
        speed: 72,
    },
};
pub const TURTONATOR: Pokemon = Pokemon {
    name: "Turtonator",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 60,
        attack: 78,
        defense: 135,
        special_attack: 91,
        special_defense: 85,
        speed: 36,
    },
};
pub const TURTWIG: Pokemon = Pokemon {
    name: "Turtwig",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: None,
    },
    stats: Stats {
        hp: 55,
        attack: 68,
        defense: 64,
        special_attack: 45,
        special_defense: 55,
        speed: 31,
    },
};
pub const TYMPOLE: Pokemon = Pokemon {
    name: "Tympole",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 50,
        attack: 50,
        defense: 40,
        special_attack: 50,
        special_defense: 40,
        speed: 64,
    },
};
pub const TYNAMO: Pokemon = Pokemon {
    name: "Tynamo",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 55,
        defense: 40,
        special_attack: 45,
        special_defense: 40,
        speed: 60,
    },
};
pub const TYPE_NULL: Pokemon = Pokemon {
    name: "Type: Null",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 95,
        defense: 95,
        special_attack: 95,
        special_defense: 95,
        speed: 59,
    },
};
pub const TYPHLOSION: Pokemon = Pokemon {
    name: "Typhlosion",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 78,
        attack: 84,
        defense: 78,
        special_attack: 109,
        special_defense: 85,
        speed: 100,
    },
};
pub const TYRANITAR: Pokemon = Pokemon {
    name: "Tyranitar",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 100,
        attack: 134,
        defense: 110,
        special_attack: 95,
        special_defense: 100,
        speed: 61,
    },
};
pub const TYRANTRUM: Pokemon = Pokemon {
    name: "Tyrantrum",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 82,
        attack: 121,
        defense: 119,
        special_attack: 69,
        special_defense: 59,
        speed: 71,
    },
};
pub const TYROGUE: Pokemon = Pokemon {
    name: "Tyrogue",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: None,
    },
    stats: Stats {
        hp: 35,
        attack: 35,
        defense: 35,
        special_attack: 35,
        special_defense: 35,
        speed: 35,
    },
};
pub const TYRUNT: Pokemon = Pokemon {
    name: "Tyrunt",
    typeset: TypeData {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 58,
        attack: 89,
        defense: 77,
        special_attack: 45,
        special_defense: 45,
        speed: 48,
    },
};
pub const UMBREON: Pokemon = Pokemon {
    name: "Umbreon",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 65,
        defense: 110,
        special_attack: 60,
        special_defense: 130,
        speed: 65,
    },
};
pub const UNFEZANT: Pokemon = Pokemon {
    name: "Unfezant",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 80,
        attack: 115,
        defense: 80,
        special_attack: 65,
        special_defense: 55,
        speed: 93,
    },
};
pub const UNOWN: Pokemon = Pokemon {
    name: "Unown",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 48,
        attack: 72,
        defense: 48,
        special_attack: 72,
        special_defense: 48,
        speed: 48,
    },
};
pub const URSARING: Pokemon = Pokemon {
    name: "Ursaring",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 90,
        attack: 130,
        defense: 75,
        special_attack: 75,
        special_defense: 75,
        speed: 55,
    },
};
pub const URSHIFU: Pokemon = Pokemon {
    name: "Urshifu",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Dark),
    },
    stats: Stats {
        hp: 100,
        attack: 130,
        defense: 100,
        special_attack: 63,
        special_defense: 60,
        speed: 97,
    },
};
pub const UXIE: Pokemon = Pokemon {
    name: "Uxie",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 75,
        defense: 130,
        special_attack: 75,
        special_defense: 130,
        speed: 95,
    },
};
pub const VANILLISH: Pokemon = Pokemon {
    name: "Vanillish",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 51,
        attack: 65,
        defense: 65,
        special_attack: 80,
        special_defense: 75,
        speed: 59,
    },
};
pub const VANILLITE: Pokemon = Pokemon {
    name: "Vanillite",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 36,
        attack: 50,
        defense: 50,
        special_attack: 65,
        special_defense: 60,
        speed: 44,
    },
};
pub const VANILLUXE: Pokemon = Pokemon {
    name: "Vanilluxe",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: None,
    },
    stats: Stats {
        hp: 71,
        attack: 95,
        defense: 85,
        special_attack: 110,
        special_defense: 95,
        speed: 79,
    },
};
pub const VAPOREON: Pokemon = Pokemon {
    name: "Vaporeon",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 130,
        attack: 65,
        defense: 60,
        special_attack: 110,
        special_defense: 95,
        speed: 65,
    },
};
pub const VENIPEDE: Pokemon = Pokemon {
    name: "Venipede",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 30,
        attack: 45,
        defense: 59,
        special_attack: 30,
        special_defense: 39,
        speed: 57,
    },
};
pub const VENOMOTH: Pokemon = Pokemon {
    name: "Venomoth",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 70,
        attack: 65,
        defense: 60,
        special_attack: 90,
        special_defense: 75,
        speed: 90,
    },
};
pub const VENONAT: Pokemon = Pokemon {
    name: "Venonat",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 60,
        attack: 55,
        defense: 50,
        special_attack: 40,
        special_defense: 55,
        speed: 45,
    },
};
pub const VENUSAUR: Pokemon = Pokemon {
    name: "Venusaur",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 80,
        attack: 82,
        defense: 83,
        special_attack: 100,
        special_defense: 100,
        speed: 80,
    },
};
pub const VESPIQUEN: Pokemon = Pokemon {
    name: "Vespiquen",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 70,
        attack: 80,
        defense: 102,
        special_attack: 80,
        special_defense: 102,
        speed: 40,
    },
};
pub const VIBRAVA: Pokemon = Pokemon {
    name: "Vibrava",
    typeset: TypeData {
        primary: PokemonType::Ground,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 50,
        attack: 70,
        defense: 50,
        special_attack: 50,
        special_defense: 50,
        speed: 70,
    },
};
pub const VICTINI: Pokemon = Pokemon {
    name: "Victini",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 100,
        attack: 100,
        defense: 100,
        special_attack: 100,
        special_defense: 100,
        speed: 100,
    },
};
pub const VICTREEBEL: Pokemon = Pokemon {
    name: "Victreebel",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 80,
        attack: 105,
        defense: 65,
        special_attack: 100,
        special_defense: 70,
        speed: 70,
    },
};
pub const VIGOROTH: Pokemon = Pokemon {
    name: "Vigoroth",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 80,
        attack: 80,
        defense: 80,
        special_attack: 55,
        special_defense: 55,
        speed: 90,
    },
};
pub const VIKAVOLT: Pokemon = Pokemon {
    name: "Vikavolt",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 77,
        attack: 70,
        defense: 90,
        special_attack: 145,
        special_defense: 75,
        speed: 43,
    },
};
pub const VILEPLUME: Pokemon = Pokemon {
    name: "Vileplume",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 75,
        attack: 80,
        defense: 85,
        special_attack: 110,
        special_defense: 90,
        speed: 50,
    },
};
pub const VIRIZION: Pokemon = Pokemon {
    name: "Virizion",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fighting),
    },
    stats: Stats {
        hp: 91,
        attack: 90,
        defense: 72,
        special_attack: 90,
        special_defense: 129,
        speed: 108,
    },
};
pub const VIVILLON: Pokemon = Pokemon {
    name: "Vivillon",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 80,
        attack: 52,
        defense: 50,
        special_attack: 90,
        special_defense: 50,
        speed: 89,
    },
};
pub const VOLBEAT: Pokemon = Pokemon {
    name: "Volbeat",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 73,
        defense: 75,
        special_attack: 47,
        special_defense: 85,
        speed: 85,
    },
};
pub const VOLCANION: Pokemon = Pokemon {
    name: "Volcanion",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 80,
        attack: 110,
        defense: 120,
        special_attack: 130,
        special_defense: 90,
        speed: 70,
    },
};
pub const VOLCARONA: Pokemon = Pokemon {
    name: "Volcarona",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fire),
    },
    stats: Stats {
        hp: 85,
        attack: 60,
        defense: 65,
        special_attack: 135,
        special_defense: 105,
        speed: 100,
    },
};
pub const VOLTORB: Pokemon = Pokemon {
    name: "Voltorb",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 30,
        defense: 50,
        special_attack: 55,
        special_defense: 55,
        speed: 100,
    },
};
pub const VULLABY: Pokemon = Pokemon {
    name: "Vullaby",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 70,
        attack: 55,
        defense: 75,
        special_attack: 45,
        special_defense: 65,
        speed: 60,
    },
};
pub const VULPIX: Pokemon = Pokemon {
    name: "Vulpix",
    typeset: TypeData {
        primary: PokemonType::Fire,
        secondary: None,
    },
    stats: Stats {
        hp: 38,
        attack: 41,
        defense: 40,
        special_attack: 50,
        special_defense: 65,
        speed: 65,
    },
};
pub const WAILMER: Pokemon = Pokemon {
    name: "Wailmer",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 130,
        attack: 70,
        defense: 35,
        special_attack: 70,
        special_defense: 35,
        speed: 60,
    },
};
pub const WAILORD: Pokemon = Pokemon {
    name: "Wailord",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 170,
        attack: 90,
        defense: 45,
        special_attack: 90,
        special_defense: 45,
        speed: 60,
    },
};
pub const WALREIN: Pokemon = Pokemon {
    name: "Walrein",
    typeset: TypeData {
        primary: PokemonType::Ice,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 110,
        attack: 80,
        defense: 90,
        special_attack: 95,
        special_defense: 90,
        speed: 65,
    },
};
pub const WARTORTLE: Pokemon = Pokemon {
    name: "Wartortle",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 59,
        attack: 63,
        defense: 80,
        special_attack: 65,
        special_defense: 80,
        speed: 58,
    },
};
pub const WATCHOG: Pokemon = Pokemon {
    name: "Watchog",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 85,
        defense: 69,
        special_attack: 60,
        special_defense: 69,
        speed: 77,
    },
};
pub const WEAVILE: Pokemon = Pokemon {
    name: "Weavile",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Ice),
    },
    stats: Stats {
        hp: 70,
        attack: 120,
        defense: 65,
        special_attack: 45,
        special_defense: 85,
        speed: 125,
    },
};
pub const WEEDLE: Pokemon = Pokemon {
    name: "Weedle",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 40,
        attack: 35,
        defense: 30,
        special_attack: 20,
        special_defense: 20,
        speed: 50,
    },
};
pub const WEEPINBELL: Pokemon = Pokemon {
    name: "Weepinbell",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 65,
        attack: 90,
        defense: 50,
        special_attack: 85,
        special_defense: 45,
        speed: 55,
    },
};
pub const WEEZING: Pokemon = Pokemon {
    name: "Weezing",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: None,
    },
    stats: Stats {
        hp: 65,
        attack: 90,
        defense: 120,
        special_attack: 85,
        special_defense: 70,
        speed: 60,
    },
};
pub const WHIMSICOTT: Pokemon = Pokemon {
    name: "Whimsicott",
    typeset: TypeData {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 60,
        attack: 67,
        defense: 85,
        special_attack: 77,
        special_defense: 75,
        speed: 116,
    },
};
pub const WHIRLIPEDE: Pokemon = Pokemon {
    name: "Whirlipede",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison),
    },
    stats: Stats {
        hp: 40,
        attack: 55,
        defense: 99,
        special_attack: 40,
        special_defense: 79,
        speed: 47,
    },
};
pub const WHISCASH: Pokemon = Pokemon {
    name: "Whiscash",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 110,
        attack: 78,
        defense: 73,
        special_attack: 76,
        special_defense: 71,
        speed: 60,
    },
};
pub const WHISMUR: Pokemon = Pokemon {
    name: "Whismur",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 64,
        attack: 51,
        defense: 23,
        special_attack: 51,
        special_defense: 23,
        speed: 28,
    },
};
pub const WIGGLYTUFF: Pokemon = Pokemon {
    name: "Wigglytuff",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Fairy),
    },
    stats: Stats {
        hp: 140,
        attack: 70,
        defense: 45,
        special_attack: 85,
        special_defense: 50,
        speed: 45,
    },
};
pub const WIGGLYTUFF_GEN1: Pokemon = Pokemon {
    name: "Wigglytuff",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 140,
        attack: 70,
        defense: 45,
        special_attack: 85,
        special_defense: 50,
        speed: 45,
    },
};
pub const WIMPOD: Pokemon = Pokemon {
    name: "Wimpod",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Water),
    },
    stats: Stats {
        hp: 25,
        attack: 35,
        defense: 40,
        special_attack: 20,
        special_defense: 30,
        speed: 80,
    },
};
pub const WINGULL: Pokemon = Pokemon {
    name: "Wingull",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 30,
        defense: 30,
        special_attack: 55,
        special_defense: 30,
        speed: 85,
    },
};
pub const WISHIWASHI: Pokemon = Pokemon {
    name: "Wishiwashi",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 20,
        defense: 20,
        special_attack: 25,
        special_defense: 25,
        speed: 30,
    },
};
pub const WOBBUFFET: Pokemon = Pokemon {
    name: "Wobbuffet",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 190,
        attack: 33,
        defense: 58,
        special_attack: 33,
        special_defense: 58,
        speed: 33,
    },
};
pub const WOOBAT: Pokemon = Pokemon {
    name: "Woobat",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 65,
        attack: 45,
        defense: 43,
        special_attack: 55,
        special_defense: 43,
        speed: 72,
    },
};
pub const WOOLOO: Pokemon = Pokemon {
    name: "Wooloo",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 42,
        attack: 40,
        defense: 55,
        special_attack: 40,
        special_defense: 45,
        speed: 48,
    },
};
pub const WOOPER: Pokemon = Pokemon {
    name: "Wooper",
    typeset: TypeData {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 55,
        attack: 45,
        defense: 45,
        special_attack: 25,
        special_defense: 25,
        speed: 15,
    },
};
pub const WORMADAM_PLANT: Pokemon = Pokemon {
    name: "Wormadam",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 60,
        attack: 59,
        defense: 85,
        special_attack: 79,
        special_defense: 105,
        speed: 36,
    },
};
pub const WORMADAM_SANDY: Pokemon = Pokemon {
    name: "Wormadam",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 60,
        attack: 79,
        defense: 105,
        special_attack: 59,
        special_defense: 85,
        speed: 36,
    },
};
pub const WORMADAM_TRASH: Pokemon = Pokemon {
    name: "Wormadam",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 60,
        attack: 69,
        defense: 95,
        special_attack: 69,
        special_defense: 95,
        speed: 36,
    },
};
pub const WURMPLE: Pokemon = Pokemon {
    name: "Wurmple",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: None,
    },
    stats: Stats {
        hp: 45,
        attack: 45,
        defense: 35,
        special_attack: 20,
        special_defense: 30,
        speed: 20,
    },
};
pub const WYNAUT: Pokemon = Pokemon {
    name: "Wynaut",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: None,
    },
    stats: Stats {
        hp: 95,
        attack: 23,
        defense: 48,
        special_attack: 23,
        special_defense: 48,
        speed: 23,
    },
};
pub const XATU: Pokemon = Pokemon {
    name: "Xatu",
    typeset: TypeData {
        primary: PokemonType::Psychic,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 65,
        attack: 75,
        defense: 70,
        special_attack: 95,
        special_defense: 70,
        speed: 95,
    },
};
pub const XERNEAS: Pokemon = Pokemon {
    name: "Xerneas",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: None,
    },
    stats: Stats {
        hp: 126,
        attack: 131,
        defense: 95,
        special_attack: 131,
        special_defense: 98,
        speed: 99,
    },
};
pub const XURKITREE: Pokemon = Pokemon {
    name: "Xurkitree",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 83,
        attack: 89,
        defense: 71,
        special_attack: 173,
        special_defense: 71,
        speed: 83,
    },
};
pub const YAMASK: Pokemon = Pokemon {
    name: "Yamask",
    typeset: TypeData {
        primary: PokemonType::Ghost,
        secondary: None,
    },
    stats: Stats {
        hp: 38,
        attack: 30,
        defense: 85,
        special_attack: 55,
        special_defense: 65,
        speed: 30,
    },
};
pub const YAMPER: Pokemon = Pokemon {
    name: "Yamper",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 59,
        attack: 45,
        defense: 50,
        special_attack: 40,
        special_defense: 50,
        speed: 26,
    },
};
pub const YANMA: Pokemon = Pokemon {
    name: "Yanma",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 65,
        attack: 65,
        defense: 45,
        special_attack: 75,
        special_defense: 45,
        speed: 95,
    },
};
pub const YANMEGA: Pokemon = Pokemon {
    name: "Yanmega",
    typeset: TypeData {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 86,
        attack: 76,
        defense: 86,
        special_attack: 116,
        special_defense: 56,
        speed: 95,
    },
};
pub const YUNGOOS: Pokemon = Pokemon {
    name: "Yungoos",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 48,
        attack: 70,
        defense: 30,
        special_attack: 30,
        special_defense: 30,
        speed: 45,
    },
};
pub const YVELTAL: Pokemon = Pokemon {
    name: "Yveltal",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 126,
        attack: 131,
        defense: 95,
        special_attack: 131,
        special_defense: 98,
        speed: 99,
    },
};
pub const ZACIAN: Pokemon = Pokemon {
    name: "Zacian",
    typeset: TypeData {
        primary: PokemonType::Fairy,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 92,
        attack: 170,
        defense: 115,
        special_attack: 80,
        special_defense: 115,
        speed: 148,
    },
};
pub const ZAMAZENTA: Pokemon = Pokemon {
    name: "Zamazenta",
    typeset: TypeData {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Steel),
    },
    stats: Stats {
        hp: 92,
        attack: 130,
        defense: 145,
        special_attack: 80,
        special_defense: 145,
        speed: 128,
    },
};
pub const ZANGOOSE: Pokemon = Pokemon {
    name: "Zangoose",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 73,
        attack: 115,
        defense: 60,
        special_attack: 60,
        special_defense: 60,
        speed: 90,
    },
};
pub const ZAPDOS: Pokemon = Pokemon {
    name: "Zapdos",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 90,
        attack: 90,
        defense: 85,
        special_attack: 125,
        special_defense: 90,
        speed: 100,
    },
};
pub const ZARUDE: Pokemon = Pokemon {
    name: "Zarude",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Grass),
    },
    stats: Stats {
        hp: 105,
        attack: 120,
        defense: 105,
        special_attack: 70,
        special_defense: 95,
        speed: 105,
    },
};
pub const ZEBSTRIKA: Pokemon = Pokemon {
    name: "Zebstrika",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 75,
        attack: 100,
        defense: 63,
        special_attack: 80,
        special_defense: 63,
        speed: 116,
    },
};
pub const ZEKROM: Pokemon = Pokemon {
    name: "Zekrom",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Electric),
    },
    stats: Stats {
        hp: 100,
        attack: 150,
        defense: 120,
        special_attack: 120,
        special_defense: 100,
        speed: 90,
    },
};
pub const ZERAORA: Pokemon = Pokemon {
    name: "Zeraora",
    typeset: TypeData {
        primary: PokemonType::Electric,
        secondary: None,
    },
    stats: Stats {
        hp: 88,
        attack: 112,
        defense: 75,
        special_attack: 102,
        special_defense: 80,
        speed: 143,
    },
};
pub const ZIGZAGOON: Pokemon = Pokemon {
    name: "Zigzagoon",
    typeset: TypeData {
        primary: PokemonType::Normal,
        secondary: None,
    },
    stats: Stats {
        hp: 38,
        attack: 30,
        defense: 41,
        special_attack: 30,
        special_defense: 41,
        speed: 60,
    },
};
pub const ZOROARK: Pokemon = Pokemon {
    name: "Zoroark",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 60,
        attack: 105,
        defense: 60,
        special_attack: 120,
        special_defense: 60,
        speed: 105,
    },
};
pub const ZORUA: Pokemon = Pokemon {
    name: "Zorua",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: None,
    },
    stats: Stats {
        hp: 40,
        attack: 65,
        defense: 40,
        special_attack: 80,
        special_defense: 40,
        speed: 65,
    },
};
pub const ZUBAT: Pokemon = Pokemon {
    name: "Zubat",
    typeset: TypeData {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Flying),
    },
    stats: Stats {
        hp: 40,
        attack: 45,
        defense: 35,
        special_attack: 30,
        special_defense: 40,
        speed: 55,
    },
};
pub const ZWEILOUS: Pokemon = Pokemon {
    name: "Zweilous",
    typeset: TypeData {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Dragon),
    },
    stats: Stats {
        hp: 72,
        attack: 85,
        defense: 70,
        special_attack: 65,
        special_defense: 70,
        speed: 58,
    },
};
pub const ZYGARDE: Pokemon = Pokemon {
    name: "Zygarde",
    typeset: TypeData {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ground),
    },
    stats: Stats {
        hp: 108,
        attack: 100,
        defense: 121,
        special_attack: 81,
        special_defense: 95,
        speed: 95,
    },
};
