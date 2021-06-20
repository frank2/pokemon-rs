#![recursion_limit="256"]

use std::collections::{HashMap, HashSet};
use std::io;

use pokemon::{Pokemon, Team};
use pokemon::monsters;
use pokemon::pokedex;
use pokemon::types::PokemonType;

struct TeamBuilder {
    team: Team,
    dex: pokedex::Pokedex,
    rejected: Vec<HashSet<Pokemon>>,
    fixed: HashSet<Pokemon>,
    banned: HashSet<Pokemon>,
    
    pub covered: HashSet<PokemonType>,
    pub uncovered: HashSet<PokemonType>,
    pub resistance: HashSet<PokemonType>,
    pub bad_weaknesses: HashSet<PokemonType>,
    pub redundancy_map: Vec<(PokemonType, usize)>,
    pub weakness_map: Vec<(PokemonType, usize)>,
    pub vulnerability_map: Vec<(PokemonType, usize)>,
    pub resistance_gap_map: Vec<(PokemonType, usize)>,
}
impl TeamBuilder {
    pub fn new(dex: &pokedex::Pokedex, fixed: Option<HashSet<Pokemon>>, banned: Option<HashSet<Pokemon>>) -> Self {
        let mut result = TeamBuilder {
            team: Team::new(dex.rules),
            dex: dex.clone(),
            rejected: Vec::<HashSet<Pokemon>>::new(),
            fixed: HashSet::<Pokemon>::new(),
            banned: HashSet::<Pokemon>::new(),
            
            covered: HashSet::<PokemonType>::new(),
            uncovered: dex.rules.to_set(),
            resistance: HashSet::<PokemonType>::new(),
            bad_weaknesses: HashSet::<PokemonType>::new(),
            redundancy_map: Vec::<(PokemonType, usize)>::new(),
            weakness_map: Vec::<(PokemonType, usize)>::new(),
            vulnerability_map: Vec::<(PokemonType, usize)>::new(),
            resistance_gap_map: Vec::<(PokemonType, usize)>::new(),
        };

        if fixed.is_some() {
            let fixed_set = fixed.unwrap();

            for pkmn in &fixed_set {
                result.insert(&pkmn);
            }

            result.fixed = fixed_set;
        }

        if banned.is_some() {
            result.banned = banned.unwrap();
        }

        result.recalculate();

        result
    }
    pub fn get_team(&self) -> Team {
        self.team.clone()
    }
    pub fn insert(&mut self, member: &Pokemon) -> bool {
        if self.banned.contains(&member) { return false; }
        
        let result = self.team.insert(&member);
        self.recalculate();
        result
    }
    pub fn remove(&mut self, target: &Pokemon) -> bool {
        if self.fixed.contains(&target) { return false; }
        
        let result = self.team.remove(&target);
        self.recalculate();
        result
    }
    pub fn replace(&mut self, target: &Pokemon, replacement: &Pokemon) -> bool {
        if self.banned.contains(&target) || self.fixed.contains(&replacement) { return false; }
        
        let result = self.team.replace(&target, &replacement);
        self.recalculate();
        result
    }
    pub fn recalculate(&mut self) {
        self.covered = self.team.coverage();
        self.uncovered = self.team.uncoverage();
        self.resistance = self.team.resistance();
        self.redundancy_map = self.team.redundancy_map();
        self.weakness_map = self.team.weakness_map();
        self.vulnerability_map = self.team.vulnerability_map();
        self.resistance_gap_map = self.team.resistance_gap_map();
        self.bad_weaknesses = self.weakness_map.iter()
            .filter(|x| x.1 >= 2)
            .map(|x| x.0)
            .collect();
    }
    pub fn add_from_vulnerabilities(&mut self) -> Option<Team> {
        let type_map = self.dex.type_map();
        let vuln_map = self.vulnerability_map.clone();
        
        if self.uncovered.len() == 0 { return None; }

        for (weakness,_) in vuln_map {
            let mut vulns = type_map.get(&weakness).unwrap().clone();
            vulns.sort();
            vulns.reverse();
            
            for pkmn in vulns {
                if self.banned.contains(&pkmn) { continue; }

                /* if the weakness is actually a weakness of this pokemon, it's not suitable */
                if pkmn.typeset
                    .weaknesses(&self.dex.rules)
                    .contains(&weakness) { continue; }
                
                /* if there are not enough Pokemon on the team, just add them */
                if self.insert(&pkmn) {
                    if self.rejected.contains(&self.team.to_set()) { self.remove(&pkmn); continue; }

                    self.rejected.push(self.team.to_set());
                    return Some(self.team.clone());
                }
                /* if the team is full:
                      * calculate the redundant coverage of each Pokemon
                      * for each redundant Pokemon, replace them with the new Pokemon
                      * if coverage increases, return the new team.
                 */
                else {
                    let redundancy_map: Vec<(PokemonType, usize)> = self.redundancy_map
                        .iter()
                        .filter(|x| x.1 >= 2)
                        .map(|&x| x)
                        .collect();
                    let mut redundancy_count = HashMap::<Pokemon, usize>::new();

                    for (redundant_type,_) in redundancy_map {
                        for redundant_pkmn in self.team.covers(redundant_type) {
                            let entry = redundancy_count.get(&redundant_pkmn);
                            let mut result = 1;

                            if entry.is_some() { result += entry.unwrap(); }

                            redundancy_count.insert(redundant_pkmn, result);
                        }
                    }

                    let mut redundancy_vec: Vec<(Pokemon, usize)> = redundancy_count.into_iter().collect();
                    redundancy_vec.sort_by(|a,b| b.1.cmp(&a.1));

                    for (redundant_pkmn,_) in redundancy_vec {
                        if self.fixed.contains(&redundant_pkmn) { continue; }
                        
                        let mut new_team = self.team.clone();
                        new_team.replace(&redundant_pkmn,&pkmn);
                        
                        if self.rejected.contains(&new_team.to_set()) { continue; }

                        if new_team.coverage().len() > self.covered.len() {
                            self.rejected.push(self.team.to_set());
                            self.team = new_team;
                            self.recalculate();

                            return Some(self.team.clone());
                        }
                    }
                }
            }
        }

        None
    }
    pub fn remove_by_weaknesses(&mut self) -> Option<Team> {
        let weakness_map: Vec<(PokemonType, usize)> = self.weakness_map.iter()
            .filter(|x| x.1 >= 2)
            .map(|&x| x)
            .collect();
        let mut weakness_count = HashMap::<Pokemon, usize>::new();

        if weakness_map.len() == 0 { return None; }

        for (weakness,_) in weakness_map {
            let shared_weakness = self.team.weak_to(weakness);

            for weak_pkmn in &shared_weakness {
                if self.fixed.contains(&weak_pkmn) { continue; }
                
                let entry = weakness_count.get(&weak_pkmn);
                let mut result = 1;

                if entry.is_some() { result += entry.unwrap(); }

                weakness_count.insert(weak_pkmn.clone(), result);
            }
        }

        let mut weakness_vec: Vec<(Pokemon, usize)> = weakness_count.iter()
            .map(|x| (x.0.clone(),*x.1))
            .collect();
        weakness_vec.sort_by(|a,b| b.1.cmp(&a.1));

        let (pkmn,_) = weakness_vec.iter().next().unwrap();

        self.remove(&pkmn);
        self.rejected.push(self.team.to_set());

        Some(self.team.clone())
    }
    pub fn remove_by_redundancy(&mut self) -> Option<Team> {
        let redundancy_map: Vec<(PokemonType, usize)> = self.redundancy_map.iter()
            .filter(|x| x.1 >= 2)
            .map(|&x| x)
            .collect();
        let mut redundancy_count = HashMap::<Pokemon, usize>::new();
        
        if redundancy_map.len() == 0 { return None; }

        for (redundancy,_) in redundancy_map {
            let redundant_members = self.team.covers(redundancy);

            for redundant_pkmn in &redundant_members {
                if self.fixed.contains(&redundant_pkmn) { continue; }
                
                let entry = redundancy_count.get(&redundant_pkmn);
                let mut result = 1;

                if entry.is_some() { result += entry.unwrap(); }

                redundancy_count.insert(redundant_pkmn.clone(), result);
            }
        }

        let mut redundancy_vec: Vec<(Pokemon, usize)> = redundancy_count.iter()
            .map(|x| (x.0.clone(),*x.1))
            .collect();
        redundancy_vec.sort_by(|a,b| b.1.cmp(&a.1));

        let redundancy = redundancy_vec.iter().next();

        if redundancy.is_none() { return None; }

        let (pkmn,_) = redundancy.unwrap();

        self.remove(&pkmn);
        self.rejected.push(self.team.to_set());

        Some(self.team.clone())
    }
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    let stdin = io::stdin();

