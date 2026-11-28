use sqlx::sqlite::SqlitePool;

use crate::db;
use crate::rank::{parse_rank, rank_to_lp};

#[derive(Debug)]
pub enum RegisterError {
    InvalidRank(String),
    Database(sqlx::Error),
    InvalidRiotId(String),
}

pub async fn register_player(
    pool: &SqlitePool,
    riot_id: &str,
    rank_input: Option<&str>,
) -> Result<i64, RegisterError> {
    if riot_id.split_once('#').map_or(true, |(name, tag)| {
        name.trim().is_empty() || tag.trim().is_empty()
    }) {
        return Err(RegisterError::InvalidRiotId(format!(
            "Invalid Riot ID format: {}. Expected format: name#tag",
            riot_id
        )));
    }
    let seed_lp: Option<u32> = match rank_input {
        Some(rank_str) => match parse_rank(rank_str) {
            Ok(rank) => Some(rank_to_lp(rank)),
            Err(e) => return Err(RegisterError::InvalidRank(e)),
        },
        None => None,
    };

    db::add_player(pool, riot_id, seed_lp)
        .await
        .map_err(RegisterError::Database)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn test_db() -> (TempDir, SqlitePool) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.db");
        let pool = db::connect(path.to_str().unwrap()).await.unwrap();
        (dir, pool)
    }

    #[tokio::test]
    async fn registers_ranked_player() {
        let (_dir, pool) = test_db().await;

        let id = register_player(&pool, "Alex#NA1", Some("gold 2"))
            .await
            .unwrap();

        let players = db::list_players(&pool).await.unwrap();
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].id, id);
        assert_eq!(players[0].riot_id, "Alex#NA1");
        assert_eq!(players[0].seed_lp, Some(1400));
    }

    #[tokio::test]
    async fn registers_unranked_player() {
        let (_dir, pool) = test_db().await;

        register_player(&pool, "Alex#NA1", None).await.unwrap();

        let players = db::list_players(&pool).await.unwrap();
        assert_eq!(players.len(), 1);
        assert_eq!(players[0].seed_lp, None);
    }

    #[tokio::test]
    async fn rejects_invalid_rank() {
        let (_dir, pool) = test_db().await;

        let result = register_player(&pool, "Alex#NA1", Some("gld 2")).await;

        assert!(matches!(result, Err(RegisterError::InvalidRank(_))));
        assert!(db::list_players(&pool).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn rejects_invalid_riot_ids() {
        let (_dir, pool) = test_db().await;

        for riot_id in ["", "Alex", "#NA1", "Alex#", "#", "   #NA1", "Alex#   "] {
            let result = register_player(&pool, riot_id, None).await;
            assert!(
                matches!(result, Err(RegisterError::InvalidRiotId(_))),
                "should reject: {:?}",
                riot_id
            );
        }

        assert!(db::list_players(&pool).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn reports_duplicate_as_database_error() {
        let (_dir, pool) = test_db().await;
        register_player(&pool, "Alex#NA1", None).await.unwrap();

        let result = register_player(&pool, "alex#na1", None).await;

        assert!(matches!(result, Err(RegisterError::Database(_))));
        assert_eq!(db::list_players(&pool).await.unwrap().len(), 1);
    }
}
