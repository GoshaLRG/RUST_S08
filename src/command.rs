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