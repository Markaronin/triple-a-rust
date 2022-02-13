use crate::{terrain::Terrain, util::AttackingOrDefending};

#[derive(PartialEq, Eq, Hash)]
pub enum TerrainPreference {
    Open,
    Wilderness,
    Levy,
    Woodland,
    Dwarven,
    Ambusher,
    Relentless,
    Unyielding,
    Flying,
    Fortification,
}
impl TerrainPreference {
    pub fn get_bonus(
        &self,
        terrain: &Terrain,
        attacking_or_defending: &AttackingOrDefending,
    ) -> i64 {
        match self {
            TerrainPreference::Open => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -1,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -1,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -1,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 3,
                    AttackingOrDefending::Defending => 1,
                },
            },
            TerrainPreference::Wilderness => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -2,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => -1,
                },
            },
            TerrainPreference::Levy => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 0,
                },
            },
            TerrainPreference::Woodland => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -1,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -1,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 2,
                    AttackingOrDefending::Defending => 3,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => -1,
                },
            },
            TerrainPreference::Dwarven => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -1,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -1,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => -1,
                },
            },
            TerrainPreference::Ambusher => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => -1,
                },
            },
            TerrainPreference::Relentless => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 2,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => -1,
                },
            },
            TerrainPreference::Unyielding => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 2,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -1,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 3,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => -1,
                },
            },
            TerrainPreference::Flying => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 1,
                    AttackingOrDefending::Defending => 1,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -2,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => -2,
                    AttackingOrDefending::Defending => -2,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
            },
            TerrainPreference::Fortification => match terrain {
                Terrain::Mountain => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 3,
                },
                Terrain::Hill => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 3,
                },
                Terrain::Cave => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 3,
                },
                Terrain::Marsh => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Forest => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::River => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
                Terrain::Settlement => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 3,
                },
                Terrain::Plains => match attacking_or_defending {
                    AttackingOrDefending::Attacking => 0,
                    AttackingOrDefending::Defending => 0,
                },
            },
        }
    }
}
