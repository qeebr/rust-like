extern crate rand;

use rand::Rng;

use super::super::character::entity::*;
use super::super::log::*;

pub struct Fight;

impl Fight {

    ///calculates a weapon hit.
    /// crit_chance from 0 to 100.
    pub fn weapon_hit<T: Generator>(log : &mut Log, generator: T, me: &Entity, enemy: &mut Entity, crit_chance: i32) {
        if enemy.is_death() {
            return;
        }

        let my_dep_stats = me.calculate_stats();
        let enemy_dep_stats = enemy.calculate_stats();

        let weapon = me.weapon.clone();
        let weapon_damage = weapon.get_damage();
        let attack_bonus = my_dep_stats.strength - enemy_dep_stats.defense;

        let actual_damage = generator.generate(weapon_damage.0 + attack_bonus, weapon_damage.1 + attack_bonus);

        let crit = generator.generate(0, 100) <= crit_chance;
        let actual_damage = if crit {
            actual_damage * 2
        } else {
            actual_damage
        };


        if actual_damage > 0 {
            enemy.current_life -= actual_damage;

            let crit_prefix = if crit {
                "CRIT! "
            } else {
                ""
            }.to_string();

            if enemy.is_death() {
                log.add_message(crit_prefix + &format!("{} killed {}!", me.name, enemy.name));
            } else {
                log.add_message(crit_prefix + &format!("{} hit {} with {}!", me.name, enemy.name, actual_damage));
            }

        } else {
            log.add_message(format!("{} missed {}", me.name, enemy.name));
        }

    }
}

pub struct RndGenerator;

impl Generator for RndGenerator {
    fn generate(&self, min_inclusive: i32, max_inclusive: i32) -> i32 {
        //gen_range generates min_inclusive to max_exclusive.
        rand::thread_rng().gen_range(min_inclusive, max_inclusive + 1)
    }
}

pub trait Generator {
    fn generate(&self, min_inclusive: i32, max_inclusive: i32) -> i32;
}

#[test]
fn test_damage() {
    let mut log = Log {messages : Vec::new()};
    let me = Entity::new(0);
    let mut enemy = Entity::new(1);

    assert_eq!(enemy.calculate_max_life(), enemy.current_life);

    Fight::weapon_hit(&mut log, RndGenerator, &me, &mut enemy, 0);

    assert!(enemy.current_life < enemy.calculate_max_life());
}

#[test]
fn test_death() {
    let mut log = Log {messages : Vec::new()};
    let me = Entity::new(0);
    let mut enemy = Entity::new(1);

    enemy.current_life = 1;

    assert ! ( !enemy.is_death());

    Fight::weapon_hit(&mut log, RndGenerator, &me, & mut enemy, 0);

    assert ! (enemy.is_death());
}
