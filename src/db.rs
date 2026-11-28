use sqlx::Row;
use sqlx::sqlite::SqlitePool;

pub struct Player {
    pub id: i64,
    pub riot_id: String,
    pub seed_lp: Option<u32>,
}

pub async fn add_player(
    pool: &SqlitePool,
    riot_id: &str,
    seed_lp: Option<u32>,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO players (riot_id, seed_lp) VALUES (?, ?)")
        .bind(riot_id)
        .bind(seed_lp)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}

pub async fn list_players(pool: &SqlitePool) -> Result<Vec<Player>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, riot_id, seed_lp FROM players ORDER BY id")
        .fetch_all(pool)
        .await?;

    let mut players = Vec::new();
    for row in rows {
        let id: i64 = row.try_get("id")?;
        let riot_id: String = row.try_get("riot_id")?;
        let seed_lp: Option<u32> = row.try_get("seed_lp")?;
        players.push(Player {
            id,
            riot_id,
            seed_lp,
        });
    }

    Ok(players)
}

pub async fn connect(path: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = sqlx::sqlite::SqliteConnectOptions::new()
        .filename(path)
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

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn test_db() -> (TempDir, SqlitePool) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.db");
        let pool = connect(path.to_str().unwrap()).await.unwrap();
        (dir, pool)
    }

    #[tokio::test]
    async fn adds_and_lists_player() {
        let (_dir, pool) = test_db().await;

        // add a player, list the players, then check the results
        let riot_id = "player1#yes";
        let seed_lp = Some(100);
        let player_id = add_player(&pool, riot_id, seed_lp).await.unwrap();
        let players = list_players(&pool).await.unwrap();
        assert_eq!(players.len(), 1);
        let player = &players[0];
        assert_eq!(player.id, player_id);
        assert_eq!(player.riot_id, riot_id);
        assert_eq!(player.seed_lp, seed_lp);
    }

    #[tokio::test]
    async fn unranked_player() {
        let (_dir, pool) = test_db().await;

        // add a player with no seed_lp, list the players, then check the results
        let riot_id = "player2#hello";
        let seed_lp = None;
        let player_id = add_player(&pool, riot_id, seed_lp).await.unwrap();
        let players = list_players(&pool).await.unwrap();
        assert_eq!(players.len(), 1);
        let player = &players[0];
        assert_eq!(player.id, player_id);
        assert_eq!(player.riot_id, riot_id);
        assert_eq!(player.seed_lp, seed_lp);
    }

    #[tokio::test]
    async fn duplicate_player() {
        let (_dir, pool) = test_db().await;

        // add a player, then try to add the same player again and check for an error
        let riot_id = "player1#yes";
        let dup_id = "PLAYER1#YES"; // test case-insensitivity
        let seed_lp = Some(100);
        let _player_id = add_player(&pool, riot_id, seed_lp).await.unwrap();
        let result = add_player(&pool, dup_id, seed_lp).await;
        assert!(result.is_err());
    }
}
