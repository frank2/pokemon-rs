use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Fairy,
}

pub const ALL_POKEMON_TYPES: [PokemonType; 18] = [
    PokemonType::Normal,
    PokemonType::Fire,
    PokemonType::Water,
    PokemonType::Electric,
    PokemonType::Grass,
    PokemonType::Ice,
    PokemonType::Fighting,
    PokemonType::Poison,
    PokemonType::Ground,
    PokemonType::Flying,
    PokemonType::Psychic,
    PokemonType::Bug,
    PokemonType::Rock,
    PokemonType::Ghost,
    PokemonType::Dragon,
    PokemonType::Dark,
    PokemonType::Steel,
    PokemonType::Fairy,
];

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct TypeInfo {
    pub type_id: PokemonType,
    pub strengths: &'static [PokemonType],
    pub weaknesses: &'static [PokemonType],
    pub ineffective: &'static [PokemonType],
}

pub const NORMAL: TypeInfo = TypeInfo {
    type_id: PokemonType::Normal,
    strengths: &[],
    weaknesses: &[PokemonType::Fighting],
    ineffective: &[PokemonType::Ghost],
};

pub const FIRE: TypeInfo = TypeInfo {
    type_id: PokemonType::Fire,
    strengths: &[
        PokemonType::Grass,
        PokemonType::Ice,
        PokemonType::Bug,
        PokemonType::Steel,
    ],
    weaknesses: &[
        PokemonType::Water,
        PokemonType::Ground,
        PokemonType::Rock,
    ],
    ineffective: &[
        PokemonType::Fire,
        PokemonType::Grass,
        PokemonType::Ice,
        PokemonType::Bug,
        PokemonType::Steel,
        PokemonType::Fairy,
    ],
};

pub const WATER: TypeInfo = TypeInfo {
    type_id: PokemonType::Water,
    strengths: &[
        PokemonType::Fire,
        PokemonType::Ground,
        PokemonType::Rock,
    ],
    weaknesses: &[
        PokemonType::Electric,
        PokemonType::Grass,
    ],
    ineffective: &[
        PokemonType::Fire,
        PokemonType::Water,
        PokemonType::Ice,
        PokemonType::Steel,
    ],
};

pub const ELECTRIC: TypeInfo = TypeInfo {
    type_id: PokemonType::Electric,
    strengths: &[
        PokemonType::Water,
        PokemonType::Flying,
    ],
    weaknesses: &[PokemonType::Ground],
    ineffective: &[
        PokemonType::Electric,
        PokemonType::Flying,
        PokemonType::Steel,
    ],
};

pub const GRASS: TypeInfo = TypeInfo {
    type_id: PokemonType::Grass,
    strengths: &[
        PokemonType::Water,
        PokemonType::Ground,
        PokemonType::Rock,
    ],
    weaknesses: &[
        PokemonType::Fire,
        PokemonType::Ice,
        PokemonType::Poison,
        PokemonType::Flying,
        PokemonType::Bug,
    ],
    ineffective: &[
        PokemonType::Water,
        PokemonType::Electric,
        PokemonType::Grass,
        PokemonType::Ground,
    ],
};

pub const ICE: TypeInfo = TypeInfo {
    type_id: PokemonType::Ice,
    strengths: &[
        PokemonType::Grass,
        PokemonType::Ground,
        PokemonType::Flying,
        PokemonType::Dragon,
    ],
    weaknesses: &[
        PokemonType::Fire,
        PokemonType::Fighting,
        PokemonType::Rock,
        PokemonType::Steel,
    ],
    ineffective: &[PokemonType::Ice],
};

pub const FIGHTING: TypeInfo = TypeInfo {
    type_id: PokemonType::Fighting,
    strengths: &[
        PokemonType::Normal,
        PokemonType::Ice,
        PokemonType::Rock,
        PokemonType::Dark,
        PokemonType::Steel,
    ],
    weaknesses: &[
        PokemonType::Flying,
        PokemonType::Psychic,
        PokemonType::Fairy,
    ],
    ineffective: &[
        PokemonType::Bug,
        PokemonType::Rock,
        PokemonType::Dark,
    ],
};

