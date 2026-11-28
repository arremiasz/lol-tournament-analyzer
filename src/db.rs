use sqlx::Row;
use sqlx::sqlite::SqlitePool;

pub struct Player {
    id: i64,
    riot_id: String,
    seed_lp: Option<u32>,
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
