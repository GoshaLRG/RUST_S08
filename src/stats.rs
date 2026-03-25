use std::fs::OpenOptions;
use std::io::Write;
use chrono::Local;

pub struct BattleStats {
    pub enemy_name: String,
    pub rounds: u32,
    pub total_damage_dealt: i32,
    pub total_healing_done: i32,
    pub total_poison_damage: i32,
    pub winner: String,
}

impl BattleStats {
    pub fn new(enemy_name: &str) -> Self {
        Self {
            enemy_name: enemy_name.to_string(),
            rounds: 0,
            total_damage_dealt: 0,
            total_healing_done: 0,
            total_poison_damage: 0,
            winner: String::new(),
        }
    }

    pub fn add_damage(&mut self, amount: i32) {
        self.total_damage_dealt += amount;
    }

    pub fn add_healing(&mut self, amount: i32) {
        self.total_healing_done += amount;
    }

    pub fn add_poison_damage(&mut self, amount: i32) {
        self.total_poison_damage += amount;
    }

    pub fn increment_rounds(&mut self) {
        self.rounds += 1;
    }

    pub fn set_winner(&mut self, winner: &str) {
        self.winner = winner.to_string();
    }

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S");

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("battle_stats.txt")?;

        writeln!(file, "Бой {}", timestamp)?;
        writeln!(file, "Противник: {}", self.enemy_name)?;
        writeln!(file, "Раундов: {}", self.rounds)?;
        writeln!(file, "Нанесено урона (атаки): {}", self.total_damage_dealt)?;
        writeln!(file, "Нанесено урона (ядом): {}", self.total_poison_damage)?;
        writeln!(file, "Получено лечения: {}", self.total_healing_done)?;
        writeln!(file, "Победитель: {}", self.winner)?;

        Ok(())
    }
}