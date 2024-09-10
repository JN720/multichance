use phf::phf_map;

use crate::setup::Player;

#[derive(Clone, PartialEq)]
pub enum SpecialMoves {
    Default,
    Arrow,
    Accelerate,
    Backstab,
    Bank,
    Metallize,
}

impl Default for SpecialMoves {
    fn default() -> Self {
        Self::Default
    }
}

impl std::fmt::Debug for SpecialMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "Default"),
            Self::Arrow => write!(f, "Arrow"),
            Self::Accelerate => write!(f, "Accelerate"),
            Self::Backstab => write!(f, "Backstab"),
            Self::Bank => write!(f, "Bank"),
            Self::Metallize => write!(f, "Metallize"),
        }
    }
}

impl SpecialMoves {
    pub fn requires_effect_target(&self) -> bool {
        match self {
            Self::Metallize => true,
            _ => false,
        }
    }
    //generate special moves and their side effects
    pub fn get(&self, user: &Player, target: &Player) -> (
        Move, 
        Option<fn(move_: &Move, teams: &mut Vec<Vec<Player>>, team: usize, player: usize, tt: usize, tp: usize)>
    ) {
        match self {
            /*
                Arrow does 0, 1, 3, or 5 damage and 1 stun if the damage if 5
             */
            Self::Arrow => {
                fn effect (
                    move_: &Move,
                    _: &mut Vec<Vec<Player>>, 
                    _: usize, 
                    _: usize, 
                    _: usize,
                    _: usize
                ) {
                    println!("{}", match move_.damage {
                        5 => "Arrow Crit",
                        3 => "Arrow Hit",
                        1 => "Arrow Grazed",
                        _ => "Arrow Missed"
                    })
                }
                let mut power: u32 = rand::random::<u32>() % 4;
                if power == 2 {
                    power = 5
                }
                (off(2, power, 1, 0, 0, (power > 3) as u32, 0, 0), Some(effect))
            },
            /*
                Starts at 3 damage, increases by 1 each time it's used
            */
            Self::Accelerate => {
                fn effect (
                    move_: &Move,
                    teams: &mut Vec<Vec<Player>>, 
                    team: usize, 
                    player: usize, 
                    _: usize,
                    _: usize
                ) {
                    teams[team][player].special.accelerate += 1;
                    println!("T{}P{} accelerated to {}", team + 1, player + 1, move_.damage - 2);
                }
                (
                    dmg(3, 3 + user.special.accelerate, 1),
                    Some(effect)
                )
            },
            /* 
                Either does 3 damage and 1 stun or 1 damage
            */
            Self::Backstab => {
                fn effect (
                    _: &Move,
                    teams: &mut Vec<Vec<Player>>, 
                    _: usize, 
                    _: usize, 
                    tt: usize,
                    tp: usize
                ) {
                    teams[tt][tp].poison = 0;
                }
                let success: bool = rand::random();
                println!("{}", if success { "Backstab Successful" } else { "Backstab failed, resorting to standard attack" });
                (
                    if rand::random() {
                        dmg(2, 1, 1)
                    } else {
                        off(2, 3, 1, 0, 0, 1, 0, 0)
                    },
                    Some(effect) 
                )
            },
            /* 
                Does a single attack that does twice the amount of poison on the target
                Afterwards, all of the target's poison is removed
            */
            Self::Metallize => {
                println!("{} poison metallized", target.poison);
                (dmg(3, target.poison * 2, 1), None)
            },
            _ => (default(), None),
        }
    }    
}

#[derive(Debug, Clone)]
pub struct Move {
    pub cost: u32,
    pub damage: u32,
    pub repeat: u32,
    pub heal: u32,
    pub shield: u32,
    pub tankify: u32,
    pub poison: u32,
    pub burn: u32,
    pub stun: u32,
    pub weaken: u32,
    pub regen: u32,
    pub strength: u32,
    pub guard: u32,
    pub dodge: u32,
    pub cleanse: u32,
    pub dispel: u32,
    pub special: SpecialMoves
}

impl Move {
    pub fn desc(&self) {
        
    }
}

pub const fn default() -> Move {
    Move {
        cost: 0,
        damage: 0,
        repeat: 0,
        heal: 0,
        shield: 0,
        tankify: 0,
        poison: 0,
        burn: 0,
        stun: 0,
        weaken: 0,
        regen: 0,
        strength: 0,
        guard: 0,
        dodge: 0,
        cleanse: 0,
        dispel: 0,
        special: SpecialMoves::Default,
    }
}

