# Sweet Turnips

Extracted from a game entry for Bitter Jam 2020.

An experiment in learning `ggez`, ECS (entity component systems), and event-driven architecture.

Created by JC Holder (jc@thirdtruck.org).

# Examples

## Bitter Turnips

### Gameplay

Villagers move around every turn (sometimes overlapping).

Villagers get hungry and harvest turnips every 4 turns.

Turnips grow into an adjacent square every 3 turns.

Villagers die if they go hungry for too long.

How many villagers can you keep alive?

### Controls

* `W` - move cursor up
* `A` - move cursor left
* `S` - move cursor down
* `D` - move cursor right
* `Space` - spawn a villager at where the cursor points
