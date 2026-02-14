use crate::{AppState, assets::*, behaviour::*};
use bevy::{platform::collections::HashSet, prelude::*};

const DARK: Color = Color::hsl(200.0, 1.0, 0.25);
const LIGHT: Color = Color::hsl(200.0, 1.0, 0.5);
const HOVER: Color = Color::hsl(200.0, 1.0, 0.8);
const LEGAL: Color = Color::hsl(100.0, 0.5, 0.8);
const SELECT: Color = Color::hsl(10.0, 0.5, 0.8);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Main), setup);
    app.add_systems(
        Update,
        (
            interact,
            deselect,
            find_legal_moves,
            update_tile_colors,
            update_selected_text,
        )
            .chain()
            .run_if(in_state(AppState::Main)),
    );
    app.register_type::<GridCoords>();
    app.register_type::<ChessGrid>();
}

#[derive(Component)]
pub struct SelectedText;

#[derive(Component)]
pub struct SelectedSquare;

#[derive(Component)]
pub struct LegalSquare;

#[derive(Component)]
pub struct TileGrid;

#[derive(Component)]
pub struct PieceNode;

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
                        let mut piece = None;
                        square.with_children(|p| {
                            let id = p
                                .spawn((
                                    Name::new("Piece"),
                                    Piece {
                                        color,
                                        kind,
                                        health,
                                    },
                                ))
                                .id();

                            piece = Some(id);
                        });

                        chessgrid.pieces[x as usize][y as usize] = piece;
                    }
                }
            });
        });

    commands.insert_resource(chessgrid);
}

fn interact(
    mut commands: Commands,
    mut chessgrid: ResMut<ChessGrid>,
    query: Query<
        (Entity, &Interaction, &GridCoords, Option<&LegalSquare>),
        (With<TileGrid>, Changed<Interaction>),
    >,
    children: Query<&Children>,
    selected: Query<(Entity, &GridCoords), With<SelectedSquare>>,
    legal_tiles: Query<Entity, With<LegalSquare>>,
    pieces: Query<&Piece>,
) {
    for (clicked_entity, interaction, clicked_coords, is_legal) in &query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        if is_legal.is_some() {
            let Ok((from_entity, from_coords)) = selected.single() else {
                return;
            };

            let Ok(children) = children.get(from_entity) else {
                return;
            };

            let mut piece_entity = None;

            for child in children.iter() {
                if pieces.get(child).is_ok() {
                    piece_entity = Some(child);
                    break;
                }
            }

            let Some(piece_entity) = piece_entity else {
                return;
            };

            chessgrid.pieces[from_coords.0.x as usize][from_coords.0.y as usize] = None;
            chessgrid.pieces[clicked_coords.0.x as usize][clicked_coords.0.y as usize] =
                Some(piece_entity);

            commands.entity(clicked_entity).add_child(piece_entity);

            commands.entity(from_entity).remove::<SelectedSquare>();

            for entity in &legal_tiles {
                commands.entity(entity).remove::<LegalSquare>();
            }

            return;
        }

        for (entity, _) in &selected {
            commands.entity(entity).remove::<SelectedSquare>();
        }

        commands.entity(clicked_entity).insert(SelectedSquare);
    }
}

fn deselect(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    selected: Query<Entity, With<SelectedSquare>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        for entity in &selected {
            commands.entity(entity).remove::<SelectedSquare>();
        }
    }
}

fn find_legal_moves(
    mut commands: Commands,
    chessgrid: Res<ChessGrid>,
    children: Query<&Children>,
    selected_tile: Query<(Entity, &GridCoords), With<SelectedSquare>>,
    legal_tiles: Query<Entity, With<LegalSquare>>,
    pieces: Query<&Piece>,
) {
    for entity in &legal_tiles {
        commands.entity(entity).remove::<LegalSquare>();
    }

    let Ok((tile_entity, grid_coords)) = selected_tile.single() else {
        return;
    };

    let Ok(children) = children.get(tile_entity) else {
        return;
    };

    let mut piece = None;

    for child in children.iter() {
        if let Ok(p) = pieces.get(child) {
            piece = Some(*p);
            break;
        }
    }

    let Some(piece) = piece else {
        return;
    };

    let moves = if piece.color == PieceColor::White {
        match piece.kind {
            PieceKind::Pawn => WhitePawnBehaviour::get_legal_moves(*grid_coords, *chessgrid),
            PieceKind::Knight => KnightBehaviour::get_legal_moves(*grid_coords, *chessgrid),

            _ => HashSet::default(),
        }
    } else {
        HashSet::default()
    };

    for coords in moves {
        let square_entity = chessgrid.get_square(coords);
        commands.entity(square_entity).insert(LegalSquare);
    }
}

fn update_tile_colors(
    mut query: Query<
        (
            &GridCoords,
            &Interaction,
            Option<&SelectedSquare>,
            Option<&LegalSquare>,
            &mut BackgroundColor,
        ),
        With<TileGrid>,
    >,
) {
    for (grid, interaction, selected, legal, mut bg) in &mut query {
        bg.0 = if selected.is_some() {
            SELECT
        } else if legal.is_some() {
            LEGAL
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
    selected_tile: Query<Entity, With<SelectedSquare>>,
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
            PieceNode,
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
            PieceNode,
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
