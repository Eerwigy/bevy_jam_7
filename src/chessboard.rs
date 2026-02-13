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
        (interact, deselect, update_tile_colors, update_selected_text)
            .chain()
            .run_if(in_state(AppState::Main)),
    );
    app.register_type::<GridCoords>();
    app.register_type::<ChessGrid>();
}

#[derive(Component)]
pub struct SelectedText;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct TileGrid;

fn setup(
    mut commands: Commands,
    font: Res<FontsCollection>,
    fg: Res<SpritesFgCollection>,
    bg: Res<SpritesBgCollection>,
) {
    let mut chessgrid = ChessGrid::default();

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
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    (
                        Name::new("Turns Text"),
                        Text::new("Turns Left: "),
                        TextFont {
                            font: font.title.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    (
                        Name::new("Abilities Text"),
                        Text::new("Abilities Left: "),
                        TextFont {
                            font: font.title.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    (
                        Name::new("Selected Text"),
                        Text::new("Selected: "),
                        SelectedText,
                        TextFont {
                            font: font.title.clone(),
                            font_size: 32.0,
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
                        let grid_coords = GridCoords::new(x, y);
                        let mut square = p.spawn((
                            Name::new("Board Square"),
                            TileGrid,
                            grid_coords,
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

                        chessgrid.squares[x as usize][y as usize] = Some(square.id());

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

    commands.insert_resource(chessgrid);
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

fn deselect(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    selected: Query<Entity, With<Selected>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for entity in &selected {
            commands.entity(entity).remove::<Selected>();
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

fn update_selected_text(
    mut text_query: Query<&mut Text, With<SelectedText>>,
    children: Query<&Children>,
    selected_tile: Query<Entity, With<Selected>>,
    pieces: Query<&Piece>,
) {
    let mut text = text_query.single_mut().unwrap();

    let Ok(tile) = selected_tile.single() else {
        text.0 = "Selected: None".to_string();
        return;
    };

    if let Ok(children) = children.get(tile) {
        for child in children.iter() {
            if let Ok(piece) = pieces.get(child) {
                text.0 = format!(
                    "Selected:\n{:?} {:?}\nHealth: {}\nPress [ESC]\nto deselect",
                    piece.color, piece.kind, piece.health
                );
                return;
            }
        }
    }

    text.0 = "Selected: Empty".to_string();
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
