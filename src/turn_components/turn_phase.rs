use druid::Data;

#[derive(PartialEq, Clone, Data)]
pub enum TurnPhase {
    CombatMove,
    BuyUnits,
    Combat,
    NonCombatMove,
    PlaceUnits,
}
