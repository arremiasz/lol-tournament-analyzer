mod game;

use game::{Game, PlayerPerformance, Role, Team};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename("tracker.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            riot_id TEXT NOT NULL UNIQUE COLLATE NOCASE,
            seed_lp INTEGER CHECK(seed_lp >= 0)
        );
        ",
    )
    .execute(&pool)
    .await?;

    println!("Database ready.");

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

    Ok(())
}
