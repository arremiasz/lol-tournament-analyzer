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

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            duration_seconds INTEGER NOT NULL CHECK(duration_seconds>0),
            winning_team TEXT NOT NULL CHECK (winning_team IN ('Blue', 'Red')),
            played_at INTEGER NOT NULL CHECK(played_at>0)
        );
        ",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS game_players (
            game_id INTEGER NOT NULL REFERENCES games(id),
            player_id INTEGER NOT NULL REFERENCES players(id),
            team TEXT NOT NULL CHECK (team IN ('Blue', 'Red')),
            role TEXT NOT NULL CHECK (role IN ('Top', 'Jungle', 'Mid', 'Bot', 'Support')),
            champion TEXT NOT NULL,
            kills INTEGER NOT NULL CHECK (kills>=0),
            deaths INTEGER NOT NULL CHECK (deaths>=0),
            assists INTEGER NOT NULL CHECK (assists>=0),
            gold INTEGER NOT NULL CHECK (gold>=0),
            UNIQUE (player_id, game_id),
            UNIQUE(role, team, game_id)
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
        add_player(&pool, "player1#yes", Some(100)).await.unwrap();

        let result = add_player(&pool, "PLAYER1#YES", Some(100)).await;

        assert!(result.is_err());
        assert_eq!(list_players(&pool).await.unwrap().len(), 1);
    }

    async fn insert_game(pool: &SqlitePool) -> i64 {
        sqlx::query(
            "INSERT INTO games (duration_seconds, winning_team, played_at)
             VALUES (1800, 'Blue', 1795824000)",
        )
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
    }

    async fn insert_game_player(
        pool: &SqlitePool,
        game_id: i64,
        player_id: i64,
        team: &str,
        role: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO game_players
             (game_id, player_id, team, role, champion, kills, deaths, assists, gold)
             VALUES (?, ?, ?, ?, 'Ahri', 0, 0, 0, 0)",
        )
        .bind(game_id)
        .bind(player_id)
        .bind(team)
        .bind(role)
        .execute(pool)
        .await?;
        Ok(())
    }

    fn assert_db_error(result: Result<(), sqlx::Error>, expected: &str) {
        let error = result.unwrap_err().to_string();
        assert!(error.contains(expected), "unexpected error: {}", error);
    }

    #[tokio::test]
    async fn accepts_valid_game_player() {
        let (_dir, pool) = test_db().await;
        let player_id = add_player(&pool, "Alex#NA1", None).await.unwrap();
        let game_id = insert_game(&pool).await;

        insert_game_player(&pool, game_id, player_id, "Blue", "Mid")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn rejects_missing_player() {
        let (_dir, pool) = test_db().await;
        let game_id = insert_game(&pool).await;

        let result = insert_game_player(&pool, game_id, 999, "Blue", "Mid").await;

        assert_db_error(result, "FOREIGN KEY constraint failed");
    }

    #[tokio::test]
    async fn rejects_missing_game() {
        let (_dir, pool) = test_db().await;
        let player_id = add_player(&pool, "Alex#NA1", None).await.unwrap();

        let result = insert_game_player(&pool, 999, player_id, "Blue", "Mid").await;

        assert_db_error(result, "FOREIGN KEY constraint failed");
    }

    #[tokio::test]
    async fn player_can_play_multiple_games_but_only_once_in_each() {
        let (_dir, pool) = test_db().await;
        let player_id = add_player(&pool, "Alex#NA1", None).await.unwrap();
        let first = insert_game(&pool).await;
        let second = insert_game(&pool).await;

        insert_game_player(&pool, first, player_id, "Blue", "Mid")
            .await
            .unwrap();
        insert_game_player(&pool, second, player_id, "Blue", "Mid")
            .await
            .unwrap();

        let result = insert_game_player(&pool, first, player_id, "Red", "Top").await;
        assert_db_error(result, "UNIQUE constraint failed");
    }

    #[tokio::test]
    async fn allows_one_of_each_role_per_team() {
        let (_dir, pool) = test_db().await;
        let first = add_player(&pool, "One#NA1", None).await.unwrap();
        let second = add_player(&pool, "Two#NA1", None).await.unwrap();
        let third = add_player(&pool, "Three#NA1", None).await.unwrap();
        let game_id = insert_game(&pool).await;

        insert_game_player(&pool, game_id, first, "Blue", "Mid")
            .await
            .unwrap();
        insert_game_player(&pool, game_id, second, "Red", "Mid")
            .await
            .unwrap();

        let result = insert_game_player(&pool, game_id, third, "Blue", "Mid").await;
        assert_db_error(result, "UNIQUE constraint failed");
    }
}
