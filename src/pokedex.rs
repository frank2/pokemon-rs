use std::collections::{HashMap, HashSet};

use crate::{Pokemon, PokemonGame};
use crate::monsters::*;
use crate::types::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Pokedex {
    pub game: Option<PokemonGame>,
    pub rules: TypeRules,
    pub pokemon: &'static [Pokemon]
}
impl Pokedex {
    pub fn typedata(&self) -> HashSet<TypeData> {
        self.pokemon.iter()
            .map(|x| x.typeset)
            .collect()
    }
    pub fn filter_type(&self, pkmn_type: PokemonType) -> Vec<Pokemon> {
        self.pokemon.iter()
            .filter(|x| x.typeset.primary == pkmn_type || x.typeset.secondary == Some(pkmn_type))
            .cloned()
            .collect()
    }
    pub fn filter_typeset(&self, pkmn_typeset: TypeData) -> Vec<Pokemon> {
        self.pokemon.iter()
            .filter(|x| x.typeset == pkmn_typeset)
            .cloned()
            .collect()
    }
    pub fn type_map(&self) -> HashMap<PokemonType, Vec<Pokemon>> {
        self.rules.to_set()
            .iter()
            .map(|&x| (x, self.filter_type(x)))
            .collect()
    }
    pub fn typeset_map(&self) -> HashMap<TypeData, Vec<Pokemon>> {
        self.typedata()
            .iter()
            .map(|&x| (x, self.filter_typeset(x)))
            .collect()
    }
    pub fn by_name(&self, name: &str) -> Option<Vec<Pokemon>> {
        let target_name = String::from(name).to_lowercase();
        let mut result = Vec::<Pokemon>::new();
        
        for pkmn in self.pokemon {
            let pkmn_name = String::from(pkmn.name).to_lowercase();

            if pkmn_name == target_name { result.push(pkmn.clone()); }
        }

        if result.len() == 0 { return None; }
        Some(result)
    }
    pub fn weakness_map(&self) -> HashMap<PokemonType, Vec<Pokemon>> {
        let mut weakness_data: HashMap<PokemonType, Vec<Pokemon>> = self.rules.to_set()
            .iter()
            .map(|&x| (x, Vec::<Pokemon>::new()))
            .collect();

        for pkmn in self.pokemon {
            for weakness in pkmn.typeset.weaknesses(&self.rules) {
                weakness_data.get_mut(&weakness)
                    .unwrap()
                    .push(pkmn.clone());
            }
        }

        weakness_data
    }
    pub fn resistance_map(&self) -> HashMap<PokemonType, Vec<Pokemon>> {
        let mut resistance_data: HashMap<PokemonType, Vec<Pokemon>> = self.rules.to_set()
            .iter()
            .map(|&x| (x, Vec::<Pokemon>::new()))
            .collect();

        for pkmn in self.pokemon {
            for resistance in pkmn.typeset.resistance(&self.rules) {
                resistance_data.get_mut(&resistance)
                    .unwrap()
                    .push(pkmn.clone());
            }
        }

        resistance_data
    }
}

pub const REDBLUEYELLOW: Pokedex = Pokedex {
    game: Some(PokemonGame::RedBlueYellow),
    rules: GEN1_RULES,
    pokemon: &[
        ABRA,
        AERODACTYL,
        ALAKAZAM,
        ARBOK,
        ARCANINE,
        ARTICUNO,
        BEEDRILL,
        BELLSPROUT,
        BLASTOISE,
        BULBASAUR,
        BUTTERFREE,
        CATERPIE,
        CHANSEY,
        CHARIZARD,
        CHARMANDER,
        CHARMELEON,
        CLEFABLE_GEN1,
        CLEFAIRY_GEN1,
        CLOYSTER,
        CUBONE,
        DEWGONG,
        DIGLETT,
        DITTO,
        DODRIO,
        DODUO,
        DRAGONAIR,
        DRAGONITE,
        DRATINI,
        DROWZEE,
        DUGTRIO,
        EEVEE,
        EKANS,
        ELECTABUZZ,
        ELECTRODE,
        EXEGGCUTE,
        EXEGGUTOR,
        FARFETCHD,
        FEAROW,
        FLAREON,
        GASTLY,
        GENGAR,
        GEODUDE,
        GLOOM,
        GOLBAT,
        GOLDEEN,
        GOLDUCK,
        GOLEM,
        GRAVELER,
        GRIMER,
        GROWLITHE,
        GYARADOS,
        HAUNTER,
        HITMONCHAN,
        HITMONLEE,
        HORSEA,
        HYPNO,
        IVYSAUR,
        JIGGLYPUFF_GEN1,
        JOLTEON,
        JYNX,
        KABUTO,
        KABUTOPS,
        KADABRA,
        KAKUNA,
        KANGASKHAN,
        KINGLER,
        KOFFING,
        KRABBY,
        LAPRAS,
        LICKITUNG,
        MACHAMP,
        MACHOKE,
        MACHOP,
        MAGIKARP,
        MAGMAR,
        MAGNEMITE_GEN1,
        MAGNETON_GEN1,
        MANKEY,
        MAROWAK,
        MEOWTH,
        METAPOD,
        MEW,
        MEWTWO,
        MOLTRES,
        MR_MIME_GEN1,
        MUK,
        NIDOKING,
        NIDOQUEEN,
        NIDORAN_F,
        NIDORAN_M,
        NIDORINA,
        NIDORINO,
        NINETALES,
        ODDISH,
        OMANYTE,
        OMASTAR,
        ONIX,
        PARAS,
        PARASECT,
        PERSIAN,
        PIDGEOT,
        PIDGEOTTO,
        PIDGEY,
        PIKACHU,
        PINSIR,
        POLIWAG,
        POLIWHIRL,
        POLIWRATH,
        PONYTA,
        PORYGON,
        PRIMEAPE,
        PSYDUCK,
        RAICHU,
        RAPIDASH,
        RATICATE,
        RATTATA,
        RHYDON,
        RHYHORN,
        SANDSHREW,
        SANDSLASH,
        SCYTHER,
        SEADRA,
        SEAKING,
        SEEL,
        SHELLDER,
        SLOWBRO,
        SLOWPOKE,
        SNORLAX,
        SPEAROW,
        SQUIRTLE,
        STARMIE,
        STARYU,
        TANGELA,
        TAUROS,
        TENTACOOL,
        TENTACRUEL,
        VAPOREON,
        VENOMOTH,
        VENONAT,
        VENUSAUR,
        VICTREEBEL,
        VILEPLUME,
        VOLTORB,
        VULPIX,
        WARTORTLE,
        WEEDLE,
        WEEPINBELL,
        WEEZING,
        WIGGLYTUFF_GEN1,
        ZAPDOS,
        ZUBAT,
    ],
};
