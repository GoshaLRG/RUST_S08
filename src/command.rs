use std::rc::Rc;
use std::cell::RefCell;
use crate::entity::Entity;

pub trait Command {
    fn execute(&self);
}

pub struct AttackCommand {
    pub attacker: Rc<RefCell<Entity>>,
    pub target: Rc<RefCell<Entity>>,
}

impl AttackCommand {
    pub fn new(attacker: Rc<RefCell<Entity>>, target: Rc<RefCell<Entity>>) -> Self {
        Self { attacker, target }
    }
}

impl Command for AttackCommand {
    fn execute(&self) {
        let mut attacker = self.attacker.borrow_mut();
        let mut target = self.target.borrow_mut();
        let damage = attacker.attack;
        println!("{} атакует {} и наносит {} урона!", attacker.name, target.name, damage);
        target.take_damage(damage);
    }
}

pub struct HealCommand {
    pub healer: Rc<RefCell<Entity>>,
    pub target: Rc<RefCell<Entity>>,
    pub amount: i32,
}

impl HealCommand {
    pub fn new(healer: Rc<RefCell<Entity>>, target: Rc<RefCell<Entity>>, amount: i32) -> Self {
        Self { healer, target, amount }
    }
}

impl Command for HealCommand {
    fn execute(&self) {
        let healer_name = self.healer.borrow().name.clone();
        let target_name = self.target.borrow().name.clone();
        println!("{} применяет лечение на {}", healer_name, target_name);

        let mut target = self.target.borrow_mut();
        target.heal(self.amount);
    }
}