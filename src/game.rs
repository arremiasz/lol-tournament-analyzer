#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Role {
    Top,
    Jungle,
    Mid,
    Bot,
    Support,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    Blue,
    Red,
}

pub struct PlayerPerformance {
    pub name: String,
    pub role: Role,
    pub champion: String,
    pub team: Team,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
    pub gold: u32,
}

impl PlayerPerformance {
    pub fn kda(&self) -> f64 {
        f64::from(self.kills + self.assists) / f64::from(self.deaths.max(1))
    }
    pub fn kp(&self, team_kills: u32) -> f64 {
        if team_kills == 0 {
            return 0.0;
        }
        f64::from(self.kills + self.assists) / f64::from(team_kills)
    }
    pub fn gold_per_minute(&self, duration_seconds: u32) -> f64 {
        if duration_seconds == 0 {
            return 0.0;
        }
        f64::from(self.gold) / (f64::from(duration_seconds) / 60.0)
    }
}

pub struct Game {
    pub performances: Vec<PlayerPerformance>,
    pub winning_team: Team,
    pub duration_seconds: u32,
}

impl Game {
    pub fn team_kills(&self, team: Team) -> u32 {
        let mut total_kills = 0;
        for performance in &self.performances {
            if performance.team == team {
                total_kills += performance.kills;
            }
        }
        total_kills
    }
    pub fn validate(&self) -> Result<(), String> {
        // Validate that each team has exactly 5 players and one player for each role
        for team in [Team::Blue, Team::Red] {
            let p_count = self.performances.iter().filter(|p| p.team == team).count();
            if p_count != 5 {
                return Err(format!(
                    "Team {:?} has {} players, expected 5.",
                    team, p_count
                ));
            }
            for role in [Role::Top, Role::Jungle, Role::Mid, Role::Bot, Role::Support] {
                let r_count = self
                    .performances
                    .iter()
                    .filter(|p| p.team == team && p.role == role)
                    .count();
                if r_count != 1 {
                    return Err(format!(
                        "Team {:?} has {} players in role {:?}, expected 1.",
                        team, r_count, role
                    ));
                }
            }
        }
        // Validate that each player is unique by name
        let mut names = std::collections::HashSet::new();
        for performance in &self.performances {
            if !names.insert(&performance.name) {
                return Err(format!("Duplicate player name found: {}", performance.name));
            }
        }
        // Validate that kills + assists do not exceed total team kills
        for performance in &self.performances {
            let team_kills = self.team_kills(performance.team);
            if performance.kills + performance.assists > team_kills {
                return Err(format!(
                    "Player {} has kills + assists ({}) exceeding team kills ({}).",
                    performance.name,
                    performance.kills + performance.assists,
                    team_kills
                ));
            }
        }
        // Validate that a game's duration is not zero
        if self.duration_seconds == 0 {
            return Err("Game duration cannot be zero.".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player(name: &str, team: Team, role: Role) -> PlayerPerformance {
        PlayerPerformance {
            name: name.to_string(),
            role,
            champion: "Test Champion".to_string(),
            team,
            kills: 0,
            deaths: 0,
            assists: 0,
            gold: 15000,
        }
    }

    // Indexes 0-4 are Blue and 5-9 are Red.
    fn valid_game() -> Game {
        let mut performances = Vec::new();
        for team in [Team::Blue, Team::Red] {
            for role in [Role::Top, Role::Jungle, Role::Mid, Role::Bot, Role::Support] {
                let name = format!("{:?} {:?}", team, role);
                performances.push(player(&name, team, role));
            }
        }
        Game {
            performances,
            winning_team: Team::Blue,
            duration_seconds: 1800,
        }
    }

    // Checks that validation fails for the expected reason
    fn assert_error(game: &Game, expected: &str) {
        let error = game.validate().unwrap_err();
        assert!(error.contains(expected), "unexpected error: {}", error);
    }

    #[test]
    fn accepts_valid_game() {
        assert_eq!(valid_game().validate(), Ok(()));
    }

    #[test]
    fn rejects_wrong_team_size() {
        let mut game = valid_game();
        game.performances.pop();
        assert_error(&game, "has 4 players");
    }

    #[test]
    fn rejects_duplicate_role() {
        let mut game = valid_game();
        game.performances[1].role = Role::Top;
        assert_error(&game, "players in role");
    }

    #[test]
    fn rejects_duplicate_name() {
        let mut game = valid_game();
        game.performances[1].name = game.performances[0].name.clone();
        assert_error(&game, "Duplicate player name");
    }

    #[test]
    fn accepts_one_kill_with_four_assists() {
        let mut game = valid_game();
        game.performances[0].kills = 1;
        for p in &mut game.performances[1..5] {
            p.assists = 1;
        }
        assert_eq!(game.validate(), Ok(()));
    }

    #[test]
    fn rejects_too_many_assists() {
        let mut game = valid_game();
        game.performances[0].kills = 1;
        game.performances[1].assists = 2;
        assert_error(&game, "exceeding team kills");
    }

    #[test]
    fn rejects_zero_duration() {
        let mut game = valid_game();
        game.duration_seconds = 0;
        assert_error(&game, "duration cannot be zero");
    }

    #[test]
    fn team_kills_only_counts_requested_team() {
        let mut game = valid_game();
        game.performances[0].kills = 10;
        game.performances[1].kills = 8;
        game.performances[5].kills = 4;
        assert_eq!(game.team_kills(Team::Blue), 18);
        assert_eq!(game.team_kills(Team::Red), 4);
    }

    #[test]
    fn calculates_stats() {
        let mut p = player("Alex", Team::Blue, Role::Mid);
        p.kills = 3;
        p.assists = 7;
        assert!((p.kda() - 10.0).abs() < 1e-9);
        assert!((p.kp(20) - 0.5).abs() < 1e-9);
        assert_eq!(p.kp(0), 0.0);
        assert!((p.gold_per_minute(1800) - 500.0).abs() < 1e-9);
    }
}
