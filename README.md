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
