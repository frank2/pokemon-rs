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
impl PokemonType {
    pub fn from_string(typestr: &String) -> Result<Self, &'static str> {
        match typestr.to_lowercase().as_str() {
            "normal" => Ok(PokemonType::Normal),
            "fire" => Ok(PokemonType::Fire),
            "water" => Ok(PokemonType::Water),
            "electric" => Ok(PokemonType::Electric),
            "grass" => Ok(PokemonType::Grass),
            "ice" => Ok(PokemonType::Ice),
            "fighting" => Ok(PokemonType::Fighting),
            "poison" => Ok(PokemonType::Poison),
            "ground" => Ok(PokemonType::Ground),
            "flying" => Ok(PokemonType::Flying),
            "psychic" => Ok(PokemonType::Psychic),
            "bug" => Ok(PokemonType::Bug),
            "rock" => Ok(PokemonType::Rock),
            "ghost" => Ok(PokemonType::Ghost),
            "dragon" => Ok(PokemonType::Dragon),
            "dark" => Ok(PokemonType::Dark),
            "steel" => Ok(PokemonType::Steel),
            "fairy" => Ok(PokemonType::Fairy),
            _ => Err("no such Pokemon type"),
        }
    }
    pub fn to_string(&self) -> String {
        String::from(format!("{:?}", self))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct TypeInfo {
    pub type_id: PokemonType,
    pub strengths: &'static [PokemonType],
    pub weaknesses: &'static [PokemonType],
    pub ineffective: &'static [PokemonType],
    pub no_effect: &'static [PokemonType],
}

pub const NORMAL: TypeInfo = TypeInfo {
    type_id: PokemonType::Normal,
    strengths: &[],
    weaknesses: &[PokemonType::Fighting],
    ineffective: &[],
    no_effect: &[PokemonType::Ghost],
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
    no_effect: &[],
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
    no_effect: &[],
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
    no_effect: &[],
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
    no_effect: &[],
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
    no_effect: &[],
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
    no_effect: &[],
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
    no_effect: &[],
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
        PokemonType::Poison,
        PokemonType::Rock,
    ],
    no_effect: &[PokemonType::Electric],
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
        PokemonType::Grass,
        PokemonType::Fighting,
        PokemonType::Bug,
    ],
    no_effect: &[PokemonType::Ground],
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
    no_effect: &[],
};

pub const PSYCHIC_GEN1: TypeInfo = TypeInfo {
    type_id: PokemonType::Psychic,
    strengths: &[
        PokemonType::Fighting,
        PokemonType::Poison,
    ],
    weaknesses: &[PokemonType::Bug],
    ineffective: &[
        PokemonType::Fighting,
        PokemonType::Psychic,
    ],
    no_effect: &[PokemonType::Ghost],
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
    no_effect: &[],
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
    no_effect: &[],
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
        PokemonType::Poison,
        PokemonType::Bug,
    ],
    no_effect: &[
        PokemonType::Normal,
        PokemonType::Fighting,
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
    no_effect: &[],
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
        PokemonType::Ghost,
        PokemonType::Dark,
    ],
    no_effect: &[PokemonType::Psychic],
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
    no_effect: &[PokemonType::Poison],
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
        PokemonType::Fighting,
        PokemonType::Bug,
        PokemonType::Dark,
    ],
    no_effect: &[PokemonType::Dragon],
};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct TypeRules {
    ruleset: &'static [TypeInfo],
}
impl TypeRules {
    pub fn iter(&self) -> TypeRulesIterator<'_> {
        TypeRulesIterator {
            rules: &self,
            index: 0,
        }
    }
    pub fn to_set(&self) -> HashSet<PokemonType> {
        self.iter()
            .map(|&x| x.type_id)
            .collect()
    }
    pub fn to_map(&self) -> HashMap<PokemonType, TypeInfo> {
        self.iter()
            .map(|&t| (t.type_id, t))
            .collect()
    }
}
impl<'a> IntoIterator for &'a TypeRules {
    type Item = &'a TypeInfo;
    type IntoIter = TypeRulesIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TypeRulesIterator {
            rules: &self,
            index: 0,
        }
    }
}

pub struct TypeRulesIterator<'a> {
    rules: &'a TypeRules,
    index: usize,
}
impl<'a> Iterator for TypeRulesIterator<'a> {
    type Item = &'a TypeInfo;