pub const fn dmg(cost: u32, damage: u32, repeat: u32) -> Move {
    Move {
        cost,
        damage,
        repeat,
        ..default()
    }
}

pub const fn off(cost: u32, damage: u32, repeat: u32, poison: u32, burn: u32, stun: u32, weaken: u32, dispel: u32) -> Move {
    Move {
        cost,
        damage,
        repeat,
        poison,
        burn,
        stun,
        weaken,
        dispel,
        ..default()
    }

}

pub const fn def(cost: u32, heal: u32, shield: u32, tankify: u32, regen: u32, strength: u32, guard: u32, dodge: u32, cleanse: u32) -> Move {
    Move {
        cost, 
        heal,
        shield,
        tankify,
        regen,
        strength,
        guard,
        dodge,
        cleanse,
        ..default()
    }
}

pub const fn hyb (
    cost: u32,
    damage: u32,
    repeat: u32, 
    heal: u32, 
    shield: u32, 
    tankify: u32, 
    poison: u32, 
    burn: u32, 
    stun: u32, 
    weaken: u32, 
    regen: u32, 
    strength: u32, 
    guard: u32, 
    dodge: u32, 
    cleanse: u32,
    dispel: u32,
) -> Move {
    Move {
        cost,
        damage,
        repeat,
        heal,
        shield,
        tankify,
        poison,
        burn,
        stun,
        weaken,
        regen,
        strength,
        guard,
        dodge,
        cleanse,
        dispel,
        ..default()
    }
}

pub const fn special(cost: u32, special: SpecialMoves) -> Move {
    Move {
        cost,
        special,
        ..default()
    }
}

pub static MOVES: phf::Map<&'static str, Move> = phf_map! {
    //damage: (cost, damage, repeat)
    "sword" => dmg(1, 1, 1),
    "multisword" => dmg(2, 1, 3),
    "cannon" => dmg(3, 5, 1),
    "hax" => dmg(3, 3, 2),
    "superstrike" => dmg(4, 8, 1),
    "parry this" => dmg(5, 15, 1),
    //offensive: (cost, damage, repeat, poison, burn, stun, weaken, dispel)
    "zap" => off(2, 2, 1, 0, 0, 1, 0, 0),
    "intoxicate" => off(2, 1, 1, 3, 0, 0, 0, 0),
    "dispel" => off(2, 0, 0, 0, 0, 0, 0, 1),
    "fireball" => off(3, 3, 1, 0, 2, 0, 0, 0),
    "micronuke" => off(3, 3, 1, 2, 1, 0, 0, 0),
    "stun" => off(3, 0, 0, 0, 0, 3, 0, 0),
    "milinuke" => off(4, 5, 1, 3, 2, 0, 0, 0),
    "asphyxiate" => off(4, 1, 1, 15, 0, 0, 0, 0),
    "reaper" => off(5, 12, 1, 0, 0, 0, 0, 8),
    //defensive: (cost, heal, shield, tankify, regen, strength, guard, dodge, cleanse)
    "cleanse" => def(1, 0, 0, 0, 0, 0, 0, 0, 1),
    "heal" => def(2, 2, 0, 0, 0, 0, 0, 0, 1),
    "regenerate" => def(2, 1, 0, 0, 3, 0, 0, 0, 0),
    "shield" => def(2, 0, 4, 0, 0, 0, 0, 0, 0),
    "strength" => def(2, 0, 0, 0, 0, 1, 0, 0, 0),
    "guard" => def(2, 0, 0, 0, 0, 0, 1, 0, 0),
    "dodge" => def(3, 0, 0, 0, 0, 0, 0, 2, 0),
    //hybrid: (cost, damage, repeat, heal, shield, tankify, poison, burn, stun, weaken, regen, strength, guard, dodge, cleanse, dispel)
    "vampirize" => hyb(3, 3, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),

    //special: (cost, special)
    "arrow" => special(2, SpecialMoves::Arrow),
    "backstab" => special(2, SpecialMoves::Backstab),
    "accelerate" => special(3, SpecialMoves::Accelerate),
    "metallize" => special(3, SpecialMoves::Metallize),
    "bank" => special(4, SpecialMoves::Bank),
};
