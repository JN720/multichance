use phf::phf_map;

#[derive(Clone)]
pub enum SpecialMoves {
    Default,
    Arrow,
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

pub const fn hybrid(
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
    "superstrike" => dmg(4, 8, 1),
    "parry this" => dmg(5, 15, 1),
    //off: (cost, damage, repeat, poison, burn, stun, dispel, weaken)
    "intoxicate" => off(2, 1, 1, 3, 0, 0, 0, 0),
    "fireball" => off(3, 1, 1, 0, 2, 0, 0, 0),
    "stun" => off(3, 0, 0, 0, 0, 3, 0, 0),
    //def: (cost, heal, shield, tankify, regen, strength, guard, dodge, cleanse)
    "shield" => def(2, 0, 4, 0, 0, 0, 0, 0, 0),
    
    //hybrid: (cost, damage, repeat, heal, tankify, poison, burn, stun, weaken, regen, strength, guard, dodge, cleanse)

    //special: (cost, special)
    "arrow" => special(2, SpecialMoves::Arrow),
};
