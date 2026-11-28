mod db;
mod game;
mod rank;
mod register;

use db::add_player;
use game::{Game, PlayerPerformance, Role, Team};

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    let pool = db::connect("tracker.db").await?;

    match args.as_slice() {
        ["register", riot_id, rank] => {
            handle_register(&pool, riot_id, Some(rank)).await;
        }
        ["register", riot_id] => {
            handle_register(&pool, riot_id, None).await;
        }
        ["list"] => {
            let players = db::list_players(&pool).await?;
            for player in players {
                println!(
                    "ID: {}, Riot ID: {}, Seed LP: {}",
                    player.id,
                    player.riot_id,
                    player
                        .seed_lp
                        .map(|lp| lp.to_string())
                        .unwrap_or_else(|| "Unranked".to_string())
                );
            }
        }
        _ => {
            println!("Usage:");
            println!("  register <riot_id> [rank] - Register a player with optional rank");
            println!("  list - List all registered players");
        }
    }

    Ok(())
}

async fn handle_register(pool: &SqlitePool, riot_id: &str, rank: Option<&str>) {
    match register::register_player(pool, riot_id.trim(), rank).await {
        Ok(id) => println!("Registered player with ID: {}", id),
        Err(register::RegisterError::InvalidRank(msg)) => {
            println!("Error: Invalid rank - {}", msg);
        }
        Err(register::RegisterError::Database(e)) => {
            println!("Database error: {}", e);
        }
        Err(register::RegisterError::InvalidRiotId(msg)) => {
            println!("Error: Invalid Riot ID - {}", msg);
        }
    }
}
