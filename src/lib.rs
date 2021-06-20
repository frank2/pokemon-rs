#![recursion_limit="256"]

pub mod monsters;
pub mod pokedex;
pub mod types;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::types::{PokemonType, TypeData, TypeRules};

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum PokemonGame {
    RedBlueYellow,
    GoldSilverCrystal,
    RubySapphireEmerald,
    FireRedLeafGreen,
    DiamondPearl,
    Platinum,
    HeartGoldSoulSilver,
    BlackWhite,
    BlackWhite2,
    XY,
    OmegaRubyAlphaSapphire,
    SunMoon,
    UltraSunUltraMoon,
    LetsGoPikachuEevee,
    SwordShield,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Stats {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed: u8,
}
impl Stats {
    pub fn total(&self) -> usize {
        (self.hp as usize +
            self.attack as usize +
            self.defense as usize +
            self.special_attack as usize +
            self.special_defense as usize +
            self.speed as usize) as usize
    }
}
impl Ord for Stats {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total().cmp(&other.total())
    }
}
impl PartialOrd for Stats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stats<HP: {}/Attack: {}/Defense: {}/Sp. Attack: {}/Sp. Defense: {}/Speed: {}>"
               ,self.hp
               ,self.attack
               ,self.defense
               ,self.special_attack
               ,self.special_defense
               ,self.speed)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Pokemon {
    pub name: &'static str,
    pub typeset: TypeData,
    pub stats: Stats,
}
impl Ord for Pokemon {
    fn cmp(&self, other: &Self) -> Ordering {
        self.stats.cmp(&other.stats)
    }
}
impl PartialOrd for Pokemon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Pokemon<{}: {}>", self.name, self.typeset)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Team {
    team: Vec<Pokemon>,
    rules: TypeRules,
}
impl Team {
    pub fn new(rules: TypeRules) -> Self {
        Team {
            team: Vec::<Pokemon>::new(),
            rules: rules,
        }
    }
    pub fn to_set(&self) -> HashSet<Pokemon> {
        self.team.iter().cloned().collect()
    }
    pub fn get_team(&self) -> Vec<Pokemon> {
        self.team.clone()
    }
    pub fn slots(&self) -> usize {
        6 - self.team.len()
    }
    pub fn insert(&mut self, member: &Pokemon) -> bool
    {
        if self.slots() == 0 { return false; }
        
        self.team.push(member.clone());
        true
    }
    pub fn remove(&mut self, target: &Pokemon) -> bool {
        if !self.team.contains(&target) { return false; }
        
        let index = self.team.iter().position(|x| x == target).unwrap();
        self.team.remove(index);
        
        true
    }
    pub fn replace(&mut self, target: &Pokemon, replacement: &Pokemon) -> bool {
        if !self.team.contains(&target) { return false; }

        self.remove(&target);
        self.team.push(replacement.clone());
       
        true
    }
    pub fn coverage(&self) -> HashSet<PokemonType> {
        let mut covered = HashSet::<PokemonType>::new();

        for pkmn in &self.team {
            for type_id in pkmn.typeset.strengths(&self.rules) {
                covered.insert(type_id);
            }
        }

        covered
    }
    pub fn uncoverage(&self) -> HashSet<PokemonType> {
        self.rules.to_set()
            .difference(&self.coverage())
            .map(|&x| x)
            .collect()
    }
    pub fn resistance(&self) -> HashSet<PokemonType> {
        let mut result = HashSet::<PokemonType>::new();

        for pkmn in &self.team {
            for type_id in pkmn.typeset.resistance(&self.rules) {
                result.insert(type_id);
            }
        }

        result
    }
    pub fn covers(&self, type_id: PokemonType) -> Vec<Pokemon> {
        let mut result = Vec::<Pokemon>::new();
        
        for pkmn in &self.team {
            if pkmn.typeset
                .strengths(&self.rules)
                .contains(&type_id) { result.push(pkmn.clone()); }
        }

        result
    }
    pub fn weak_to(&self, type_id: PokemonType) -> Vec<Pokemon> {
        let mut result = Vec::<Pokemon>::new();

        for pkmn in &self.team {
            if pkmn.typeset
                .weaknesses(&self.rules)
                .contains(&type_id) { result.push(pkmn.clone()); }
        }

        result
    }
    pub fn resists(&self, type_id: PokemonType) -> Vec<Pokemon> {
        let mut result = Vec::<Pokemon>::new();

        for pkmn in &self.team {
            if pkmn.typeset
                .resistance(&self.rules)
                .contains(&type_id) { result.push(pkmn.clone()); }
        }

        result
    }
    pub fn redundancy_map(&self) -> Vec<(PokemonType, usize)> {
        let mut redundancy_map = HashMap::<PokemonType, usize>::new();

        for pkmn in &self.team {
            let strengths: HashSet<PokemonType> = pkmn.typeset.strengths(&self.rules);
         
            for strength in strengths {
                let entry = redundancy_map.get(&strength);
                let mut result = 1;

                if entry.is_some() { result += entry.unwrap(); }

                redundancy_map.insert(strength, result);
            }
        }

        let mut result: Vec<(PokemonType, usize)> = redundancy_map.into_iter().collect();
        result.sort_by(|a,b| b.1.cmp(&a.1));

        result
    }
    pub fn weakness_map(&self) -> Vec<(PokemonType, usize)> {
        let mut weakness_map = HashMap::<PokemonType, usize>::new();

        for pkmn in &self.team {
            let weaknesses: HashSet<PokemonType> = pkmn.typeset.weaknesses(&self.rules);

            for weakness in weaknesses {
                let entry = weakness_map.get(&weakness);
                let mut result = 1;

                if entry.is_some() { result += entry.unwrap(); }

                weakness_map.insert(weakness, result);
            }
        }

        let mut result: Vec<(PokemonType, usize)> = weakness_map.into_iter().collect();
        result.sort_by(|a,b| b.1.cmp(&a.1));

        result
    }
    pub fn vulnerability_map(&self) -> Vec<(PokemonType, usize)> {
        let mut vulnerability_map = HashMap::<PokemonType, usize>::new();
        let uncovered = self.uncoverage();
        let type_map = self.rules.to_map();
        let rules = self.rules.to_set();

        for type_id in uncovered {
            let type_entry = type_map.get(&type_id).unwrap();

            for weakness in type_entry.weaknesses.iter() {
                if !rules.contains(&weakness) { continue; }
                
                let vuln_entry = vulnerability_map.get(weakness);
                let mut result = 1;

                if vuln_entry.is_some() { result += vuln_entry.unwrap(); }

                vulnerability_map.insert(*weakness, result);
            }
        }

        let mut result: Vec<(PokemonType, usize)> = vulnerability_map.into_iter().collect();
        result.sort_by(|a,b| b.1.cmp(&a.1));

        result
    }
    pub fn resistance_gap_map(&self) -> Vec<(PokemonType, usize)> {
        let mut resistance_gap_map = HashMap::<PokemonType, usize>::new();
        let resistance = self.resistance();
        let type_map = self.rules.to_map();
        let rules = self.rules.to_set();

        for type_id in &rules {
            let type_entry = type_map.get(&type_id).unwrap();

            for resist_id in type_entry.ineffective
                .iter()
                .chain(type_entry.no_effect.iter()) {
                    if !rules.contains(&resist_id) { continue; }
                    if resistance.contains(&resist_id) { continue; }

                    let resist_entry = resistance_gap_map.get(&type_id);
                    let mut result = 1;

                    if resist_entry.is_some() { result += resist_entry.unwrap(); }

                    resistance_gap_map.insert(*type_id, result);
                }
        }

        let mut result: Vec<(PokemonType, usize)> = resistance_gap_map.into_iter().collect();
        result.sort_by(|a,b| b.1.cmp(&a.1));

        result
    }
}
impl<'a> IntoIterator for &'a Team {
    type Item = &'a Pokemon;
    type IntoIter = TeamIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TeamIterator {
            team: &self.team,
            index: 0,
        }
    }
}

pub struct TeamIterator<'a> {
    team: &'a Vec<Pokemon>,
    index: usize,
}
impl<'a> Iterator for TeamIterator<'a> {
    type Item = &'a Pokemon;

    fn next(&mut self) -> Option<&'a Pokemon> {
        if self.index >= self.team.len() { return None; }

        let element = &self.team[self.index];
        self.index += 1;

        Some(element)
    }
}
impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::from("[\n");

        for member in &self.team {
            result.push_str(&String::from(format!("\t{}\n", member)));
        }

        result.push_str("]\n");

        write!(f, "{}", result)
    }
}
