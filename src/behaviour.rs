use bevy::prelude::*;

const PAWN_HEALTH: f32 = 50.0;
const KNIGHT_HEALTH: f32 = 100.0;
const BISHOP_HEALTH: f32 = 100.0;
const ROOK_HEALTH: f32 = 200.0;
const QUEEN_HEALTH: f32 = 150.0;
const KING_HEALTH: f32 = 300.0;

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
