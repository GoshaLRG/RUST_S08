use crate::effect::Effect;

#[derive(Debug)] 
pub struct Entity {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub effects: Vec<Box<dyn Effect>>,
    pub team: Team,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Team {
    Player,
    Enemy,
}

impl Entity {
    pub fn new(name: &str, hp: i32, attack: i32, team: Team) -> Self {
        Self {
            name: name.to_string(),
            hp,
            max_hp: hp,
            attack,
            effects: Vec::new(),
            team,
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
        if self.hp < 0 {
            self.hp = 0;
        }
        println!("{} получает {} урона. Осталось HP: {}", self.name, damage, self.hp);
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp += amount;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
        }
        println!("{} исцелён на {}. HP: {}", self.name, amount, self.hp);
    }

    pub fn add_effect(&mut self, effect: Box<dyn Effect>) {
        println!("{} получил эффект: {}", self.name, effect.name());
        self.effects.push(effect);
    }

    
    pub fn apply_effects(&mut self) {
        let mut effects = std::mem::take(&mut self.effects);
        for mut effect in effects {
            effect.apply(self);
            self.effects.push(effect);
        }
    }

    pub fn tick_effects(&mut self) {
        for effect in &mut self.effects {
            effect.tick();
        }
    }

    pub fn remove_expired_effects(&mut self) {
        self.effects.retain(|e| !e.is_expired());
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}