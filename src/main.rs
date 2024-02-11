use crate::setup::get_user_input;
use moves::{Move, SpecialMoves};
use rand::distributions::{Distribution, Uniform};
use setup::Player; 

pub mod setup;
pub mod moves;

fn attack(
    move_: &moves::Move, 
    teams: &mut Vec<Vec<setup::Player>>, 
    team: usize, 
    player: usize, 
    tt: usize, 
    tp: usize
) {
    if move_.dispel > 0 {
        teams[tt][tp].regen -= move_.dispel;
        teams[tt][tp].strength -= move_.dispel;
        teams[tt][tp].guard -= move_.dispel;
        teams[tt][tp].dodge -= move_.dispel;
        if teams[tt][tp].shield > 0 {
            if teams[tt][tp].shield <= move_.dispel * 2 {
                teams[tt][tp].shield = 0;
                println!("T{}P{}'s shield broke", tt + 1, tp + 1);
            } else {
                teams[tt][tp].shield -= move_.dispel * 2;
                println!(
                    "T{}P{}'s shield took {} damage",
                    tt + 1,
                    tp + 1,
                    move_.dispel * 2
                );
            }
        }
        
    }

    for _ in 0..move_.repeat {
        let mut remaining_dmg = move_.damage;
        //check strength 
        if teams[team][player].strength > 0 {
            teams[team][player].strength -= 1;
            remaining_dmg *= 2;
            println!("The attack is strengthened")
        }
        //check weaken
        if teams[team][player].weaken > 0 {
            teams[team][player].weaken -= 1;
            remaining_dmg /= 2;
            if remaining_dmg < 1 {
                println!("Too weak to attack")
            } else {
                println!("The attack is weakened");
            }
        }
        //check dodge
        if teams[tt][tp].dodge > 0 && remaining_dmg > 3 {
            teams[tt][tp].dodge -= 1;
            println!("T{}P{} dodged", tt + 1, tp + 1);
            continue;
        }
        //check guard
        if teams[tt][tp].guard > 0 && remaining_dmg < 3 {
            teams[tt][tp].guard -= 1;
            println!("T{}P{} guarded", tt + 1, tp + 1);
            break;
        }
        //shield fully blocks
        if teams[tt][tp].shield > 0 {
        //shield
            if teams[tt][tp].shield > remaining_dmg {
                teams[tt][tp].shield -= remaining_dmg;
                println!("T{}P{}'s shield took {} damage", tt + 1, tp + 1, remaining_dmg);
            //shield break
            } else if teams[tt][tp].shield <= move_.damage {
                remaining_dmg -= teams[tt][tp].shield;
                teams[tt][tp].shield = 0;
                println!("T{}P{}'s shield broke", tt + 1, tp + 1);
                if remaining_dmg == 0 {
                    continue;
                }
            }
        }
        //actual damage
        teams[tt][tp].hp -= remaining_dmg as i32;
        println!("T{}P{} dealt {} damage to T{}P{}", team + 1, player + 1, remaining_dmg, tt + 1, tp + 1);
        if teams[tt][tp].hp <= 0 {
            println!("T{}P{} was felled!", tt + 1, tp + 1);
        }
    }
    //debuffs
    //poison
    if move_.poison > 0 {
        teams[tt][tp].poison = if teams[tt][tp].poison > move_.poison {
            teams[tt][tp].poison
        } else {
            move_.poison
        };
        println!("T{}P{} was poisoned for {} turn(s)", tt + 1, tp + 1, move_.poison);
    }
    //burn
    if move_.burn > 0 {
        teams[tt][tp].burn = if teams[tt][tp].burn> move_.burn {
            teams[tt][tp].burn
        } else {
            move_.burn
        };
        println!("T{}P{} was burned for {} turn(s)", tt + 1, tp + 1, move_.burn);
    }
    //stun
    if move_.stun > 0 {
        teams[tt][tp].stun = if teams[tt][tp].burn > move_.stun {
            teams[tt][tp].stun
        } else {
            move_.stun
        };
        println!("T{}P{} was stunned for {} turn(s)", tt + 1, tp + 1, move_.stun);
    }
    //weaken
    if move_.weaken > 0 {
        teams[tt][tp].weaken = if teams[tt][tp].weaken > move_.weaken {
            teams[tt][tp].weaken
        } else {
            move_.weaken
        };
        println!("T{}P{} was weakened for {}", tt + 1, tp + 1, move_.weaken);
    }
}

