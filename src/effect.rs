use crate::entity::Entity;

pub trait Effect: std::fmt::Debug {
    fn name(&self) -> &str;
    fn apply(&mut self, target: &mut Entity);
    fn tick(&mut self);
    fn is_expired(&self) -> bool;
}

#[derive(Debug)]
pub struct Poison {
    pub duration: u32,
    pub damage: u32,
}

impl Poison {
    pub fn new(duration: u32, damage: u32) -> Self {
        Self { duration, damage }
    }
}

impl Effect for Poison {
    fn name(&self) -> &str {
        "Яд"
    }

    fn apply(&mut self, target: &mut Entity) {
        println!("Яд наносит {} урона!", self.damage);
        target.take_damage(self.damage as i32);
    }

    fn tick(&mut self) {
        if self.duration > 0 {
            self.duration -= 1;
            println!("Яд: осталось {} ходов", self.duration);
        }
    }

    fn is_expired(&self) -> bool {
        self.duration == 0
    }
}

#[derive(Debug)]
pub struct Regeneration {
    pub duration: u32,
    pub heal_amount: u32,
}

impl Regeneration {
    pub fn new(duration: u32, heal_amount: u32) -> Self {
        Self { duration, heal_amount }
    }
}

impl Effect for Regeneration {
    fn name(&self) -> &str {
        "Регенерация"
    }

    fn apply(&mut self, target: &mut Entity) {
        println!("Регенерация восстанавливает {} HP!", self.heal_amount);
        target.heal(self.heal_amount as i32);
    }

    fn tick(&mut self) {
        if self.duration > 0 {
            self.duration -= 1;
            println!("Регенерация: осталось {} ходов", self.duration);
        }
    }

    fn is_expired(&self) -> bool {
        self.duration == 0
    }
}