    let pokedex = pokedex::REDBLUEYELLOW;

    /*
    let weakness_map = pokedex.weakness_map();
    let resistance_map = pokedex.resistance_map();
    
    println!("Resistance map: {:?}", resistance_map.get(&PokemonType::Psychic));
    println!("Gengar: {}", pokedex.by_name("gengar").unwrap().pop().unwrap());
     */

    let fixed_set: HashSet<Pokemon> = [monsters::CHARIZARD].iter().cloned().collect();
    let banned_set: HashSet<Pokemon> = [
        monsters::GENGAR,
        monsters::MACHAMP,
        monsters::GOLEM,
        monsters::VENUSAUR,
        monsters::BLASTOISE,
    ].iter().cloned().collect();
    let mut builder = TeamBuilder::new(&pokedex, Some(fixed_set), Some(banned_set));
    
    let mut result = builder.add_from_vulnerabilities();

    while result.is_some() {
        println!("Team: {}", result.unwrap());
        stdin.read_line(&mut line).ok();

        result = builder.add_from_vulnerabilities();

        if result.is_none() {
            let redundancy = builder.redundancy_map.iter()
                .filter(|x| x.1 >= 2)
                .count();
            let weaknesses = builder.weakness_map.iter()
                .filter(|x| x.1 >= 2)
                .count();

            if redundancy >= weaknesses {
                result = builder.remove_by_redundancy();
            }
            else {
                result = builder.remove_by_weaknesses();
            }
        }
    }

    println!("Team: {}", builder.get_team());
    println!("Gap map: {:?}", builder.resistance_gap_map);

    Ok(())
}
