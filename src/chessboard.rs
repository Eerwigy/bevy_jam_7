use crate::{AppState, assets::*, behaviour::*};
use bevy::prelude::*;

const DARK: Color = Color::hsl(200.0, 1.0, 0.25);
const LIGHT: Color = Color::hsl(200.0, 1.0, 0.5);
const HOVER: Color = Color::hsl(200.0, 1.0, 0.8);
const LEGAL: Color = Color::hsl(100.0, 0.5, 0.8);
const SELECT: Color = Color::hsl(10.0, 0.5, 0.8);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Main), setup);
    app.add_systems(
        Update,
        (interact, update_tile_colors)
            .chain()
            .run_if(in_state(AppState::Main)),
    );
    app.register_type::<GridCoords>();
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct TileGrid;

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct GridCoords(pub IVec2);

impl GridCoords {
    pub fn new(x: i32, y: i32) -> Self {
        Self(ivec2(x, y))
    }
}

fn setup(
    mut commands: Commands,
    font: Res<FontsCollection>,
    fg: Res<SpritesFgCollection>,
    bg: Res<SpritesBgCollection>,
) {
    commands
        .spawn((
            Name::new("Main Node"),
            Node {
                width: percent(100.0),
                height: percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn((
                Name::new("Left Panel"),
                Node {
                    height: percent(100.0),
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    left: px(10.0),
                    top: px(10.0),
                    ..default()
                },
                children![
                    (
                        Name::new("Money Text"),
                        Text::new("Money: "),
                        TextFont {
                            font: font.title.clone(),
                            ..default()
                        },
                    ),
                    (
                        Name::new("Turns Text"),
                        Text::new("Turns Left: "),
                        TextFont {
                            font: font.title.clone(),
                            ..default()
                        },
                    ),
                    (
                        Name::new("Abilities Text"),
                        Text::new("Abilities Left: "),
                        TextFont {
                            font: font.title.clone(),
                            ..default()
                        },
                    ),
                    (
                        Name::new("Selected Text"),
                        Text::new("Selected: "),
                        TextFont {
                            font: font.title.clone(),
                            ..default()
                        },
                    ),
                ],
            ));
            p.spawn((
                Name::new("Chessboard"),
                Node {
                    width: vh(80.0),
                    height: vh(80.0),
                    display: Display::Grid,
                    padding: px(10.0).into(),
                    grid_template_columns: RepeatedGridTrack::flex(8, 1.0),
                    grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
                    ..default()
                },
                BackgroundColor(DARK),
            ))
            .with_children(|p| {
                for x in 0_i32..8 {
                    for y in 0_i32..8 {
                        let mut square = p.spawn((
                            Name::new("Board Square"),
                            TileGrid,
                            GridCoords::new(x, y),
                            Node {
                                width: percent(100.0),
                                height: percent(100.0),
                                grid_row: GridPlacement::start(y as i16 + 1),
                                grid_column: GridPlacement::start(x as i16 + 1),
                                overflow: Overflow::visible(),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::End,
                                ..default()
                            },
                            BackgroundColor(if (x + y) % 2 == 0 { LIGHT } else { DARK }),
                            Interaction::None,
                            ZIndex(10),
                        ));

                        let Some((color, kind)) = get_piece(x, y) else {
                            continue;
                        };

                        let (fg, bg, health) = match kind {
                            PieceKind::Pawn => (fg.pawn.clone(), bg.pawn.clone(), PAWN_HEALTH),
                            PieceKind::Knight => {
                                (fg.knight.clone(), bg.knight.clone(), KNIGHT_HEALTH)
                            }
                            PieceKind::Bishop => {
                                (fg.bishop.clone(), bg.bishop.clone(), BISHOP_HEALTH)
                            }
                            PieceKind::Rook => (fg.rook.clone(), bg.rook.clone(), ROOK_HEALTH),
                            PieceKind::Queen => (fg.queen.clone(), bg.queen.clone(), QUEEN_HEALTH),
                            PieceKind::King => (fg.king.clone(), bg.king.clone(), KING_HEALTH),
                        };

                        square.insert(spawn_piece_node(color, bg, fg));
                        square.with_child((Piece {
                            color,
                            kind,
                            health,
                        },));
                    }
                }
            });
        });
}

fn interact(
    mut commands: Commands,
    query: Query<(Entity, &Interaction), (With<TileGrid>, Changed<Interaction>)>,
    selected: Query<Entity, With<Selected>>,
) {
    for (entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for e in &selected {
                commands.entity(e).remove::<Selected>();
            }

            commands.entity(entity).insert(Selected);
        }
    }
}

fn update_tile_colors(
    mut query: Query<
        (
            &GridCoords,
            &Interaction,
            Option<&Selected>,
            &mut BackgroundColor,
        ),
        With<TileGrid>,
    >,
) {
    for (grid, interaction, selected, mut bg) in &mut query {
        bg.0 = if selected.is_some() {
            SELECT
        } else if *interaction == Interaction::Hovered {
            HOVER
        } else if grid.0.element_sum() % 2 == 0 {
            LIGHT
        } else {
            DARK
        };
    }
}

fn spawn_piece_node(color: PieceColor, bg: Handle<Image>, fg: Handle<Image>) -> impl Bundle {
    let color = match color {
        PieceColor::White => Color::hsl(175.0, 1.0, 0.75),
        PieceColor::Black => Color::hsl(10.0, 1.0, 0.25),
    };

    children![
        (
            Name::new("Piece Node Bg"),
            Node {
                width: percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode {
                color,
                image: bg,
                ..default()
            }
        ),
        (
            Name::new("Piece Node Fg"),
            Node {
                width: percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode {
                image: fg,
                ..default()
            }
        ),
    ]
}

fn get_piece(x: i32, y: i32) -> Option<(PieceColor, PieceKind)> {
    if y == 1 {
        return Some((PieceColor::Black, PieceKind::Pawn));
    }

    if y == 6 {
        return Some((PieceColor::White, PieceKind::Pawn));
    }

    let kind = match x {
        0 | 7 => PieceKind::Rook,
        1 | 6 => PieceKind::Knight,
        2 | 5 => PieceKind::Bishop,
        3 => PieceKind::Queen,
        4 => PieceKind::King,
        _ => {
            unreachable!();
        }
    };

    if y == 0 {
        return Some((PieceColor::Black, kind));
    }

    if y == 7 {
        return Some((PieceColor::White, kind));
    }

    None
}
