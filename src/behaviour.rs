use bevy::{platform::collections::HashSet, prelude::*};

pub const PAWN_HEALTH: f32 = 50.0;
pub const KNIGHT_HEALTH: f32 = 100.0;
pub const BISHOP_HEALTH: f32 = 100.0;
pub const ROOK_HEALTH: f32 = 200.0;
pub const QUEEN_HEALTH: f32 = 150.0;
pub const KING_HEALTH: f32 = 300.0;

#[derive(Component, Reflect, Copy, Clone, Debug)]
#[reflect(Component)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
    pub health: f32,
}

#[derive(Reflect, Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Reflect, Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Component, Clone, Copy, Reflect, PartialEq, Eq, Hash)]
#[reflect(Component)]
pub struct GridCoords(pub IVec2);

impl GridCoords {
    pub fn new(x: i32, y: i32) -> Self {
        Self(ivec2(x, y))
    }
}

#[derive(Resource, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Resource)]
pub struct ChessGrid {
    pub squares: [[Option<Entity>; 8]; 8],
}
pub trait PieceBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords>;
}
