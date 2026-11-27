#[derive(Debug, Clone, Copy, PartialEq)]
enum Role {
    Top,
    Jungle,
    Mid,
    Bot,
    Support
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Team {
    Blue,
    Red
}

struct PlayerPerformance {
    name: String,
    role: Role,
    champion: String,
    team: Team,
    kills: u32,
    deaths: u32,
    assists: u32,
    gold: u32
}

impl PlayerPerformance {
    fn kda(&self) -> f64 {
        f64::from(self.kills + self.assists) / f64::from(self.deaths.max(1))
    }
    fn kp(&self, team_kills: u32) -> f64 {
        if team_kills == 0 {
            return 0.0;
        }
        f64::from(self.kills + self.assists) / f64::from(team_kills)
    }
}

struct Game {
    performances: Vec<PlayerPerformance>
}

impl Game {
    fn team_kills(&self, team: Team) -> u32 {
        let mut total_kills = 0;
        for performance in &self.performances {
            if performance.team == team {
                total_kills += performance.kills;
            }
        }   
        total_kills
    }
    fn validate(&self) -> Result<(), String> {
        // Validate that each team has exactly 5 players and one player for each role
        for team in [Team::Blue, Team::Red] {
            let p_count = self.performances.iter().filter(|p| p.team == team).count();
            if p_count != 5 {
                return Err(format!("Team {:?} has {} players, expected 5.", team, p_count));
            }
            for role in [Role::Top, Role::Jungle, Role::Mid, Role::Bot, Role::Support] {
                let r_count = self.performances.iter().filter(|p| p.team == team && p.role == role).count();
                if r_count != 1 {
                    return Err(format!("Team {:?} has {} players in role {:?}, expected 1.", team, r_count, role));
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
                return Err(format!("Player {} has kills + assists ({}) exceeding team kills ({}).", performance.name, performance.kills + performance.assists, team_kills));
            }
        }
        Ok(())
    }
}



fn main() {
    let player1 = PlayerPerformance {
        name: String::from("Alex"),
        role: Role::Mid,
        champion: String::from("Ahri"),
        team: Team::Blue,
        kills: 10,
        deaths: 2,
        assists: 5,
        gold: 15000
    };
    let player2 = PlayerPerformance {
        name: String::from("Jordan"),
        role: Role::Jungle,
        champion: String::from("Lee Sin"),
        team: Team::Blue,
        kills: 8,
        deaths: 3,
        assists: 7,
        gold: 14000
    };
    let player3 = PlayerPerformance {
        name: String::from("Taylor"),
        role: Role::Top,
        champion: String::from("Darius"),
        team: Team::Red,
        kills: 4,
        deaths: 5,
        assists: 0,
        gold: 13000
    };
    let game = Game {
        performances: vec![player1, player2, player3]
    };

    for performance in &game.performances {
        let team_kills = game.team_kills(performance.team);
        println!("Player: {}, KDA: {:.2}, KP: {:.2}%", performance.name, performance.kda(), performance.kp(team_kills) * 100.0);
    }

    match game.validate() {
        Ok(_) => println!("Game data is valid."),
        Err(e) => println!("Game data validation error: {}", e),
    }
}