use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{self, Write};
use crate::entity::{Entity, Team};
use crate::command::{Command, AttackCommand, HealCommand};
use crate::stats::BattleStats;

pub struct Engine {
    pub entities: Vec<Rc<RefCell<Entity>>>,
    pub action_queue: VecDeque<Box<dyn Command>>,
    round: u32,
    max_rounds: u32,
    stats: BattleStats,
}

impl Engine {
    pub fn new(max_rounds: u32, enemy_name: &str) -> Self {
        Self {
            entities: Vec::new(),
            action_queue: VecDeque::new(),
            round: 0,
            max_rounds,
            stats: BattleStats::new(enemy_name),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(Rc::new(RefCell::new(entity)));
    }

    pub fn pre_turn(&mut self) {
        for entity_rc in &self.entities {
            let mut entity = entity_rc.borrow_mut();
            entity.apply_effects(&mut self.stats);
            entity.tick_effects();
            entity.remove_expired_effects();
        }
    }

    pub fn gather_commands(&mut self) {
        let mut player = None;
        let mut enemy = None;
        for entity_rc in &self.entities {
            let entity = entity_rc.borrow();
            match entity.team {
                Team::Player => player = Some(entity_rc.clone()),
                Team::Enemy => enemy = Some(entity_rc.clone()),
            }
        }
        let (player, enemy) = match (player, enemy) {
            (Some(p), Some(e)) => (p, e),
            _ => return,
        };

        println!("\n[Ваш ход]");
        println!("1. Атаковать врага");
        println!("2. Лечить себя на 20 HP)");
        print!("Выберите действие: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                let cmd = AttackCommand::new(player, enemy);
                self.action_queue.push_back(Box::new(cmd));
            }
            "2" => {
                let cmd = HealCommand::new(player.clone(), player, 20);
                self.action_queue.push_back(Box::new(cmd));
            }
            _ => {
                println!("Неверный выбор, пропуск хода.");
            }
        }
    }

    pub fn execute_commands(&mut self) {
        while let Some(cmd) = self.action_queue.pop_front() {
            cmd.execute(&mut self.stats);
        }
    }

    pub fn post_turn(&mut self) -> bool {
        self.entities.retain(|e| e.borrow().is_alive());

        let mut player_alive = false;
        let mut enemy_alive = false;
        for e in &self.entities {
            match e.borrow().team {
                Team::Player => player_alive = true,
                Team::Enemy => enemy_alive = true,
            }
        }
        !(player_alive && enemy_alive)
    }

    pub fn run(&mut self) {
        println!("=== Бой начинается! ===");
        while self.round < self.max_rounds {
            self.round += 1;
            self.stats.increment_rounds();
            println!("\n--- Раунд {} ---", self.round);

            self.pre_turn();
            self.gather_commands();
            self.execute_commands();

            let finished = self.post_turn();
            if finished {
                println!("Бой окончен после раунда {}!", self.round);
                break;
            }
        }
        if self.round >= self.max_rounds {
            println!("Бой завершён по достижении максимального числа раундов.");
        }

        self.determine_winner();
        self.print_result();
        if let Err(e) = self.stats.save_to_file() {
            eprintln!("Не удалось сохранить статистику: {}", e);
        }
    }

    fn determine_winner(&mut self) {
        let player = self.entities.iter().find(|e| e.borrow().team == Team::Player);
        let enemy = self.entities.iter().find(|e| e.borrow().team == Team::Enemy);
        match (player, enemy) {
            (Some(_), None) => self.stats.set_winner("Игрок"),
            (None, Some(_)) => self.stats.set_winner("Враг"),
            (Some(_), Some(_)) => self.stats.set_winner("Ничья"),
            (None, None) => self.stats.set_winner("Все мертвы"),
        }
    }

    pub fn print_result(&self) {
        println!("\n=== Итоги ===");
        if self.entities.is_empty() {
            println!("Все погибли. Ничья.");
            return;
        }
        for e in &self.entities {
            let entity = e.borrow();
            println!("{}: HP = {}/{}", entity.name, entity.hp, entity.max_hp);
        }
        let player = self.entities.iter().find(|e| e.borrow().team == Team::Player);
        let enemy = self.entities.iter().find(|e| e.borrow().team == Team::Enemy);
        match (player, enemy) {
            (Some(_), None) => println!("Победил игрок!"),
            (None, Some(_)) => println!("Победил враг!"),
            (Some(_), Some(_)) => println!("Оба живы, но время вышло."),
            (None, None) => println!("Все мертвы."),
        }
    }
}