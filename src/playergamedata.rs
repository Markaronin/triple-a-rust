use crate::player::Player;

pub struct PlayerGameData {
    pub money: u64,
}
impl PlayerGameData {
    pub fn starting_value(player_name: &Player) -> Self {
        let money = match player_name {
            Player::Dwarves => 38,
            _ => 0,
        };
        Self { money }
    }
}