    fn next(&mut self) -> Option<&'a TypeInfo> {
        if self.index >= self.rules.ruleset.len() { return None; }

        let element = &self.rules.ruleset[self.index];
        self.index += 1;

        Some(element)
    }
}

pub const GEN1_RULES: TypeRules = TypeRules {
    ruleset: &[NORMAL,
               FIRE,
               WATER,
               ELECTRIC,
               GRASS,
               ICE,
               FIGHTING,
               POISON,
               GROUND,
               FLYING,
               PSYCHIC_GEN1,
               BUG,
               ROCK,
               GHOST,
               DRAGON],
};

pub const GEN2_RULES: TypeRules = TypeRules {
    ruleset: &[NORMAL,
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
               STEEL],
};

pub const GEN6_RULES: TypeRules = TypeRules {
    ruleset: &[NORMAL,
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
               FAIRY],
};
    
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct TypeData {
    pub primary: PokemonType,
    pub secondary: Option<PokemonType>,
}
impl TypeData {
    pub fn strengths(&self, rules: &TypeRules) -> HashSet<PokemonType> {
        let type_map = GEN6_RULES.to_map();
        let mut strengths: HashSet<PokemonType> = type_map.get(&self.primary)
            .unwrap()
            .strengths
            .iter()
            .map(|&x| x)
            .collect();

        if self.secondary.is_none() {
            return strengths.intersection(&rules.to_set())
                .map(|&x| x)
                .collect();
        }

        let secondary = self.secondary.clone().unwrap();

        for strength in type_map.get(&secondary)
            .unwrap()
            .strengths {
                strengths.insert(*strength);
            }

        strengths.intersection(&rules.to_set())
            .map(|&x| x)
            .collect()
    }
    pub fn weaknesses(&self, rules: &TypeRules) -> HashSet<PokemonType> {
        let type_map = GEN6_RULES.to_map();
        let mut weaknesses: HashSet<PokemonType> = type_map.get(&self.primary)
            .unwrap()
            .weaknesses
            .iter()
            .map(|&x| x)
            .collect();

        if self.secondary.is_none() {
            return weaknesses.intersection(&rules.to_set())
                .map(|&x| x)
                .collect();
        }

        let secondary = self.secondary.clone().unwrap();

        for weakness in type_map.get(&secondary)
            .unwrap()
            .weaknesses {
                weaknesses.insert(*weakness);
            }

        let resistance_set = self.resistance(rules);

        weaknesses = weaknesses
            .difference(&resistance_set)
            .map(|&x| x)
            .collect();

        weaknesses.intersection(&rules.to_set())
            .map(|&x| x)
            .collect()
    }
    pub fn ineffective(&self, rules: &TypeRules) -> HashSet<PokemonType> {
        let type_map = GEN6_RULES.to_map();
        let mut ineffective: HashSet<PokemonType> = type_map.get(&self.primary)
            .unwrap()
            .ineffective
            .iter()
            .map(|&x| x)
            .collect();

        if self.secondary.is_none() {
            return ineffective.intersection(&rules.to_set())
                .map(|&x| x)
                .collect();
        }

        let secondary = self.secondary.clone().unwrap();

        for ineffective_type in type_map.get(&secondary)
            .unwrap()
            .ineffective {
                ineffective.insert(*ineffective_type);
            }

        ineffective.intersection(&rules.to_set())
            .map(|&x| x)
            .collect()
    }
    pub fn no_effect(&self, rules: &TypeRules) -> HashSet<PokemonType> {
        let type_map = GEN6_RULES.to_map();
        let mut no_effect: HashSet<PokemonType> = type_map.get(&self.primary)
            .unwrap()
            .no_effect
            .iter()
            .map(|&x| x)
            .collect();

        if self.secondary.is_none() {
            return no_effect.intersection(&rules.to_set())
                .map(|&x| x)
                .collect();
        }

        let secondary = self.secondary.clone().unwrap();

        for no_effect_type in type_map.get(&secondary)
            .unwrap()
            .no_effect {
                no_effect.insert(*no_effect_type);
            }

        no_effect.intersection(&rules.to_set())
            .map(|&x| x)
            .collect()
    }
    pub fn resistance(&self, rules: &TypeRules) -> HashSet<PokemonType> {
        self.ineffective(rules)
            .union(&self.no_effect(rules))
            .map(|&x| x)
            .collect()
    }
}
impl fmt::Display for TypeData {
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
