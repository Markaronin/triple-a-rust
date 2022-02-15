use druid::Data;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Data)]
pub struct UnitGameData {
    pub current_hp: usize,
    pub movement_left: usize,
}
