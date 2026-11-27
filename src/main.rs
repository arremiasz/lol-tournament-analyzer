#[derive(Debug)]
enum Role {
    Top,
    Jungle,
    Mid,
    Bot,
    Support
}

struct PlayerPerformance {
    name: String,
    role: Role,
    champion: String,
    kills: u32,
    deaths: u32,
    assists: u32,
    gold: u32
}

impl PlayerPerformance {
    fn kda(&self) -> f64 {
        if self.deaths == 0 {
            return (self.kills + self.assists) as f64;
        }
        (self.kills + self.assists) as f64 / self.deaths as f64
    }
}

fn main() {
    let player1 = PlayerPerformance {
        name: String::from("Alex"),
        role: Role::Mid,
        champion: String::from("Ahri"),
        kills: 10,
        deaths: 2,
        assists: 5,
        gold: 15000
    };
    println!("Player Name: {}", player1.name);
    println!("Champion: {}", player1.champion);
    println!("KDA: {}", player1.kda());
    println!("Role: {:?}", player1.role);
}
