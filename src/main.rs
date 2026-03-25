mod entity;
mod effect;
mod command;
mod engine;

use entity::{Entity, Team};
use effect::Poison;
use engine::Engine;

fn main() {
    let mut engine = Engine::new(10);

    let player = Entity::new("Герой", 100, 15, Team::Player);
    let mut enemy = Entity::new("Гоблин", 80, 12, Team::Enemy);

    // Навешиваем яд на гоблина
    let poison = Box::new(Poison::new(3, 5));
    enemy.add_effect(poison);

    engine.add_entity(player);
    engine.add_entity(enemy);

    engine.run();
}