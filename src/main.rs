#![recursion_limit="256"]

mod types;
mod pokedex;
mod pokemon;

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, Read};
use std::iter::FromIterator;

use types::*;
use pokemon::*;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Team {
    pub team: Vec<Pokemon>,
    pub covered: HashSet<PokemonType>,
    pub uncovered: HashSet<PokemonType>,
    pub bad_weaknesses: HashSet<PokemonType>,
    pub redundancy: Vec<(PokemonType, usize)>,
    pub weakness: Vec<(PokemonType, usize)>,
}
impl Team {
    pub fn new() -> Self {
        Team {
            team: Vec::<Pokemon>::new(),
            covered: HashSet::<PokemonType>::new(),
            uncovered: HashSet::<PokemonType>::from_iter(ALL_POKEMON_TYPES.into_iter().map(|&x| x)),
            bad_weaknesses: HashSet::<PokemonType>::new(),
            redundancy: Vec::<(PokemonType, usize)>::new(),
            weakness: Vec::<(PokemonType, usize)>::new(),
        }
    }
    pub fn slots(&self) -> usize {
        6 - self.team.len()
    }
    pub fn insert(&mut self, member: Pokemon) -> bool
    {
        if self.slots() == 0 { return false; }
        
        self.team.push(member);
        self.recalculate();
        true
    }
    pub fn remove(&mut self, target: Pokemon) -> bool {
        if !self.team.contains(&target) { return false; }
        
        let index = self.team.iter().position(|&x| x == target).unwrap();
        self.team.remove(index);
        self.recalculate();
        
        true
    }
    pub fn replace(&mut self, target: Pokemon, replacement: Pokemon) -> bool {
        if !self.team.contains(&target) { return false; }

        self.remove(target);
        self.team.push(replacement);
        self.recalculate();
        
        true
    }
    pub fn as_set(&self) -> HashSet<Pokemon> {
        self.team.iter().map(|&x| x).collect()
    }
    pub fn coverage(&mut self) -> HashSet<PokemonType> {
        self.covered = HashSet::<PokemonType>::new();

        for pkmn in &self.team {
            for type_id in pkmn.typeset.strengths() {
                self.covered.insert(type_id);
            }
        }

        self.covered.clone()
    }
    pub fn uncoverage(&mut self) -> HashSet<PokemonType> {
        self.uncovered = HashSet::<PokemonType>::from_iter(ALL_POKEMON_TYPES.iter().map(|&x| x));
        self.uncovered = self.uncovered.difference(&self.covered)
            .into_iter()
            .map(|&x| x)
            .collect();

        self.uncovered.clone()
    }
    pub fn redundancy_map(&mut self) -> Vec<(PokemonType, usize)> {
        let mut redundancy_map = HashMap::<PokemonType, usize>::new();

        for pkmn in &self.team {
            let strengths = pkmn.typeset.strengths();

            for strength in strengths {
                let mut entry = redundancy_map.get(&strength);

                if entry.is_none() { redundancy_map.insert(strength, 1); }
                else { redundancy_map.insert(strength, entry.unwrap()+1); }
            }
        }

        let mut result: Vec<(PokemonType, usize)> = redundancy_map.into_iter().collect();
        result.sort_by(|a,b| b.1.cmp(&a.1));
        self.redundancy = result;
        self.redundancy.clone()
    }
    pub fn weakness_map(&mut self) -> Vec<(PokemonType, usize)> {
        let mut weakness_map = HashMap::<PokemonType, usize>::new();

        for pkmn in &self.team {
            let weaknesses = pkmn.typeset.weaknesses();

            for weakness in weaknesses {
                let mut entry = weakness_map.get(&weakness);

                if entry.is_none() { weakness_map.insert(weakness, 1); }
                else { weakness_map.insert(weakness, entry.unwrap()+1); }
            }
        }

        let mut result: Vec<(PokemonType, usize)> = weakness_map.into_iter().collect();
        result.sort_by(|a,b| b.1.cmp(&a.1));
        self.weakness = result;

        self.weakness.clone()
    }
    pub fn recalculate(&mut self) {
        self.coverage();
        self.uncoverage();
        self.redundancy_map();
        self.weakness_map();
        self.bad_weaknesses = self.weakness
            .iter()
            .filter(|x| x.1 >= 2)
            .map(|x| x.0)
            .collect();
    }
    pub fn covers(&self, type_id: PokemonType) -> Vec<Pokemon> {
        let mut result = Vec::<Pokemon>::new();
        
        for pkmn in &self.team {
            if pkmn.typeset.strengths().contains(&type_id) { result.push(*pkmn); }
        }

        result
    }
    pub fn weak_to(&self, type_id: PokemonType) -> Vec<Pokemon> {
        let mut result = Vec::<Pokemon>::new();

        for pkmn in &self.team {
            if pkmn.typeset.weaknesses().contains(&type_id) { result.push(*pkmn); }
        }

        result
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

fn main() -> io::Result<()> {
    /* start with three sets: the set of types covered, the set of types uncovered, and the set of rejected teams
       calculate the type map of the gen4 pokedex
       while uncovered > 0:
          create a mapping of PokemonType to u8 to count weaknesses
          calculate the weaknesses of the currently uncovered set
          sort the weaknesses by appearance
          for each type:
             for each pokemon of that type:
                if the team is empty, add it and continue
                   if team is rejected, remove it instead
                if the team is full:
                   calculate the redundancy map of the old team, filtering for redundancy > 1
                   create a mapping of Pokemon to usize to count appearances
                   for each redundant type:
                      get the pokemon who cover this type, count them in the mapping
                   sort the mapping by highest appearance
                   for each redundant team member:
                      replace the team member
                      if team is rejected, get next team member
                      calculate the new coverage
                      if greater coverage accomplished:
                         reject old team
                         update covered, uncovered and bad weaknesses
                         select this team and continue

          if we get here, that means we couldn't find a Pokemon to increase team coverage, so we need to remove someone
          calculate the weakness map of the current team, filtering for weakness weight > 1
          create a mapping of Pokemon to u8 to count appearances
          for each weakness entry:
             get the pokemon weak to this type, count them in the mapping
          sort the mapping by highest appearance
          remove that member from the team and continue
     */
    let mut team = Team::new();
    let mut rejected = Vec::<HashSet<Pokemon>>::new();
    let gen4_type_map = pokedex::GEN4.type_map();
    let type_info_map = type_info_map();

    let mut line = String::new();
    let mut stdin = io::stdin();

    loop {
        while team.uncovered.len() > 0 {
            let mut opposing_weakness_map = HashMap::<PokemonType, usize>::new();
            let mut opposing_weakness_set = HashSet::<PokemonType>::new();
            let mut next_iter: bool = false;
            
            for type_id in &team.uncovered {
                for weakness in type_info_map.get(type_id)
                    .unwrap()
                    .weaknesses {
                        let mut entry = opposing_weakness_map.get(weakness);
                        
                        opposing_weakness_set.insert(*weakness);
                        
                        if entry.is_none() { opposing_weakness_map.insert(*weakness, 1); }
                        else { opposing_weakness_map.insert(*weakness, entry.unwrap()+1); }
                    }
            }
            
            let mut opposing_weakness_vec: Vec<(PokemonType, usize)> = opposing_weakness_map.into_iter().collect();
            opposing_weakness_vec.sort_by(|a,b| b.1.cmp(&a.1));
            
            for (weakness,_) in opposing_weakness_vec {
                for pkmn in gen4_type_map.get(&weakness).unwrap() {
                    if pkmn.typeset
                        .weaknesses()
                        .intersection(&team.bad_weaknesses)
                        .count() > 0 { continue; }
                    
                    if team.insert(*pkmn) {
                        if rejected.contains(&team.as_set()) { team.remove(*pkmn); continue; }
                        next_iter = true;
                        break;
                    }
                    else {
                        let redundancy_map: Vec<(PokemonType, usize)> = team.redundancy
                            .iter()
                            .filter(|x| x.1 >= 2)
                            .map(|&x| x)
                            .collect();
                        let mut redundancy_count = HashMap::<Pokemon, usize>::new();

                        for (redundant_type,_) in redundancy_map {
                            for redundant_pkmn in team.covers(redundant_type) {
                                let mut entry = redundancy_count.get(&redundant_pkmn);

                                if entry.is_none() { redundancy_count.insert(redundant_pkmn, 1); }
                                else { redundancy_count.insert(redundant_pkmn, entry.unwrap()+1); }
                            }
                        }

                        let mut redundancy_vec: Vec<(Pokemon, usize)> = redundancy_count.into_iter().collect();
                        redundancy_vec.sort_by(|a,b| b.1.cmp(&a.1));

                        for (redundant_pkmn,_) in redundancy_vec {
                            let mut new_team = team.clone();
                            new_team.replace(redundant_pkmn,*pkmn);

                            if rejected.contains(&new_team.as_set()) { continue; }

                            if new_team.covered.len() > team.covered.len() {
                                rejected.push(team.as_set());
                                team = new_team;
                                next_iter = true;
                                
                                break;                                
                            }
                        }
                    }

                    if next_iter { break; }
                }
                
                if next_iter { break; }
            }
            
            rejected.push(team.as_set());

            println!("Team (iter): {}", team);
            stdin.read_line(&mut line);
            
            if next_iter { continue; }

            let mut is_rejected: bool = true;
            
            while is_rejected && team.slots() > 0 {
                let weakness_set: HashSet<PokemonType> = team.weakness.iter()
                    .map(|x| x.0)
                    .collect();
                let mut weakness_count = HashMap::<Pokemon, usize>::new();

                for pkmn in &team.team {
                    weakness_count.insert(*pkmn, pkmn.typeset.weaknesses()
                                          .intersection(&weakness_set)
                                          .count());
                }

                let mut weakness_list: Vec<(Pokemon, usize)> = weakness_count.into_iter().collect();
                weakness_list.sort_by(|a,b| b.1.cmp(&a.1));
                let (pkmn, count) = weakness_list.iter().next().unwrap();

                team.remove(*pkmn);
                is_rejected = rejected.contains(&team.as_set());
            }

            if !is_rejected { continue; }

            println!("All teams rejected.");
            return Ok(());
        }
        
        println!("Team (finished): {}", team);
        stdin.read_line(&mut line);

        if !rejected.contains(&team.as_set()) {
            rejected.push(team.as_set());
        }

        let weakness_set: HashSet<PokemonType> = team.weakness.iter()
            .filter(|x| x.1 >= 2)
            .map(|x| x.0)
            .collect();
        let mut weakness_count = HashMap::<Pokemon, usize>::new();

        for pkmn in &team.team {
            weakness_count.insert(*pkmn, pkmn.typeset.weaknesses()
                                  .intersection(&weakness_set)
                                  .count());
        }

        let mut weakness_list: Vec<(Pokemon, usize)> = weakness_count.into_iter().collect();
        weakness_list.sort_by(|a,b| b.1.cmp(&a.1));
        let (pkmn, count) = weakness_list.iter().next().unwrap();
                
        println!("Weakness set: {:?}", weakness_set);
        println!("{} weaknesses: {}", count, pkmn);

        team.remove(*pkmn);
    }    
    /*
    let strengths = INFERNAPE.typeset.strengths();
    let weaknesses = QUAGSIRE.typeset.weaknesses();

    println!("Infernape strengths: {:?}", strengths);
    println!("Quagsire weaknesses: {:?}", weaknesses);
    println!("Gen4 Fire types: {:?}", pokedex::GEN4.filter(PokemonType::Fire));
     */
}
