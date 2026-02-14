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

    pub fn in_bounds(&self) -> bool {
        self.0.x >= 0 && self.0.x <= 7 && self.0.y >= 0 && self.0.y <= 7
    }
}

#[derive(Resource, Debug, Reflect, Default, Clone, Copy)]
#[reflect(Resource)]
pub struct ChessGrid {
    pub squares: [[Option<Entity>; 8]; 8],
    pub pieces: [[Option<Entity>; 8]; 8],
}

impl ChessGrid {
    pub fn get_square(&self, GridCoords(IVec2 { x, y }): GridCoords) -> Entity {
        self.squares[x as usize][y as usize].unwrap()
    }

    pub fn get_piece(&self, GridCoords(IVec2 { x, y }): GridCoords) -> Option<Entity> {
        self.pieces[x as usize][y as usize]
    }
}

pub trait PieceBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords>;
    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords>;
}

pub struct WhitePawnBehaviour;
pub struct BlackPawnBehaviour;
pub struct KnightBehaviour;
pub struct BishopBehaviour;
pub struct RookBehaviour;
pub struct QueenBehaviour;
pub struct KingBehaviour;

impl WhitePawnBehaviour {
    const ATTACKS: [IVec2; 2] = [IVec2::NEG_ONE, IVec2::new(1, -1)];
}

impl PieceBehaviour for WhitePawnBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut moves = HashSet::default();
        let potential = GridCoords(pos.0 - IVec2::Y);

        if potential.in_bounds() && grid.get_piece(potential).is_none() {
            moves.insert(potential);
        }

        moves
    }

    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut attack = HashSet::default();

        for offset in Self::ATTACKS {
            let potential = GridCoords(pos.0 + offset);
            if potential.in_bounds() && grid.get_piece(potential).is_some() {
                attack.insert(potential);
            }
        }

        attack
    }
}

impl BlackPawnBehaviour {
    const ATTACKS: [IVec2; 2] = [IVec2::ONE, IVec2::new(-1, 1)];
}

impl PieceBehaviour for BlackPawnBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut moves = HashSet::default();
        let potential = GridCoords(pos.0 + IVec2::Y);

        if potential.in_bounds() && grid.get_piece(potential).is_none() {
            moves.insert(potential);
        }

        moves
    }

    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut attack = HashSet::default();

        for offset in Self::ATTACKS {
            let potential = GridCoords(pos.0 + offset);
            if potential.in_bounds() && grid.get_piece(potential).is_some() {
                attack.insert(potential);
            }
        }

        attack
    }
}

impl KnightBehaviour {
    const OFFSETS: [IVec2; 8] = [
        IVec2::new(1, 2),
        IVec2::new(2, 1),
        IVec2::new(2, -1),
        IVec2::new(1, -2),
        IVec2::new(-1, -2),
        IVec2::new(-2, -1),
        IVec2::new(-2, 1),
        IVec2::new(-1, 2),
    ];
}

impl PieceBehaviour for KnightBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut moves = HashSet::default();

        for offset in Self::OFFSETS {
            let potential = GridCoords(pos.0 + offset);
            if potential.in_bounds() && grid.get_piece(potential).is_none() {
                moves.insert(potential);
            }
        }

        moves
    }

    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut attack = HashSet::default();

        for offset in Self::OFFSETS {
            let potential = GridCoords(pos.0 + offset);
            if potential.in_bounds() && grid.get_piece(potential).is_some() {
                attack.insert(potential);
            }
        }

        attack
    }
}

impl BishopBehaviour {
    const DIRECTIONS: [IVec2; 4] = [
        IVec2::ONE,
        IVec2::NEG_ONE,
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
    ];
}

impl PieceBehaviour for BishopBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        sliding_moves(pos, grid, &Self::DIRECTIONS)
    }

    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        sliding_attacks(pos, grid, &Self::DIRECTIONS)
    }
}

impl RookBehaviour {
    const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
}

impl PieceBehaviour for RookBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        sliding_moves(pos, grid, &Self::DIRECTIONS)
    }

    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        sliding_attacks(pos, grid, &Self::DIRECTIONS)
    }
}

impl QueenBehaviour {
    const DIRECTIONS: [IVec2; 8] = [
        IVec2::X,
        IVec2::NEG_X,
        IVec2::Y,
        IVec2::NEG_Y,
        IVec2::ONE,
        IVec2::NEG_ONE,
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
    ];
}

impl PieceBehaviour for QueenBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        sliding_moves(pos, grid, &Self::DIRECTIONS)
    }

    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        sliding_attacks(pos, grid, &Self::DIRECTIONS)
    }
}

impl KingBehaviour {
    const OFFSETS: [IVec2; 8] = [
        IVec2::X,
        IVec2::Y,
        IVec2::NEG_X,
        IVec2::NEG_Y,
        IVec2::ONE,
        IVec2::NEG_ONE,
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
    ];
}

impl PieceBehaviour for KingBehaviour {
    fn get_legal_moves(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut moves = HashSet::default();

        for offset in Self::OFFSETS {
            let potential = GridCoords(pos.0 + offset);
            if potential.in_bounds() && grid.get_piece(potential).is_none() {
                moves.insert(potential);
            }
        }

        moves
    }

    fn get_attacks(pos: GridCoords, grid: ChessGrid) -> HashSet<GridCoords> {
        let mut attacks = HashSet::default();

        for offset in Self::OFFSETS {
            let potential = GridCoords(pos.0 + offset);
            if potential.in_bounds() && grid.get_piece(potential).is_some() {
                attacks.insert(potential);
            }
        }

        attacks
    }
}

fn sliding_moves(pos: GridCoords, grid: ChessGrid, directions: &[IVec2]) -> HashSet<GridCoords> {
    let mut moves = HashSet::default();

    for dir in directions {
        let mut current = pos.0 + *dir;

        while GridCoords(current).in_bounds() {
            let potential = GridCoords(current);
            if grid.get_piece(potential).is_some() {
                break;
            }
            moves.insert(potential);
            current += *dir;
        }
    }

    moves
}

fn sliding_attacks(pos: GridCoords, grid: ChessGrid, directions: &[IVec2]) -> HashSet<GridCoords> {
    let mut attacks = HashSet::default();

    for dir in directions {
        let mut current = pos.0 + *dir;

        while GridCoords(current).in_bounds() {
            let potential = GridCoords(current);
            attacks.insert(potential);

            if grid.get_piece(potential).is_some() {
                break;
            }
            current += *dir;
        }
    }

    attacks
}
