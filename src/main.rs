mod game;

use game::{Game, PlayerPerformance, Role, Team};

fn main() {
    let player1 = PlayerPerformance {
        name: String::from("Alex"),
        role: Role::Mid,
        champion: String::from("Ahri"),
        team: Team::Blue,
        kills: 10,
        deaths: 2,
        assists: 5,
        gold: 15000,
    };
    let player2 = PlayerPerformance {
        name: String::from("Jordan"),
        role: Role::Jungle,
        champion: String::from("Lee Sin"),
        team: Team::Blue,
        kills: 8,
        deaths: 3,
        assists: 7,
        gold: 14000,
    };
    let player3 = PlayerPerformance {
        name: String::from("Taylor"),
        role: Role::Top,
        champion: String::from("Darius"),
        team: Team::Red,
        kills: 4,
        deaths: 5,
        assists: 0,
        gold: 13000,
    };
    let game = Game {
        performances: vec![player1, player2, player3],
        winning_team: Team::Blue,
        duration_seconds: 1800,
    };

    for performance in &game.performances {
        let team_kills = game.team_kills(performance.team);
        println!(
            "Player: {}, KDA: {:.2}, KP: {:.2}%, Gold per minute: {:.0}",
            performance.name,
            performance.kda(),
            performance.kp(team_kills) * 100.0,
            performance.gold_per_minute(game.duration_seconds)
        );
    }

    match game.validate() {
        Ok(_) => println!("Game data is valid."),
        Err(e) => println!("Game data validation error: {}", e),
    }
}
