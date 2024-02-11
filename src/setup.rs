use std::io;

#[derive(Debug, Default)]
pub struct Player {
    pub is_cpu: bool,
    pub max_hp: u32,
    pub hp: u32,
    pub regen: u32,
    pub poison: u32,
    pub burn: u32,
    pub stun: u32,
    pub strength: u32,
    pub shield: u32,
    pub dodge: u32,
    pub guard: u32,
    pub weaken: u32,
    pub morph: Morph,
    pub morph_lvl: u32,
    pub special: SpecialState
}

#[derive(Debug)]
pub enum Morph {
    Default,
    Archmage,
}

impl Default for Morph {
    fn default() -> Self {
        Morph::Default
    }
}


#[derive(Debug, Default)]
pub struct SpecialState {
    pub bank: u32,
    pub accelerate: u32
}

pub fn get_user_input<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}

#[derive(Debug)]
pub struct Game {
    pub starting_hp: u32,
    pub luck: u32,
    pub game_mode: String,
    pub num_teams: u32,
    pub team_player_count: Vec<u32>,
    pub teams: Vec<Vec<Player>>
}

impl Game {
    fn new(starting_hp: u32, luck: u32, game_mode: String, num_teams: u32, team_player_count: Vec<u32>, teams: Vec<Vec<Player>>) -> Self {
        Game { starting_hp, luck, game_mode, num_teams, team_player_count, teams }
    }

    pub fn display(&self) {
        println!("Game Settings:");
        println!("Starting HP: {}", self.starting_hp);
        println!("Luck: {}", self.luck);
        println!("Game Mode: {}", self.game_mode);
        println!("Number of Teams: {}", self.num_teams);
        println!("Team Player Count: {:?}", self.team_player_count);
        println!("Players: {:?}", self.teams);
    }
}

pub fn setup_game() -> Game {
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

    let mut teams = Vec::new();
    for team in 1..=num_teams {
        let mut players = Vec::new();
        for player in 1..=team_player_count[(team - 1) as usize] {
            println!("Is Player {} in Team {} a CPU? (true/false):", player, team);
            let is_cpu: bool = get_user_input();
            players.push(Player { is_cpu, ..Player::default() });
        }
        teams.push(players);
    }

    let game = Game::new(starting_hp, luck, game_mode, num_teams, team_player_count, teams);

    game
}