pub const POISON: TypeInfo = TypeInfo {
    type_id: PokemonType::Poison,
    strengths: &[
        PokemonType::Grass,
        PokemonType::Fairy,
    ],
    weaknesses: &[
        PokemonType::Ground,
        PokemonType::Psychic,
    ],
    ineffective: &[
        PokemonType::Grass,
        PokemonType::Fighting,
        PokemonType::Poison,
        PokemonType::Bug,
        PokemonType::Fairy
    ],
};

pub const GROUND: TypeInfo = TypeInfo {
    type_id: PokemonType::Ground,
    strengths: &[
        PokemonType::Fire,
        PokemonType::Electric,
        PokemonType::Poison,
        PokemonType::Rock,
        PokemonType::Steel,
    ],
    weaknesses: &[
        PokemonType::Water,
        PokemonType::Grass,
        PokemonType::Ice,
    ],
    ineffective: &[
        PokemonType::Electric,
        PokemonType::Poison,
        PokemonType::Rock,
    ],
};

pub const FLYING: TypeInfo = TypeInfo {
    type_id: PokemonType::Flying,
    strengths: &[
        PokemonType::Grass,
        PokemonType::Fighting,
        PokemonType::Bug,
    ],
    weaknesses: &[
        PokemonType::Electric,
        PokemonType::Ice,
        PokemonType::Rock,
    ],
    ineffective: &[
        PokemonType::Ground,
        PokemonType::Grass,
        PokemonType::Fighting,
        PokemonType::Bug,
    ],
};

pub const PSYCHIC: TypeInfo = TypeInfo {
    type_id: PokemonType::Psychic,
    strengths: &[
        PokemonType::Fighting,
        PokemonType::Poison,
    ],
    weaknesses: &[
        PokemonType::Bug,
        PokemonType::Ghost,
        PokemonType::Dark,
    ],
    ineffective: &[
        PokemonType::Fighting,
        PokemonType::Psychic,
    ],
};

pub const BUG: TypeInfo = TypeInfo {
    type_id: PokemonType::Bug,
    strengths: &[
        PokemonType::Grass,
        PokemonType::Psychic,
        PokemonType::Dark,
    ],
    weaknesses: &[
        PokemonType::Fire,
        PokemonType::Flying,
        PokemonType::Rock,
    ],
    ineffective: &[
        PokemonType::Grass,
        PokemonType::Fighting,
        PokemonType::Ground,
    ],
};

pub const ROCK: TypeInfo = TypeInfo {
    type_id: PokemonType::Rock,
    strengths: &[
        PokemonType::Fire,
        PokemonType::Ice,
        PokemonType::Flying,
        PokemonType::Bug,
    ],
    weaknesses: &[
        PokemonType::Water,
        PokemonType::Grass,
        PokemonType::Fighting,
        PokemonType::Ground,
        PokemonType::Steel,
    ],
    ineffective: &[
        PokemonType::Normal,
        PokemonType::Fire,
        PokemonType::Poison,
        PokemonType::Flying,
    ],
};

pub const GHOST: TypeInfo = TypeInfo {
    type_id: PokemonType::Ghost,
    strengths: &[
        PokemonType::Psychic,
        PokemonType::Ghost,
    ],
    weaknesses: &[
        PokemonType::Ghost,
        PokemonType::Dark,
    ],
    ineffective: &[
        PokemonType::Normal,
        PokemonType::Fighting,
        PokemonType::Poison,
        PokemonType::Bug,
    ],
};

pub const DRAGON: TypeInfo = TypeInfo {
    type_id: PokemonType::Dragon,
    strengths: &[PokemonType::Dragon],
    weaknesses: &[
        PokemonType::Ice,
        PokemonType::Dragon,
        PokemonType::Fairy,
    ],
    ineffective: &[
        PokemonType::Fire,
        PokemonType::Water,
        PokemonType::Electric,
        PokemonType::Grass,
    ],
};

pub const DARK: TypeInfo = TypeInfo {
    type_id: PokemonType::Dark,
    strengths: &[
        PokemonType::Psychic,
        PokemonType::Ghost,
    ],
    weaknesses: &[
        PokemonType::Fighting,
        PokemonType::Bug,
        PokemonType::Fairy,
    ],
    ineffective: &[
        PokemonType::Psychic,
        PokemonType::Ghost,
        PokemonType::Dark,
    ],
};

