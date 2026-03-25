mod entity;
mod effect;
mod command;
mod engine;

use entity::{Entity, Team};
use effect::Poison;
use engine::Engine;
use std::io::{self, Write};

fn main() {
    println!("Добро пожаловать в мое RPG!");
    println!("Выберите противника:");
    println!("1. Гоблин");
    println!("2. Усиленный гоблин");
    print!("Ваш выбор: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let choice = input.trim();

    let (enemy_name, hp, attack) = match choice {
        "1" => ("Гоблин", 80, 12),
        "2" => ("Усиленный Гоблин", 200, 8),
        _ => {
            println!("Неверный выбор, выбран Гоблин по умолчанию.");
            ("Гоблин", 80, 12)
        }
    };

    let mut engine = Engine::new(12);

    let player = Entity::new("Герой", 100, 15, Team::Player);
    let mut enemy = Entity::new(enemy_name, hp, attack, Team::Enemy);

    // Навешиваем яд на врага
    let poison = Box::new(Poison::new(3, 5));
    enemy.add_effect(poison);

    engine.add_entity(player);
    engine.add_entity(enemy);

    engine.run();
}