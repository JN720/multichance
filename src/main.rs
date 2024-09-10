use crate::setup::get_user_input;
use moves::{Move, SpecialMoves, MOVES};
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
        //dispel target's buffs
        if teams[tt][tp].regen > move_.dispel {
            teams[tt][tp].regen -= move_.dispel;
        } else {
            teams[tt][tp].regen = 0;
        }
        if teams[tt][tp].strength > move_.dispel {
            teams[tt][tp].strength -= move_.dispel;
        } else {
            teams[tt][tp].strength = 0;
        }
        if teams[tt][tp].guard > move_.dispel {
            teams[tt][tp].guard -= move_.dispel;
        } else {
            teams[tt][tp].guard = 0;
        }
        if teams[tt][tp].dodge > 0 {
            teams[tt][tp].dodge -= move_.dispel;
        } else {
            teams[tt][tp].dodge = 0;
        }
        //shield takes twice the dispel amount as damage
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

fn get_target(teams: &Vec<Vec<Player>>, team: usize, is_cpu: bool) -> (usize, usize) {
    //figure out target for cpu
    //TODO: make better
    if is_cpu {
        let valid_teams = (0..teams.len())
            .filter(|t| 
                *t != team && teams[*t]
                    .iter()
                    .any(|p| p.hp > 0))
            .collect::<Vec<usize>>();
        let tt = valid_teams[rand::random::<usize>() % valid_teams.len()];
        let valid_players = (0..teams[tt].len())
            .filter(|p| teams[tt][*p].hp > 0)
            .collect::<Vec<usize>>();
        let tp = valid_players[rand::random::<usize>() % valid_players.len()];
        return (tt, tp);
    }
    let mut tt = 0;
    let mut tp = 0;
    let mut invalid = true;
    while invalid {
        //infer target when possible
        if teams.len() == 2 {
            tt = 1 - team;
        } else {
            println!("Select team to target:");
            tt = get_num_between(1, teams.len()) - 1;
            if tt == team || teams[tt].iter().all(|p| p.hp <= 0) {
                println!("Invalid Team");
                continue;
            }
        }
        if teams[tt].len() == 1 {
            tp = 0;
        } else {
            println!("Select player to target:");
            tp = get_num_between(1, teams[tt].len()) - 1;
        }
        if teams[tt][tp].hp <= 0 {
            println!("Invalid Player");
        } else {
            invalid = false;
        }
    }
    (tt, tp)
}

fn execute(teams: &mut Vec<Vec<setup::Player>>, move_type: &moves::Move, team: usize, player: usize) {
    let move_: &moves::Move;
    let is_cpu = teams[team][player].is_cpu;
    //generated special move
    let gen_move: moves::Move;
    //target team and player
    let mut tt;
    let mut tp;
    //side effect of special moves
    let effect: Option::<fn(move_: &Move, teams: &mut Vec<Vec<Player>>, team: usize, player: usize, tt: usize, tp: usize)>;
    //since special moves are created on the spot and may depend on the target,
    //we may need to figure out the target before executing the move
    let requires_target = move_type.special.requires_effect_target();
    if move_type.special == SpecialMoves::Default { 
        move_ = move_type;
        effect = None;
    } else if requires_target {
        (tt, tp) = get_target(teams, team, is_cpu);
        (gen_move, effect) = move_type.special.get(&teams[team][player], &teams[tt][tp]);
        move_ = &gen_move;
    } else {
        (gen_move, effect) = move_type.special.get(&teams[team][player], &teams[0][0]);
        move_ = &gen_move;
    }
    //regular targetting for non-special moves
    if !requires_target && (move_.repeat > 0 || move_.poison > 0 || move_.burn > 0 || move_.stun > 0 || move_.weaken > 0 || move_.dispel > 0) {
        (tt, tp) = get_target(teams, team, is_cpu);
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
    //execute special move side effect if applicable
    if effect.is_some() {
        effect.unwrap()(move_, teams, team, player, tt, tp);
    }
}

fn get_cpu_input(streak: u32, random: u32) -> String {
    if streak == 0 {
        return "s".to_string()
    }
    if random > 33 && streak != 5 {
        "s".to_string()
    } else {
        let valid_moves = MOVES
            .entries()
            .filter(|e| e.1.cost == streak)
            .map(|e| *e.0)
            .collect::<Vec<&str>>();
        let chosen = valid_moves[rand::random::<usize>() % valid_moves.len()].to_string();
        println!("{}", chosen);
        chosen
    }
}

fn take_turn(game: &mut setup::Game, team: usize, player: usize, dist: &Uniform<u32>) -> (bool, usize) {
    let is_cpu = game.teams[team][player].is_cpu;
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
            let input = if is_cpu { get_cpu_input(streak, dist.sample(&mut rng)) } else { get_user_input::<String>().to_ascii_lowercase() };
            //streak
            if input == "s" {
                let random = dist.sample(&mut rng);
                if random < 50 + game.luck as u32 {
                    streak += 1;
                    println!("Streak Successful");
                } else {
                    turn = false;
                    println!("Streak Failed");
                }
            //move
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
            //TODO: help and move list
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
    //check for defeat by status effect
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

pub fn get_num_between<T>(min: T, max: T) -> T 
where
    T: std::str::FromStr,
    T: std::fmt::Display,
    T::Err: std::fmt::Debug,
    T: PartialOrd
{
    let mut input: T = get_user_input();
    while input < min || input > max {
        println!("Invalid Input");
        println!(
            "The number must be between {} and {}",
            min, max
        );
        input = get_user_input();
    }
    input
}

fn main() {
    println!("Welcome to MultiChance!");
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
                //TODO: is there really no better way to do this?
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
