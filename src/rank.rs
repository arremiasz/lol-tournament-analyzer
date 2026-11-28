pub enum Tier {
    Iron,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Emerald,
    Diamond,
}

pub enum Division {
    IV,
    III,
    II,
    I,
}

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
}
