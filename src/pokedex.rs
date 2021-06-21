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
    pub fn strength_map(&self) -> HashMap<PokemonType, Vec<Pokemon>> {
        let mut strength_data: HashMap<PokemonType, Vec<Pokemon>> = self.rules.to_set()
            .iter()
            .map(|&x| (x, Vec::<Pokemon>::new()))
            .collect();

        for pkmn in self.pokemon {
            for strength in pkmn.typeset.strengths(&self.rules) {
                strength_data.get_mut(&strength)
                    .unwrap()
                    .push(pkmn.clone());
            }
        }

        strength_data
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

pub const GOLDSILVERCRYSTAL: Pokedex = Pokedex {
    game: Some(PokemonGame::GoldSilverCrystal),
    rules: GEN2_RULES,
    pokemon: &[
        ABRA,
        AERODACTYL,
        AIPOM,
        ALAKAZAM,
        AMPHAROS,
        ARBOK,
        ARCANINE,
        ARIADOS,
        ARTICUNO,
        AZUMARILL_GEN2,
        BAYLEEF,
        BEEDRILL,
        BELLOSSOM,
        BELLSPROUT,
        BLASTOISE,
        BLISSEY,
        BULBASAUR,
        BUTTERFREE,
        CATERPIE,
        CELEBI,
        CHANSEY,
        CHARIZARD,
        CHARMANDER,
        CHARMELEON,
        CHIKORITA,
        CHINCHOU,
        CLEFABLE_GEN1,
        CLEFAIRY_GEN1,
        CLEFFA_GEN2,
        CLOYSTER,
        CORSOLA,
        CROBAT,
        CROCONAW,
        CUBONE,
        CYNDAQUIL,
        DELIBIRD,
        DEWGONG,
        DIGLETT,
        DITTO,
        DODRIO,
        DODUO,
        DONPHAN,
        DRAGONAIR,
        DRAGONITE,
        DRATINI,
        DROWZEE,
        DUGTRIO,
        DUNSPARCE,
        EEVEE,
        EKANS,
        ELECTABUZZ,
        ELECTRODE,
        ELEKID,
        ENTEI,
        ESPEON,
        EXEGGCUTE,
        EXEGGUTOR,
        FARFETCHD,
        FEAROW,
        FERALIGATR,
        FLAAFFY,
        FLAREON,
        FORRETRESS,
        FURRET,
        GASTLY,
        GENGAR,
        GEODUDE,
        GIRAFARIG,
        GLIGAR,
        GLOOM,
        GOLBAT,
        GOLDEEN,
        GOLDUCK,
        GOLEM,
        GRANBULL_GEN2,
        GRAVELER,
        GRIMER,
        GROWLITHE,
        GYARADOS,
        HAUNTER,
        HERACROSS,
        HITMONCHAN,
        HITMONLEE,
        HITMONTOP,
        HO_OH,
        HOOTHOOT,
        HOPPIP,
        HORSEA,
        HOUNDOOM,
        HOUNDOUR,
        HYPNO,
        IGGLYBUFF_GEN2,
        IVYSAUR,
        JIGGLYPUFF_GEN1,
        JOLTEON,
        JUMPLUFF,
        JYNX,
        KABUTO,
        KABUTOPS,
        KADABRA,
        KAKUNA,
        KANGASKHAN,
        KINGDRA,
        KINGLER,
        KOFFING,
        KRABBY,
        LANTURN,
        LAPRAS,
        LARVITAR,
        LEDIAN,
        LEDYBA,
        LICKITUNG,
        LUGIA,
        MACHAMP,
        MACHOKE,
        MACHOP,
        MAGBY,
        MAGCARGO,
        MAGIKARP,
        MAGMAR,
        MAGNEMITE,
        MAGNETON,
        MANKEY,
        MANTINE,
        MAREEP,
        MARILL_GEN2,
        MAROWAK,
        MEGANIUM,
        MEOWTH,
        METAPOD,
        MEW,
        MEWTWO,
        MILTANK,
        MISDREAVUS,
        MOLTRES,
        MR_MIME_GEN1,
        MUK,
        MURKROW,
        NATU,
        NIDOKING,
        NIDOQUEEN,
        NIDORAN_F,
        NIDORAN_M,
        NIDORINA,
        NIDORINO,
        NINETALES,
        NOCTOWL,
        OCTILLERY,
        ODDISH,
        OMANYTE,
        OMASTAR,
        ONIX,
        PARAS,
        PARASECT,
        PERSIAN,
        PHANPY,
        PICHU,
        PIDGEOT,
        PIDGEOTTO,
        PIDGEY,
        PIKACHU,
        PILOSWINE,
        PINECO,
        PINSIR,
        POLITOED,
        POLIWAG,
        POLIWHIRL,
        POLIWRATH,
        PONYTA,
        PORYGON,
        PORYGON2,
        PRIMEAPE,
        PSYDUCK,
        PUPITAR,
        QUAGSIRE,
        QUILAVA,
        QWILFISH,
        RAICHU,
        RAIKOU,
        RAPIDASH,
        RATICATE,
        RATTATA,
        REMORAID,
        RHYDON,
        RHYHORN,
        SANDSHREW,
        SANDSLASH,
        SCIZOR,
        SCYTHER,
        SEADRA,
        SEAKING,
        SEEL,
        SENTRET,
        SHELLDER,
        SHUCKLE,
        SKARMORY,
        SKIPLOOM,
        SLOWBRO,
        SLOWKING,
        SLOWPOKE,
        SLUGMA,
        SMEARGLE,
        SMOOCHUM,
        SNEASEL,
        SNORLAX,
        SNUBBULL_GEN2,
        SPEAROW,
        SPINARAK,
        SQUIRTLE,
        STANTLER,
        STARMIE,
        STARYU,
        STEELIX,
        SUDOWOODO,
        SUICUNE,
        SUNFLORA,
        SUNKERN,
        SWINUB,
        TANGELA,
        TAUROS,
        TEDDIURSA,
        TENTACOOL,
        TENTACRUEL,
        TOGEPI_GEN2,
        TOGETIC_GEN2,
        TOTODILE,
        TYPHLOSION,
        TYRANITAR,
        TYROGUE,
        UMBREON,
        UNOWN,
        URSARING,
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
        WOBBUFFET,
        WOOPER,
        XATU,
        YANMA,
        ZAPDOS,
        ZUBAT,
    ],
};