fn get_target(teams: &Vec<Vec<Player>>, team: usize) -> (usize, usize) {
    //infer target when possible
    //TODO: do something about targeting players with < 1 hp
    let mut tt = 0;
    let mut tp = 0;
    let mut invalid = true;
    while invalid {
        if teams.len() == 2 {
            tt = 1 - team;
        } else {
            println!("Select team to target:");
            tt = get_usize_between(1, teams.len()) - 1;
            if tt == team || teams[tt].iter().all(|p| p.hp <= 0) {
                println!("Invalid Team");
                continue;
            }
        }
        if teams[tt].len() == 1 {
            tp = 0;
        } else {
            println!("Select player to target:");
            tp = get_usize_between(1, teams[tt].len()) - 1;
            if teams[tt][tp].hp <= 0 {
                println!("Invalid Player");
            } else {
                invalid = false;
            }
        }
    }
    (tt, tp)
}

fn execute(teams: &mut Vec<Vec<setup::Player>>, move_type: &moves::Move, team: usize, player: usize) {
    let move_: &moves::Move;
    let gen_move: moves::Move;
    let mut tt;
    let mut tp;
    let effect: Option::<fn(move_: &Move, teams: &mut Vec<Vec<Player>>, team: usize, player: usize, tt: usize, tp: usize)>;
    let requires_target = move_type.special.requires_effect_target();
    if move_type.special == SpecialMoves::Default { 
        move_ = move_type;
        effect = None;
    } else if requires_target {
        (tt, tp) = get_target(teams, team);
        (gen_move, effect) = move_type.special.get(&teams[team][player], &teams[tt][tp]);
        move_ = &gen_move;
    } else {
        (gen_move, effect) = move_type.special.get(&teams[team][player], &teams[0][0]);
        move_ = &gen_move;
    }

    if !requires_target && (move_.damage > 0 || move_.poison > 0 || move_.burn > 0 || move_.stun > 0 || move_.weaken > 0 || move_.dispel > 0) {
        (tt, tp) = get_target(teams, team);
    } else {
        tt = 0;
        tp = 0;
    }
    
    //execution order: tankify, heal, cleanse, user str/weaken, dispel, target shield/guard/dodge, damage, debuff, buff
    //tankify
    if move_.tankify > 0 {
        teams[team][player].max_hp += move_.tankify;
        println!("T{}P{} tankified for {}", team + 1, player + 1, move_.tankify);
    }
    //heal
    if move_.heal > 0 {
        teams[team][player].hp += move_.heal as i32;
        if teams[team][player].hp > teams[team][player].max_hp as i32 {
            teams[team][player].hp = teams[team][player].max_hp as i32;
        }
        println!("T{}P{} healed for {}", team + 1, player + 1, move_.heal);
    }
    //cleanse
    if move_.cleanse > 0 {
        if move_.cleanse > teams[team][player].poison {
            teams[team][player].poison = 0;
        } else {
            teams[team][player].poison -= move_.cleanse;
        }
        if move_.cleanse > teams[team][player].burn {
            teams[team][player].burn = 0;
        } else {
            teams[team][player].burn -= move_.cleanse;
        }
        if move_.cleanse > teams[team][player].weaken {
            teams[team][player].weaken = 0;
        } else {
            teams[team][player].weaken -= move_.cleanse;
        }
        println!("T{}P{} cleansed for {}", team + 1, player + 1, move_.cleanse);
    }
    //attack sequence
    attack(&move_, teams, team, player, tt, tp);
    //buffs
    let user = &mut teams[team][player];
    //regen
    if move_.regen > 0 {
        user.regen = if user.regen > move_.regen {
            user.regen
        } else {
            move_.regen
        };
        println!("T{}P{} begun regenerating for {}", team + 1, player + 1, move_.regen);
    }
    //strength
    if move_.strength > 0 {
        user.strength += move_.strength;
        println!("T{}P{} gained {} strength", team + 1, player + 1, move_.strength);
    }
    //shield
    if move_.shield > 0 {
        user.shield = if user.shield > move_.shield {
            user.shield
        } else {
            move_.shield
        };
        println!("T{}P{} acquired a shield of {}", team + 1, player + 1, move_.shield);
    }
    //guard
    if move_.guard > 0 {
        user.guard += move_.guard;
        println!("T{}P{} gained {} guard", team + 1, player + 1, move_.guard);
    }
    //dodge
    if move_.dodge > 0 {
        user.dodge += move_.dodge;
        println!("T{}P{} gained {} dodge", team + 1, player + 1, move_.dodge);
    }
    if effect.is_some() {
        effect.unwrap()(move_, teams, team, player, tt, tp);
    }
}

