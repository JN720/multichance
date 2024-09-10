# MultiChance

This is a local multiplayer recreation of the game Chance.
I built this project as a proxy for learning the basics of Rust.

# Gameplay

MultiChance is a luck-based CLI game where your goal is to defeat all opponents on all other teams.
An opponent is defeated by getting their HP down to 0.
When it is your turn, you can type 's' to attempt streak or utilize a move.
Every move requires some amount of streak, with higher-streak moves being more powerful.
When attempting streak, there is a random chance depending on the luck value to succeed.
Succeeding adds one to your current streak value, and you can attempt for more.
Failing streak however, causes you to lose all your current streak and your turn is over.
Utilizing a move also ends your turn.
While some moves have direct effects such as damage and healing, others can apply status effects.
Currently, the game has the following negative status effects:

- Poison: deals 1 damage each turn
- Burn: deals 2 damage each turn
- Stun: causes you to lose your turn
- Weaken: causes your next attack's damage to be halved (per individual instance of damage, moves with repeated damage exhaust more weaken)

Additionally, there are the following positive status effects:

- Shield: a shield that protects from direct damage until destroyed
- Regeneration: heals 1 HP each turn
- Strength: causes your next attack's damage to be doubled (applies like weaken)
- Guard: gives immunity to all attacks in a move that deal under 3 damage
- Dodge: gives immunity to an individual attack that deals over 3 damage

Some moves have a cleanse and dispel amount.
Cleanse removes 1 turn of each negative effect on yourself
Dispel removes 1 turn of a positive effect on a target.
Against a shield, dispel deals twice the dispel amount as damage to the shield.

Some moves are also special and have unique effects:

- Arrow: executes one of the 4 scenarios randomly: 0 damage, 1 damage, 3 damage, 5 damage & 1 stun
- Accelerate: starts by dealing 3 damage and does 1 more damage each time it is used
- Backstab: randomly either deals 1 damage or 3 damage & 1 stun
- Metallize: converts the number of turns the target has poison into a single attack dealing double the poison amount

# Game Settings

The game has the following settings.

-Starting HP: the amount of HP each player starts with
-Luck: this value is added to the default 50% likelihood of succeeding in streak (e.g. 10 luck means a 60% chance of succeeding)
-Game Mode: this setting specifies the game mode, though currently there is only one mode
-Number of Teams: this is the number of teams playing in the current game
-Player Counts: this is the number of players on a team, specified for each one
-CPU: if a particular player will be played by the CPU

# Running

This is a standard Rust project built with Cargo.
Ensure that Rust is installed.
To run it, navigate to the directory of the repository (with Cargo.toml) and execute

```
cargo build
```

followed by

```
cargo run
```

# Future

While I do not plan to return to this specific version, I may make a graphical version of the game
in a game engine sometime in the future, preferably with online multiplayer.
Some features I would like to add or that appeared in other versions of this game are:

-Draft Mode: each player selects a predesignated number of moves they can use

-Morphs: a type of move that gives passive abilities and access to a small repertoire of abilities
that are more powerful than non-morph abilities at the same streak level

-Summons: entities spawnable by certain moves with HP similar to players that have a
specific set of moves either controlled by the summoner or by AI
