use strum::EnumIter;

#[derive(PartialEq, Eq, Clone, Debug, Hash, EnumIter)]
pub enum PlayerName {
    Saruman,
    Angmar,
    Mordor,
    Arnor,
    Gondor,
    Northmen,
    Lorien,
    Orcs,
    Rhun,
    HighElves,
    WoodlandRealm,
    Harad,
    DolGoldur,
    Freefolk,
    Dwarves,
    Rohan,
    Neutral,
}