fn take_turn(game: &mut setup::Game, team: usize, player: usize, dist: &Uniform<u32>) -> (bool, usize) {
    //print game state
    println!("{}", game.teams
        .iter()
        .map(|t| t
            .iter()
            .map(|p| if p.hp > 0 { p.hp.to_string() } else { "X".to_string() })
            .collect::<Vec<String>>()
        .join(", "))
        .collect::<Vec<String>>()
        .join(" - ")
    );
    println!("Player {} of Team {}'s Turn!", player + 1, team + 1);
    //regen
    if game.teams[team][player].regen > 0 {
        println!("T{}P{} regenerated 1 hp", team + 1, player + 1);
        game.teams[team][player].regen -= 1;
    }
    //stun
    if game.teams[team][player].stun > 0 {
        println!("T{}P{} is stunned!", team + 1, player + 1);
    } else {
        let mut turn = true;
        let mut streak = 0;
        let mut rng = rand::thread_rng();
        while turn {
            let input = get_user_input::<String>().to_ascii_lowercase();
            if input == "s" {
                let random = dist.sample(&mut rng);
                if random < 50 + game.luck as u32 {
                    streak += 1;
                    println!("Streak Successful");
                } else {
                    turn = false;
                    println!("Streak Failed");
                }
            } else if let Some(move_) = moves::MOVES.get(&input.as_str()) {
                if streak >= move_.cost {
                    execute(&mut game.teams, &move_, team, player);
                    turn = false;
                } else {
                    println!(
                        "You need {} more streak points to use {}!",
                        move_.cost - streak,
                        input
                    )
                }
            } else {
                println!("Invalid Input");
            }
        }
    }
    //poison
    if game.teams[team][player].poison > 0 {
        game.teams[team][player].poison -= 1;
        game.teams[team][player].hp -= 1;
        println!("T{}P{} took 1 damage from poison ({} remaining)", team + 1, player + 1, game.teams[team][player].poison);
    }
    //burn
    if game.teams[team][player].burn > 0 {
        game.teams[team][player].burn -= 1;
        game.teams[team][player].hp -= 2;
        println!("T{}P{} took 2 damage from burning ({} remaining)", team + 1, player + 1, game.teams[team][player].burn);
    }
    //check for status effect defeat
    if game.teams[team][player].hp <= 0 {
        println!("T{}P{} was felled!", team + 1, player + 1);
        game.teams[team][player].poison = 0;
        game.teams[team][player].burn = 0;
        game.teams[team][player].stun = 0;
        game.teams[team][player].weaken = 0;
    }
    game_over(&game.teams)
}

fn game_over(teams: &Vec<Vec<setup::Player>>) -> (bool, usize) {
    (
        teams
            .iter()
            .filter(|team| team
                .iter()
                .any(|player| player.hp > 0)
            )
            .count() == 1,
        teams
            .iter()
            .position(|team| team
                .iter()
                .any(|player| player.hp > 0)
            )
            .unwrap(),
    )
}

pub fn get_usize_between(min: usize, max: usize) -> usize {
    let mut input: usize = get_user_input();
    while input < min || input > max {
        println!("Invalid Input");
        input = get_user_input();
    }
    input
}

fn main() {
    println!("Welcome to the game!");
    let mut game = setup::setup_game();

    let distribution = Uniform::new(1, 100);

    let mut game_over = false;
    let mut winner = 0;
    while !game_over {
        for team in 0..game.teams.len() {
            for player in 0..game.teams[team].len() {
                if game.teams[team][player].hp > 0 {
                    (game_over, winner) = take_turn(&mut game, team, player, &distribution);
                }
                if game_over {
                    break;
                }
            }
            if game_over {
                break;
            }
        }
        if game_over {
            break;
        }
    }
    println!("Team {} won the game!", winner + 1);
}
