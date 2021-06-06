use crate::types::{PokemonType, ALL_POKEMON_TYPES};
use crate::pokemon::*;

use std::collections::HashMap;

pub struct Pokedex {
    pub name: &'static str,
    pub pokemon: &'static [Pokemon]
}
impl Pokedex {
    pub fn filter(&self, pkmn_type: PokemonType) -> Vec<Pokemon> {
        self.pokemon.iter()
            .filter(|x| x.typeset.primary == pkmn_type || x.typeset.secondary == Some(pkmn_type))
            .map(|&x| x)
            .collect()
    }
    pub fn type_map(&self) -> HashMap<PokemonType, Vec<Pokemon>> {
        let mut result = HashMap::<PokemonType, Vec<Pokemon>>::new();

        for pkmn_type in ALL_POKEMON_TYPES.iter() { result.insert(*pkmn_type, self.filter(*pkmn_type)); }

        result
    }
}

pub const GEN4: Pokedex = Pokedex {
    name: "Diamon/Pearl",
    pokemon: &[
        ABOMASNOW,
        ALAKAZAM,
        AMBIPOM,
        AZELF,
        AZUMARILL,
        BASTIODON,
        BEAUTIFLY,
        BIBAREL,
        BLISSEY,
        BRONZONG,
        CARNIVINE,
        CHATOT,
        CHERRIM,
        CHIMECHO,
        CLEFABLE,
        CROBAT,
        DIALGA,
        DRAPION,
        DRIFBLIM,
        DUSTOX,
        EMPOLEON,
        FLOATZEL,
        GARCHOMP,
        GASTRODON,
        GENGAR,
        GIRAFARIG,
        GOLDUCK,
        GOLEM,
        GYARADOS,
        HERACROSS,
        HIPPOWDON,
        HONCHKROW,
        INFERNAPE,
        KRICKETUNE,
        LOPUNNY,
        LUCARIO,
        LUMINEON,
        LUXRAY,
        MACHAMP,
        MANAPHY,
        MANTINE,
        MEDICHAM,
        MESPRIT,
        MILOTIC,
        MOTHIM,
        MR_MIME,
        NOCTOWL,
        OCTILLERY,
        PACHIRISU,
        PALKIA,
        PELIPPER,
        PURUGLY,
        QUAGSIRE,
        RAICHU,
        RAMPARDOS,
        RAPIDASH,
        REMORAID,
        ROSERADE,
        SEAKING,
        SKUNTANK,
        SNORLAX,
        SPIRITOMB,
        STARAPTOR,
        STEELIX,
        SUDOWOODO,
        TENTACRUEL,
        TORTERRA,
        TOXICROAK,
        UNOWN,
        UXIE,
        VESPIQUEN,
        WEAVILE,
        WHISCASH,
        WORMADAM_PLANT,
        WORMADAM_SANDY,
        WORMADAM_TRASH,
    ]
};
