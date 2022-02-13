use crate::player::PlayerName;

pub struct PlayerGameData {
    pub money: u64,
}
impl PlayerGameData {
    pub fn starting_value(player_name: &PlayerName) -> Self {
        let money = match player_name {
            PlayerName::Dwarves => 38,
            _ => 0,
        };
        Self { money }
    }
}
