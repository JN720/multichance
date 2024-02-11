use crate::setup::get_user_input;
use rand::distributions::{Distribution, Uniform}; 

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
            teams[tt][tp].shield -= move_.dispel * 2;
            if teams[tt][tp].shield <= 0 {
                teams[tt][tp].shield = 0;
                println!("T{}P{}'s shield broke", tt, tp)
            }
        }
        
    }

    if move_.damage > 0 {
        for _ in 0..move_.repeat {
            let mut remaining_dmg = move_.damage;
            //check weaken
            if teams[team][player].weaken > 0 {
                teams[team][player].weaken -= 1;
                remaining_dmg /= 2;
                if remaining_dmg < 1 {
                    println!("Too weak to attack")
                } else {
                    println!("The attack was weakened");
                }
            }
            //check dodge
            if teams[tt][tp].dodge > 0 && remaining_dmg > 3 {
                teams[tt][tp].dodge -= 1;
                println!("T{}P{} dodged", tt, tp);
                continue;
            }
            //check guard
            if teams[tt][tp].guard > 0 && remaining_dmg < 3 {
                teams[tt][tp].guard -= 1;
                println!("T{}P{} guarded", tt, tp);
                break;
            }
            //shield fully blocks
            if teams[tt][tp].shield > remaining_dmg {
                teams[tt][tp].shield -= remaining_dmg;
                println!("T{}P{}'s shield took {} damage", tt, tp, remaining_dmg);
            //shield break
            } else if teams[tt][tp].shield <= move_.damage {
                remaining_dmg -= teams[tt][tp].shield;
                teams[tt][tp].shield = 0;
                println!("T{}P{}'s shield broke", tt, tp);
                if remaining_dmg == 0 {
                    continue;
                }
            }
            //actual damage
            teams[tt][tp].hp -= remaining_dmg;
            println!("T{}P{} dealt {} damage to T{}P{}", team, player, remaining_dmg, tt, tp);
            if teams[tt][tp].hp <= 0 {
                println!("T{}P{} was felled!", tt, tp);
            }
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
        println!("T{}P{} was poisoned for {}", tt, tp, move_.poison);
    }
    //burn
    if move_.burn > 0 {
        teams[tt][tp].burn = if teams[tt][tp].burn> move_.burn {
            teams[tt][tp].burn
        } else {
            move_.burn
        };
        println!("T{}P{} was burned for {}", tt, tp, move_.burn);
    }
    //stun
    if move_.stun > 0 {
        teams[tt][tp].stun = if teams[tt][tp].burn > move_.stun {
            teams[tt][tp].stun
        } else {
            move_.stun
        };
        println!("T{}P{} was stunned for {}", tt, tp, move_.stun);
    }
    //weaken
    if move_.weaken > 0 {
        teams[tt][tp].weaken = if teams[tt][tp].weaken > move_.weaken {
            teams[tt][tp].weaken
        } else {
            move_.weaken
        };
        println!("T{}P{} was weakened for {}", tt, tp, move_.weaken);
    }
}

fn execute(teams: &mut Vec<Vec<setup::Player>>, move_: &moves::Move, team: usize, player: usize) {
    //execution order: tankify, heal, cleanse, user str/weaken, dispel, target shield/guard/dodge, damage, debuff, buff
    //tankify
    if move_.tankify > 0 {
        teams[team][player].max_hp += move_.tankify;
        println!("T{}P{} tankified for {}", team, player, move_.tankify);
    }
    //heal
    if move_.heal > 0 {
        teams[team][player].hp += move_.heal;
        if teams[team][player].hp > teams[team][player].max_hp {
            teams[team][player].hp = teams[team][player].max_hp;
        }
        println!("T{}P{} healed for {}", team, player, move_.heal);
    }
    //cleanse
    if move_.cleanse > 0 {
        teams[team][player].poison -= 1;
        teams[team][player].burn -= 1;
        teams[team][player].stun -= 1;
        teams[team][player].weaken -= 1;
        println!("T{}P{} cleansed for {}", team, player, move_.cleanse);
    }

    //attack sequence
    if move_.damage > 0 || move_.poison > 0 || move_.burn > 0 || move_.stun > 0 || move_.weaken > 0 || move_.dispel > 0 {
        //infer target when possible
        let tt: usize;
        if teams.len() == 2 {
            tt = 1 - team;
        } else {
            tt = get_usize_between(1, teams.len()) - 1;
        }
        let tp: usize;
        if teams[tt].len() == 1 {
            tp = 0;
        } else {
            tp = get_usize_between(1, teams[tt].len()) - 1;
        }
        attack(move_, teams, team, player, tt, tp)
    }
    //buffs
    let user = &mut teams[team][player];
    //regen
    if move_.regen > 0 {
        user.regen = if user.regen > move_.regen {
            user.regen
        } else {
            move_.regen
        };
        println!("T{}P{} begun regenerating for {}", team, player, move_.regen);
    }
    //strength
    if move_.strength > 0 {
        user.strength += move_.strength;
        println!("T{}P{} gained {} strength", team, player, move_.strength);
    }
    //shield
    if move_.shield > 0 {
        user.shield = if user.shield > move_.shield {
            user.shield
        } else {
            move_.shield
        };
        println!("T{}P{} acquired a shield of {}", team, player, move_.shield);
    }
    //guard
    if move_.guard > 0 {
        user.guard += move_.guard;
        println!("T{}P{} gained {} guard", team, player, move_.guard);
    }
    //dodge
    if move_.dodge > 0 {
        user.dodge += move_.dodge;
        println!("T{}P{} gained {} dodge", team, player, move_.dodge);
    }
}

fn take_turn(game: &mut setup::Game, team: usize, player: usize, dist: &Uniform<u32>) -> (bool, usize) {
    let mut turn = true;
    let mut streak = 0;

    println!("Player {} of Team {}'s Turn!", team + 1, player + 1);
    let mut rng = rand::thread_rng();
    while turn {
        let input = get_user_input::<String>().to_ascii_lowercase();
        if input == "s" {
            let random = dist.sample(&mut rng);
            if random > 50 + game.luck {
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
            println!("Invalid input!");
        }
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

fn get_usize_between(min: usize, max: usize) -> usize {
    let mut input: usize = get_user_input();
    while input < min || input > max {
        println!("Invalid input!");
        input = get_user_input();
    }
    input
}

fn main() {
    println!("Welcome to the game!");
    let mut game = setup::setup_game();

    let distribution = Uniform::new(1, 100);

    let mut game_active = true;
    let mut winner = 0;
    while game_active {
        for team in 0..game.teams.len() {
            for player in 0..game.teams[team].len() {
                (game_active, winner) = take_turn(&mut game, team, player, &distribution);
            }
        }
    }
    println!("{} won the game!", winner);

    
    
}
