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
}