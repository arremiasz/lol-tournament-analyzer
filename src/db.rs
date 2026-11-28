use sqlx::sqlite::SqlitePool;

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
