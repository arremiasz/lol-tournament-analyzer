#[derive(Debug, PartialEq)]
pub enum Tier {
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Emerald,
    Diamond,
}

#[derive(Debug, PartialEq)]
pub enum Division {
    IV,
    III,
    II,
    I,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Standard(Tier, Division),
    Apex(u32),
}

pub fn rank_to_lp(rank: Rank) -> u32 {
    match rank {
        Rank::Standard(tier, division) => {
            let base_lp = match tier {
                Tier::Iron => 0,
                Tier::Bronze => 400,
                Tier::Silver => 800,
                Tier::Gold => 1200,
                Tier::Platinum => 1600,
                Tier::Emerald => 2000,
                Tier::Diamond => 2400,
            };
            let division_offset = match division {
                Division::IV => 0,
                Division::III => 100,
                Division::II => 200,
                Division::I => 300,
            };
            base_lp + division_offset
        }
        Rank::Apex(lp) => 2800 + lp,
    }
}

pub fn parse_rank(input: &str) -> Result<Rank, String> {
    let lowercase = input.to_lowercase();
    let parts: Vec<&str> = lowercase.split_whitespace().collect();

    if parts.len() != 2 {
        return Err(format!("Invalid rank format: {}", input));
    }

    if matches!(parts[0], "master" | "grandmaster" | "challenger") {
        let lp: u32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid LP value: {}", parts[1]))?;
        if lp > 10000 {
            return Err(format!("LP value too high: {}", lp));
        }
        return Ok(Rank::Apex(lp));
    }

    let tier = match parts[0] {
        "iron" => Tier::Iron,
        "bronze" => Tier::Bronze,
        "silver" => Tier::Silver,
        "gold" => Tier::Gold,
        "platinum" => Tier::Platinum,
        "emerald" => Tier::Emerald,
        "diamond" => Tier::Diamond,
        _ => return Err(format!("Unknown tier: {}", parts[0])),
    };

    let division = match parts[1] {
        "4" => Division::IV,
        "3" => Division::III,
        "2" => Division::II,
        "1" => Division::I,
        _ => return Err(format!("Unknown division: {}", parts[1])),
    };

    Ok(Rank::Standard(tier, division))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_to_lp() {
        assert_eq!(rank_to_lp(Rank::Standard(Tier::Iron, Division::I)), 300);
        assert_eq!(rank_to_lp(Rank::Standard(Tier::Bronze, Division::IV)), 400);
        assert_eq!(rank_to_lp(Rank::Standard(Tier::Silver, Division::III)), 900);
        assert_eq!(rank_to_lp(Rank::Standard(Tier::Gold, Division::II)), 1400);
        assert_eq!(
            rank_to_lp(Rank::Standard(Tier::Platinum, Division::I)),
            1900
        );
        assert_eq!(
            rank_to_lp(Rank::Standard(Tier::Emerald, Division::IV)),
            2000
        );
        assert_eq!(
            rank_to_lp(Rank::Standard(Tier::Diamond, Division::III)),
            2500
        );
        assert_eq!(rank_to_lp(Rank::Apex(50)), 2850);
        assert_eq!(rank_to_lp(Rank::Apex(100)), 2900);
        assert_eq!(rank_to_lp(Rank::Apex(200)), 3000);
    }

    #[test]
    fn parses_standard_ranks() {
        assert_eq!(
            parse_rank("gold 2"),
            Ok(Rank::Standard(Tier::Gold, Division::II))
        );
        assert_eq!(
            parse_rank("  Diamond 4 "),
            Ok(Rank::Standard(Tier::Diamond, Division::IV))
        );
    }

    #[test]
    fn parses_apex_ranks() {
        assert_eq!(parse_rank("master 250"), Ok(Rank::Apex(250)));
        assert_eq!(parse_rank("  GrandMaster   600 "), Ok(Rank::Apex(600)));
        assert_eq!(parse_rank("challenger 1200"), Ok(Rank::Apex(1200)));
        assert_eq!(parse_rank("master 0"), Ok(Rank::Apex(0)));
        assert_eq!(parse_rank("master 10000"), Ok(Rank::Apex(10000)));
    }

    #[test]
    fn rejects_invalid_input() {
        let inputs = [
            "",
            "gold",
            "gold 5",
            "wood 2",
            "gold 2 extra",
            "250",
            "master",
            "masters 250",
            "master abc",
            "master -1",
            "master 10001",
            "master 999999",
            "challengerxyz 5",
        ];
        for input in inputs {
            assert!(parse_rank(input).is_err(), "should reject: {:?}", input);
        }
    }
}
