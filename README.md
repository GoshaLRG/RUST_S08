**RPG Engine – a little Rust battle game**

This is a simple turn‑based RPG engine I built as a learning project. It’s not huge, but it shows how you can put together a battle system with effects, commands, and a bit of player choice.

**What’s in it?**

**Entities** – every character has HP, attack power, a team (player/enemy), and a list of active effects.
**Effects** – things like poison (damage each turn) or regeneration (heal over time). Each effect has a duration, knows how to apply itself, how to tick down, and when it expires.
**Commands** – actions (attack, heal) are packed into objects that wait in a queue. The engine just runs them one by one. Makes it easy to add new actions later.
**Engine** – the core loop:  
  1. Apply effects (poison hurts you, regeneration heals you).  
  2. Let the player choose a move (attack or heal).  
  3. Execute the queued commands.  
  4. Clean up dead characters and check if the fight is over.
**Stats & logging** – the game tracks how many rounds passed, how much damage you dealt (both normal attacks and poison), how much you healed, and who won. After the battle, it appends all that to a file called `battle_stats.txt` with a timestamp.
**Interaction** – at the start you pick your enemy (normal goblin or a stronger one). Each round you choose between attacking or healing yourself. Everything prints to the console so you can follow the fight.

**How to run**

```bash
cargo run
```

========================================================================================================================================================================================================================================================

**RPG-движок – небольшая боевая игра на Rust**

Это простой пошаговый RPG-движок, который я создал в качестве учебного проекта. Он не очень большой, но показывает, как можно собрать боевую систему с эффектами, командами и небольшим выбором игрока.

**Что в нём?**

**Сущности** – у каждого персонажа есть HP, сила атаки, команда (игрок/враг) и список активных эффектов.

**Эффекты** – такие вещи, как яд (наносит урон каждый ход) или регенерация (лечит со временем). Каждый эффект имеет длительность, знает, как его применить, как отсчитывать время и когда он истекает.

**Команды** – действия (атака, лечение) упакованы в объекты, которые ожидают в очереди. Движок просто выполняет их по одному. Это упрощает добавление новых действий в дальнейшем.

**Движок** – основной цикл:

1. Применение эффектов (яд наносит урон, регенерация лечит).

2. Предоставьте игроку выбор хода (атака или лечение).

3. Выполните поставленные в очередь команды.

4. Уберите погибших персонажей и проверьте, закончился ли бой.

**Статистика и логирование** – игра отслеживает количество прошедших раундов, нанесенный урон (как обычными атаками, так и ядом), количество вылеченного урона и победителя. После боя все эти данные добавляются в файл `battle_stats.txt` с меткой времени.

**Взаимодействие** – в начале вы выбираете врага (обычного гоблина или более сильного). В каждом раунде вы выбираете между атакой и лечением. Все данные выводятся в консоль, чтобы вы могли следить за боем.

**Как запустить**

```bash
cargo run
```
