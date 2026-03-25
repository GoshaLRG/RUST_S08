use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use crate::entity::Entity;
use crate::command::{Command, AttackCommand};

pub struct Engine {
    pub entities: Vec<Rc<RefCell<Entity>>>,
    pub action_queue: VecDeque<Box<dyn Command>>,
    round: u32,
    max_rounds: u32,
}

impl Engine {
    pub fn new(max_rounds: u32) -> Self {
        Self {
            entities: Vec::new(),
            action_queue: VecDeque::new(),
            round: 0,
            max_rounds,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(Rc::new(RefCell::new(entity)));
    }

    pub fn pre_turn(&mut self) {
        for entity_rc in &self.entities {
            let mut entity = entity_rc.borrow_mut();
            entity.apply_effects();
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
                crate::entity::Team::Player => player = Some(entity_rc.clone()),
                crate::entity::Team::Enemy => enemy = Some(entity_rc.clone()),
            }
        }

        if let (Some(player), Some(enemy)) = (player, enemy) {
            let cmd = AttackCommand::new(player, enemy);
            self.action_queue.push_back(Box::new(cmd));
        }
    }

    pub fn execute_commands(&mut self) {
        while let Some(cmd) = self.action_queue.pop_front() {
            cmd.execute();
        }
    }

    pub fn post_turn(&mut self) -> bool {
        self.entities.retain(|e| e.borrow().is_alive());

        let mut player_alive = false;
        let mut enemy_alive = false;

        for e in &self.entities {
            match e.borrow().team {
                crate::entity::Team::Player => player_alive = true,
                crate::entity::Team::Enemy => enemy_alive = true,
            }
        }

        !(player_alive && enemy_alive)
    }

    pub fn run(&mut self) {
        println!("=== Бой начинается! ===");

        while self.round < self.max_rounds {
            self.round += 1;
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

        self.print_result();
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

        let player = self.entities.iter().find(|e| e.borrow().team == crate::entity::Team::Player);
        let enemy = self.entities.iter().find(|e| e.borrow().team == crate::entity::Team::Enemy);

        match (player, enemy) {
            (Some(_), None) => println!("Победил игрок!"),
            (None, Some(_)) => println!("Победил враг!"),
            (Some(_), Some(_)) => println!("Оба живы, но время вышло."),
            (None, None) => println!("Все мертвы."),
        }
    }
}