use std::io;

#[derive(Debug)]
struct Player {
    is_cpu: bool,
}

#[derive(Debug)]
struct Game {
    starting_hp: u32,
    luck: u32,
    game_mode: String,
    num_teams: u32,
    team_player_count: Vec<u32>,
    players: Vec<Player>,
}

impl Game {
    fn new(starting_hp: u32, luck: u32, game_mode: String, num_teams: u32, team_player_count: Vec<u32>, players: Vec<Player>) -> Self {
        Game { starting_hp, luck, game_mode, num_teams, team_player_count, players }
    }

    fn display(&self) {
        println!("Game Settings:");
        println!("Starting HP: {}", self.starting_hp);
        println!("Luck: {}", self.luck);
        println!("Game Mode: {}", self.game_mode);
        println!("Number of Teams: {}", self.num_teams);
        println!("Team Player Count: {:?}", self.team_player_count);
        println!("Players: {:?}", self.players);
    }
}

fn main() {
    // Get user input
    println!("Enter starting HP:");
    let starting_hp: u32 = get_user_input();

    println!("Enter luck:");
    let luck: u32 = get_user_input();

    println!("Enter game mode:");
    let game_mode: String = get_user_input();

    println!("Enter number of teams:");
    let num_teams: u32 = get_user_input();

    let mut team_player_count = Vec::new();
    for team in 1..=num_teams {
        println!("Enter player count for Team {}:", team);
        team_player_count.push(get_user_input());
    }

    let mut players = Vec::new();
    for team in 1..=num_teams {
        for player in 1..=team_player_count[(team - 1) as usize] {
            println!("Is Player {} in Team {} a CPU? (true/false):", player, team);
            let is_cpu: bool = get_user_input();
            players.push(Player { is_cpu });
        }
    }

    // Create game instance
    let game = Game::new(starting_hp, luck, game_mode, num_teams, team_player_count, players);

    // Display game settings
    game.display();
}

fn get_user_input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}