pub const STEEL: TypeInfo = TypeInfo {
    type_id: PokemonType::Steel,
    strengths: &[
        PokemonType::Ice,
        PokemonType::Rock,
        PokemonType::Fairy,
    ],
    weaknesses: &[
        PokemonType::Fire,
        PokemonType::Fighting,
        PokemonType::Ground,
    ],
    ineffective: &[
        PokemonType::Normal,
        PokemonType::Grass,
        PokemonType::Ice,
        PokemonType::Flying,
        PokemonType::Psychic,
        PokemonType::Bug,
        PokemonType::Rock,
        PokemonType::Dragon,
        PokemonType::Steel,
        PokemonType::Fairy,
    ],
};

pub const FAIRY: TypeInfo = TypeInfo {
    type_id: PokemonType::Fairy,
    strengths: &[
        PokemonType::Fighting,
        PokemonType::Dragon,
        PokemonType::Dark,
    ],
    weaknesses: &[
        PokemonType::Poison,
        PokemonType::Steel,
    ],
    ineffective: &[
        PokemonType::Dragon,
        PokemonType::Fighting,
        PokemonType::Bug,
        PokemonType::Dark,
    ],
};

pub const ALL_TYPE_INFO: [TypeInfo; 18] = [
    NORMAL,
    FIRE,
    WATER,
    ELECTRIC,
    GRASS,
    ICE,
    FIGHTING,
    POISON,
    GROUND,
    FLYING,
    PSYCHIC,
    BUG,
    ROCK,
    GHOST,
    DRAGON,
    DARK,
    STEEL,
    FAIRY,
];

pub fn type_info_map() -> HashMap<PokemonType, TypeInfo> {
    ALL_TYPE_INFO.iter()
        .map(|&t| (t.type_id, t))
        .collect()
}
    
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct TypeSet {
    pub primary: PokemonType,
    pub secondary: Option<PokemonType>,
}
impl fmt::Display for TypeSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.secondary.is_none() {
            return write!(f, "{:?}", self.primary);
        }
        else
        {
            return write!(f, "{:?}/{:?}", self.primary, self.secondary.clone().unwrap());
        }
    }
}
impl TypeSet {
    pub fn strengths(&self) -> HashSet<PokemonType> {
        let mut strengths = HashSet::<PokemonType>::new();
        let type_map = type_info_map();

        strengths = type_map.get(&self.primary)
            .unwrap()
            .strengths
            .iter()
            .map(|&x| x)
            .collect();

        if self.secondary.is_none() { return strengths; }

        let secondary = self.secondary.clone().unwrap();

        for strength in type_map.get(&secondary)
            .unwrap()
            .strengths {
                strengths.insert(*strength);
            }

        strengths
    }
    pub fn weaknesses(&self) -> HashSet<PokemonType> {
        let mut weaknesses = HashSet::<PokemonType>::new();
        let type_map = type_info_map();

        weaknesses = type_map.get(&self.primary)
            .unwrap()
            .weaknesses
            .iter()
            .map(|&x| x)
            .collect();

        if self.secondary.is_none() { return weaknesses; }

        let secondary = self.secondary.clone().unwrap();

        for weakness in type_map.get(&secondary)
            .unwrap()
            .weaknesses {
                weaknesses.insert(*weakness);
            }

        let ineffective_set = self.ineffective();

        weaknesses
            .difference(&ineffective_set)
            .map(|&x| x)
            .collect()
    }
    pub fn ineffective(&self) -> HashSet<PokemonType> {
        let mut ineffective = HashSet::<PokemonType>::new();
        let type_map = type_info_map();

        ineffective = type_map.get(&self.primary)
            .unwrap()
            .ineffective
            .iter()
            .map(|&x| x)
            .collect();

        if self.secondary.is_none() { return ineffective; }

        let secondary = self.secondary.clone().unwrap();

        for ineffective_type in type_map.get(&secondary)
            .unwrap()
            .ineffective {
                ineffective.insert(*ineffective_type);
            }

        ineffective
    }
}

pub fn type_coverage(type_id: PokemonType, current_coverage: Option<HashSet<PokemonType>>)
                     -> HashSet<PokemonType> {
    let type_map = type_info_map();
    let coverage: HashSet<PokemonType> = type_map.get(&type_id)
        .unwrap()
        .strengths
        .iter()
        .map(|&x| x)
        .collect();

    if current_coverage.is_none() { return coverage; }

    coverage.difference(&current_coverage.unwrap())
        .map(|&t| t)
        .collect()
}
