use crate::types::{PokemonType, TypeSet};

use std::fmt;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Pokemon {
    pub name: &'static str,
    pub typeset: TypeSet
}
impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PKMN<{}: {}>", self.name, self.typeset)
    }
}

pub const ABOMASNOW: Pokemon = Pokemon {
    name: "Abomasnow",
    typeset: TypeSet {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Ice)
    }
};
pub const ALAKAZAM: Pokemon = Pokemon {
    name: "Alakazam",
    typeset: TypeSet {
        primary: PokemonType::Psychic,
        secondary: None
    }
};
pub const AMBIPOM: Pokemon = Pokemon {
    name: "Ambipom",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: None
    }
};
pub const AZELF: Pokemon = Pokemon {
    name: "Azelf",
    typeset: TypeSet {
        primary: PokemonType::Psychic,
        secondary: None
    }
};
pub const AZUMARILL: Pokemon = Pokemon {
    name: "Azumarill",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const BASTIODON: Pokemon = Pokemon {
    name: "Bastiodon",
    typeset: TypeSet {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Steel)
    }
};
pub const BEAUTIFLY: Pokemon = Pokemon {
    name: "Beautifly",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying)
    }
};
pub const BIBAREL: Pokemon = Pokemon {
    name: "Bibarel",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Water)
    }
};
pub const BLISSEY: Pokemon = Pokemon {
    name: "Blissey",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: None
    }
};
pub const BRONZONG: Pokemon = Pokemon {
    name: "Bronzong",
    typeset: TypeSet {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Psychic)
    }
};
pub const CARNIVINE: Pokemon = Pokemon {
    name: "Carnivine",
    typeset: TypeSet {
        primary: PokemonType::Grass,
        secondary: None
    }
};
pub const CHATOT: Pokemon = Pokemon {
    name: "Chatot",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying)
    }
};
pub const CHERRIM: Pokemon = Pokemon {
    name: "Cherrim",
    typeset: TypeSet {
        primary: PokemonType::Grass,
        secondary: None
    }
};
pub const CHIMECHO: Pokemon = Pokemon {
    name: "Chimecho",
    typeset: TypeSet {
        primary: PokemonType::Psychic,
        secondary: None
    }
};
pub const CLEFABLE: Pokemon = Pokemon {
    name: "Clefable",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: None
    }
};
pub const CROBAT: Pokemon = Pokemon {
    name: "Crobat",
    typeset: TypeSet {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Flying)
    }
};
pub const DIALGA: Pokemon = Pokemon {
    name: "Dialga",
    typeset: TypeSet {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Dragon)
    }
};
pub const DRAPION: Pokemon = Pokemon {
    name: "Drapion",
    typeset: TypeSet {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dark)
    }
};
pub const DRIFBLIM: Pokemon = Pokemon {
    name: "Drifblim",
    typeset: TypeSet {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Flying)
    }
};
pub const DUSTOX: Pokemon = Pokemon {
    name: "Dustox",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Poison)
    }
};
pub const EMPOLEON: Pokemon = Pokemon {
    name: "Empoleon",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Steel)
    }
};
pub const FLOATZEL: Pokemon = Pokemon {
    name: "Floatzel",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const GARCHOMP: Pokemon = Pokemon {
    name: "Garchomp",
    typeset: TypeSet {
        primary: PokemonType::Dragon,
        secondary: Some(PokemonType::Ground)
    }
};
pub const GASTRODON: Pokemon = Pokemon {
    name: "Gastrodon",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground)
    }
};
pub const GENGAR: Pokemon = Pokemon {
    name: "Gengar",
    typeset: TypeSet {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Poison)
    }
};
pub const GIRAFARIG: Pokemon = Pokemon {
    name: "Girafarig",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Psychic)
    }
};
pub const GOLDUCK: Pokemon = Pokemon {
    name: "Golduck",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const GOLEM: Pokemon = Pokemon {
    name: "Golem",
    typeset: TypeSet {
        primary: PokemonType::Rock,
        secondary: Some(PokemonType::Ground)
    }
};
pub const GYARADOS: Pokemon = Pokemon {
    name: "Gyarados",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying)
    }
};
pub const HERACROSS: Pokemon = Pokemon {
    name: "Heracross",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Fighting)
    }
};
pub const HIPPOWDON: Pokemon = Pokemon {
    name: "Hippowdon",
    typeset: TypeSet {
        primary: PokemonType::Ground,
        secondary: None
    }
};
pub const HONCHKROW: Pokemon = Pokemon {
    name: "Honchkrow",
    typeset: TypeSet {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Flying)
    }
};
pub const INFERNAPE: Pokemon = Pokemon {
    name: "Infernape",
    typeset: TypeSet {
        primary: PokemonType::Fire,
        secondary: Some(PokemonType::Fighting)
    }
};
pub const KRICKETUNE: Pokemon = Pokemon {
    name: "Kricketune",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: None
    }
};
pub const LOPUNNY: Pokemon = Pokemon {
    name: "Lopunny",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: None
    }
};
pub const LUCARIO: Pokemon = Pokemon {
    name: "Lucario",
    typeset: TypeSet {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Steel)
    }
};
pub const LUMINEON: Pokemon = Pokemon {
    name: "Lumineon",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const LUXRAY: Pokemon = Pokemon {
    name: "Luxray",
    typeset: TypeSet {
        primary: PokemonType::Electric,
        secondary: None
    }
};
pub const MACHAMP: Pokemon = Pokemon {
    name: "Machamp",
    typeset: TypeSet {
        primary: PokemonType::Fighting,
        secondary: None
    }
};
pub const MANAPHY: Pokemon = Pokemon {
    name: "Manaphy",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const MANTINE: Pokemon = Pokemon {
    name: "Mantine",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying)
    }
};
pub const MEDICHAM: Pokemon = Pokemon {
    name: "Medicham",
    typeset: TypeSet {
        primary: PokemonType::Fighting,
        secondary: Some(PokemonType::Psychic)
    }
};
pub const MESPRIT: Pokemon = Pokemon {
    name: "Mesprit",
    typeset: TypeSet {
        primary: PokemonType::Psychic,
        secondary: None
    }
};
pub const MILOTIC: Pokemon = Pokemon {
    name: "Milotic",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const MOTHIM: Pokemon = Pokemon {
    name: "Mothim",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying)
    }
};
pub const MR_MIME: Pokemon = Pokemon {
    name: "Mr. Mime",
    typeset: TypeSet {
        primary: PokemonType::Psychic,
        secondary: None
    }
};
pub const NOCTOWL: Pokemon = Pokemon {
    name: "Noctowl",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying)
    }
};
pub const OCTILLERY: Pokemon = Pokemon {
    name: "Octillery",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const PACHIRISU: Pokemon = Pokemon {
    name: "Pachirisu",
    typeset: TypeSet {
        primary: PokemonType::Electric,
        secondary: None
    }
};
pub const PALKIA: Pokemon = Pokemon {
    name: "Palkia",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Dragon)
    }
};
pub const PELIPPER: Pokemon = Pokemon {
    name: "Pelipper",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Flying)
    }
};
pub const PURUGLY: Pokemon = Pokemon {
    name: "Purugly",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: None
    }
};
pub const QUAGSIRE: Pokemon = Pokemon {
    name: "Quagsire",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground)
    }
};
pub const RAICHU: Pokemon = Pokemon {
    name: "Raichu",
    typeset: TypeSet {
        primary: PokemonType::Electric,
        secondary: None
    }
};
pub const RAMPARDOS: Pokemon = Pokemon {
    name: "Rampardos",
    typeset: TypeSet {
        primary: PokemonType::Rock,
        secondary: None
    }
};
pub const RAPIDASH: Pokemon = Pokemon {
    name: "Rapidash",
    typeset: TypeSet {
        primary: PokemonType::Fire,
        secondary: None
    }
};
pub const REMORAID: Pokemon = Pokemon {
    name: "Remoraid",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const ROSERADE: Pokemon = Pokemon {
    name: "Roserade",
    typeset: TypeSet {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Poison)
    }
};
pub const SEAKING: Pokemon = Pokemon {
    name: "Seaking",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: None
    }
};
pub const SKUNTANK: Pokemon = Pokemon {
    name: "Skuntank",
    typeset: TypeSet {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Dark)
    }
};
pub const SNORLAX: Pokemon = Pokemon {
    name: "Snorlax",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: None
    }
};
pub const SPIRITOMB: Pokemon = Pokemon {
    name: "Spiritomb",
    typeset: TypeSet {
        primary: PokemonType::Ghost,
        secondary: Some(PokemonType::Dark)
    }
};
pub const STARAPTOR: Pokemon = Pokemon {
    name: "Staraptor",
    typeset: TypeSet {
        primary: PokemonType::Normal,
        secondary: Some(PokemonType::Flying)
    }
};
pub const STEELIX: Pokemon = Pokemon {
    name: "Steelix",
    typeset: TypeSet {
        primary: PokemonType::Steel,
        secondary: Some(PokemonType::Ground)
    }
};
pub const SUDOWOODO: Pokemon = Pokemon {
    name: "Sudowoodo",
    typeset: TypeSet {
        primary: PokemonType::Rock,
        secondary: None
    }
};
pub const TENTACRUEL: Pokemon = Pokemon {
    name: "Tentacruel",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Poison)
    }
};
pub const TORTERRA: Pokemon = Pokemon {
    name: "Torterra",
    typeset: TypeSet {
        primary: PokemonType::Grass,
        secondary: Some(PokemonType::Ground)
    }
};
pub const TOXICROAK: Pokemon = Pokemon {
    name: "Toxicroak",
    typeset: TypeSet {
        primary: PokemonType::Poison,
        secondary: Some(PokemonType::Fighting)
    }
};
pub const UNOWN: Pokemon = Pokemon {
    name: "Unown",
    typeset: TypeSet {
        primary: PokemonType::Psychic,
        secondary: None
    }
};
pub const UXIE: Pokemon = Pokemon {
    name: "Uxie",
    typeset: TypeSet {
        primary: PokemonType::Psychic,
        secondary: None
    }
};
pub const VESPIQUEN: Pokemon = Pokemon {
    name: "Vespiquen",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Flying)
    }
};
pub const WEAVILE: Pokemon = Pokemon {
    name: "Weavile",
    typeset: TypeSet {
        primary: PokemonType::Dark,
        secondary: Some(PokemonType::Ice)
    }
};
pub const WHISCASH: Pokemon = Pokemon {
    name: "Whiscash",
    typeset: TypeSet {
        primary: PokemonType::Water,
        secondary: Some(PokemonType::Ground)
    }
};
pub const WORMADAM_PLANT: Pokemon = Pokemon {
    name: "Wormadam (Plant Cloak)",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Grass)
    }
};
pub const WORMADAM_SANDY: Pokemon = Pokemon {
    name: "Wormadam (Sandy Cloak)",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Ground)
    }
};
pub const WORMADAM_TRASH: Pokemon = Pokemon {
    name: "Wormadam (Trash Cloak)",
    typeset: TypeSet {
        primary: PokemonType::Bug,
        secondary: Some(PokemonType::Steel)
    }
};
