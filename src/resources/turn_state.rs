#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TurnState {
    PlayerInput,
    PlayerTurn,
    NpcTurn,
}
