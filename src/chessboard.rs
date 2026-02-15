use crate::{
    AppState, Typewriter, assets::*, behaviour::*, generate_character_text, stats::TurnsStat,
};
use bevy::{audio::Sample, platform::collections::HashSet, prelude::*};
use bevy_seedling::sample::{AudioSample, SamplePlayer};
use rand::prelude::*;

const DARK: Color = Color::hsl(200.0, 1.0, 0.25);
const LIGHT: Color = Color::hsl(200.0, 1.0, 0.5);
const HOVER: Color = Color::hsl(200.0, 1.0, 0.8);
const LEGAL: Color = Color::hsl(100.0, 0.5, 0.8);
const SELECT: Color = Color::hsl(10.0, 0.5, 0.8);
const ATTACK: Color = Color::hsl(50.0, 0.9, 0.5);

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
            update_turns_text.run_if(resource_changed::<TurnsStat>),
            pass_turn.run_if(resource_changed::<ButtonInput<KeyCode>>),
        )
            .chain()
            .run_if(in_state(AppState::Main)),
    );
    app.insert_resource(TurnsStat(3));
    app.register_type::<GridCoords>();
    app.register_type::<ChessGrid>();
}

#[derive(Component)]
pub struct SelectedText;

#[derive(Component)]
pub struct TurnsText;

#[derive(Component)]
pub struct SelectedSquare;

#[derive(Component)]
pub struct LegalSquare;

#[derive(Component)]
pub struct AttackedSquare;

#[derive(Component)]
pub struct TileGrid;

#[derive(Component)]
pub struct PieceNode;

#[derive(Component)]
pub struct QueenBubbleText;

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
                        Name::new("Pass Text"),
                        Text::new("Press [P] to Pass\nor End Turn"),
                        TextFont {
                            font: font.title.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    (
                        Name::new("Turns Text"),
                        Text::new("Turns Left: 3"),
                        TurnsText,
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
            p.spawn((
                Name::new("Right Panel"),
                Node {
                    height: percent(100.0),
                    width: px(300.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    right: px(10.0),
                    top: px(10.0),
                    bottom: px(10.0),
                    padding: px(10.0).into(),
                    ..default()
                },
            ))
            .with_children(|panel| {
                panel
                    .spawn((
                        Name::new("Text Bubble"),
                        Node {
                            width: percent(100.0),
                            min_height: px(120.0),
                            margin: UiRect::bottom(px(20.0)),
                            padding: px(15.0).into(),
                            ..default()
                        },
                        BackgroundColor(Color::hsl(0.0, 0.0, 0.9)),
                    ))
                    .with_children(|bubble| {
                        bubble.spawn((
                            Name::new("Character Bubble Text"),
                            Text::new("Hello!"),
                            QueenBubbleText,
                            Typewriter {
                                full_text: generate_character_text(),
                                visible_chars: 0,
                                timer: Timer::from_seconds(0.06, TimerMode::Repeating),
                            },
                            TextColor(Color::BLACK),
                            TextFont {
                                font: font.title.clone(),
                                font_size: 22.0,
                                ..default()
                            },
                        ));
                    });

                panel
                    .spawn((
                        Name::new("Character"),
                        Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexEnd,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                    ))
                    .with_children(|p| {
                        p.spawn((
                            Name::new("Character Bg"),
                            ImageNode {
                                image: bg.queen.clone(),
                                color: Color::hsl(10.0, 1.0, 0.25),
                                ..default()
                            },
                            Node {
                                width: px(200.0),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                        ));

                        p.spawn((
                            Name::new("Character Fg"),
                            ImageNode {
                                image: fg.queen.clone(),
                                ..default()
                            },
                            Node {
                                width: px(200.0),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                        ));
                    });
            });
        });

    commands.insert_resource(chessgrid);
}

fn interact(
    mut commands: Commands,
    mut chessgrid: ResMut<ChessGrid>,
    mut turns: ResMut<TurnsStat>,
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
            if turns.0 == 0 {
                return;
            }

            let Ok((from_entity, from_coords)) = selected.single() else {
                return;
            };

            let Ok(from_children) = children.get(from_entity) else {
                return;
            };

            let mut piece_entity = None;
            let mut piece_nodes = Vec::new();

            for child in from_children.iter() {
                if pieces.get(child).is_ok() {
                    piece_entity = Some(child);
                } else {
                    if commands.get_entity(child).is_ok() {
                        piece_nodes.push(child);
                    }
                }
            }

            let Some(piece_entity) = piece_entity else {
                return;
            };

            chessgrid.pieces[from_coords.0.x as usize][from_coords.0.y as usize] = None;
            chessgrid.pieces[clicked_coords.0.x as usize][clicked_coords.0.y as usize] =
                Some(piece_entity);

            commands.entity(clicked_entity).add_child(piece_entity);

            for node in piece_nodes {
                commands.entity(clicked_entity).add_child(node);
            }

            commands.entity(from_entity).remove::<SelectedSquare>();

            for entity in &legal_tiles {
                commands.entity(entity).remove::<LegalSquare>();
            }

            turns.0 -= 1;
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
    turns: Res<TurnsStat>,
    children: Query<&Children>,
    selected_tile: Query<(Entity, &GridCoords), With<SelectedSquare>>,
    attacked_tiles: Query<Entity, With<AttackedSquare>>,
    legal_tiles: Query<Entity, With<LegalSquare>>,
    pieces: Query<&Piece>,
) {
    for entity in &legal_tiles {
        commands.entity(entity).remove::<LegalSquare>();
    }

    for entity in &attacked_tiles {
        commands.entity(entity).remove::<AttackedSquare>();
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

    let moves = if piece.color == PieceColor::White && turns.0 > 0 {
        match piece.kind {
            PieceKind::Pawn => WhitePawnBehaviour::get_legal_moves(*grid_coords, *chessgrid),
            PieceKind::Knight => KnightBehaviour::get_legal_moves(*grid_coords, *chessgrid),
            PieceKind::Bishop => BishopBehaviour::get_legal_moves(*grid_coords, *chessgrid),
            PieceKind::Rook => RookBehaviour::get_legal_moves(*grid_coords, *chessgrid),
            PieceKind::Queen => QueenBehaviour::get_legal_moves(*grid_coords, *chessgrid),
            PieceKind::King => KingBehaviour::get_legal_moves(*grid_coords, *chessgrid),
        }
    } else {
        HashSet::default()
    };

    for coords in moves {
        let square_entity = chessgrid.get_square(coords);
        commands.entity(square_entity).insert(LegalSquare);
    }
    let attacks = match piece.kind {
        PieceKind::Pawn => {
            if piece.color == PieceColor::White {
                WhitePawnBehaviour::get_attacks(*grid_coords, *chessgrid)
            } else {
                BlackPawnBehaviour::get_attacks(*grid_coords, *chessgrid)
            }
        }
        PieceKind::Knight => KnightBehaviour::get_attacks(*grid_coords, *chessgrid),
        PieceKind::Bishop => BishopBehaviour::get_attacks(*grid_coords, *chessgrid),
        PieceKind::Rook => RookBehaviour::get_attacks(*grid_coords, *chessgrid),
        PieceKind::Queen => QueenBehaviour::get_attacks(*grid_coords, *chessgrid),
        PieceKind::King => KingBehaviour::get_attacks(*grid_coords, *chessgrid),
    };

    for coords in attacks {
        if let Some(target_entity) = chessgrid.get_piece(coords) {
            if let Ok(target_piece) = pieces.get(target_entity) {
                if target_piece.color != piece.color {
                    let square_entity = chessgrid.get_square(coords);
                    commands.entity(square_entity).insert(AttackedSquare);
                }
            }
        }
    }
}

fn update_tile_colors(
    mut query: Query<
        (
            &GridCoords,
            &Interaction,
            Option<&SelectedSquare>,
            Option<&LegalSquare>,
            Option<&AttackedSquare>,
            &mut BackgroundColor,
        ),
        With<TileGrid>,
    >,
) {
    for (grid, interaction, selected, legal, attack, mut bg) in &mut query {
        bg.0 = if selected.is_some() {
            SELECT
        } else if legal.is_some() {
            LEGAL
        } else if attack.is_some() {
            ATTACK
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

fn update_turns_text(mut text_query: Query<&mut Text, With<TurnsText>>, turns: Res<TurnsStat>) {
    let mut text = text_query.single_mut().unwrap();
    text.0 = format!("Turns Left: {}", turns.0);
}

fn pass_turn(
    mut commands: Commands,
    mut chessgrid: ResMut<ChessGrid>,
    mut turns: ResMut<TurnsStat>,
    mut pieces: Query<(Entity, &mut Piece)>,
    mut bubble_query: Query<(&mut Typewriter, &mut Text), With<QueenBubbleText>>,
    keys: Res<ButtonInput<KeyCode>>,
    sounds: Res<SoundsCollection>,
    children: Query<&Children>,
    tiles: Query<(Entity, &GridCoords), With<TileGrid>>,
) {
    if !keys.just_pressed(KeyCode::KeyP) {
        return;
    }

    commands.spawn(SamplePlayer::new(sounds.passturn.clone()));

    apply_damage_for_color(
        &mut commands,
        &mut chessgrid,
        &mut pieces,
        &children,
        PieceColor::White,
        sounds.vineboom.clone(),
    );

    let mut rng = rand::rng();

    for _ in 0..3 {
        for _ in 0..64 {
            let x = rng.random_range(0..8);
            let y = rng.random_range(0..8);

            let from = GridCoords::new(x, y);
            let Some(piece_ent) = chessgrid.get_piece(from) else {
                continue;
            };

            let (_, piece) = pieces.get(piece_ent).unwrap();

            if piece.color == PieceColor::White {
                continue;
            }

            let moves = match piece.kind {
                PieceKind::Pawn => BlackPawnBehaviour::get_legal_moves(from, *chessgrid),
                PieceKind::Knight => KnightBehaviour::get_legal_moves(from, *chessgrid),
                PieceKind::Bishop => BishopBehaviour::get_legal_moves(from, *chessgrid),
                PieceKind::Rook => RookBehaviour::get_legal_moves(from, *chessgrid),
                PieceKind::Queen => QueenBehaviour::get_legal_moves(from, *chessgrid),
                PieceKind::King => KingBehaviour::get_legal_moves(from, *chessgrid),
            };

            let len = moves.len();

            if len == 0 {
                continue;
            }

            let idx = rng.random_range(0..len);
            let to = moves.iter().nth(idx).unwrap();

            chessgrid.pieces[from.0.x as usize][from.0.y as usize] = None;
            chessgrid.pieces[to.0.x as usize][to.0.y as usize] = Some(piece_ent);

            let mut from_tile = None;
            let mut to_tile = None;

            for (tile_entity, coords) in &tiles {
                if *coords == from {
                    from_tile = Some(tile_entity);
                }
                if *coords == *to {
                    to_tile = Some(tile_entity);
                }
            }

            let (Some(from_tile), Some(to_tile)) = (from_tile, to_tile) else {
                return;
            };

            let mut extra_nodes = Vec::new();

            if let Ok(from_children) = children.get(from_tile) {
                for child in from_children.iter() {
                    if child != piece_ent {
                        extra_nodes.push(child);
                    }
                }
            }

            commands.entity(to_tile).add_child(piece_ent);

            for node in extra_nodes {
                commands.entity(to_tile).add_child(node);
            }

            break;
        }
    }

    apply_damage_for_color(
        &mut commands,
        &mut chessgrid,
        &mut pieces,
        &children,
        PieceColor::Black,
        sounds.vineboom.clone(),
    );

    if let Ok((mut typewriter, mut text)) = bubble_query.single_mut() {
        typewriter.full_text = generate_character_text();
        typewriter.visible_chars = 0;
        typewriter.timer.reset();
        text.0.clear();
    }
    turns.0 = 3;
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

fn apply_damage_for_color(
    commands: &mut Commands,
    chessgrid: &mut ChessGrid,
    pieces_query: &mut Query<(Entity, &mut Piece)>,
    children: &Query<&Children>,
    attacker_color: PieceColor,
    s: Handle<AudioSample>,
) {
    const DAMAGE: f32 = 10.0;

    let mut damage_events: Vec<(Entity, f32)> = Vec::new();

    for x in 0..8 {
        for y in 0..8 {
            let from = GridCoords::new(x as i32, y as i32);

            let Some(attacker_ent) = chessgrid.get_piece(from) else {
                continue;
            };

            let Ok((_, attacker_piece)) = pieces_query.get_mut(attacker_ent) else {
                continue;
            };

            if attacker_piece.color != attacker_color {
                continue;
            }

            let attacks = match attacker_piece.kind {
                PieceKind::Pawn => {
                    if attacker_piece.color == PieceColor::White {
                        WhitePawnBehaviour::get_attacks(from, *chessgrid)
                    } else {
                        BlackPawnBehaviour::get_attacks(from, *chessgrid)
                    }
                }
                PieceKind::Knight => KnightBehaviour::get_attacks(from, *chessgrid),
                PieceKind::Bishop => BishopBehaviour::get_attacks(from, *chessgrid),
                PieceKind::Rook => RookBehaviour::get_attacks(from, *chessgrid),
                PieceKind::Queen => QueenBehaviour::get_attacks(from, *chessgrid),
                PieceKind::King => KingBehaviour::get_attacks(from, *chessgrid),
            };

            for target_pos in attacks {
                if let Some(target_ent) = chessgrid.get_piece(target_pos) {
                    if let Ok((_, target_piece)) = pieces_query.get_mut(target_ent) {
                        if target_piece.color != attacker_color {
                            damage_events.push((target_ent, DAMAGE));
                        }
                    }
                }
            }
        }
    }

    for (entity, dmg) in damage_events {
        if let Ok((ent, mut piece)) = pieces_query.get_mut(entity) {
            piece.health -= dmg;

            if piece.health <= 0.0 {
                commands.spawn(SamplePlayer::new(s.clone()));
                for x in 0..8 {
                    for y in 0..8 {
                        if chessgrid.pieces[x][y] == Some(ent) {
                            chessgrid.pieces[x][y] = None;
                        }
                    }
                }

                for x in 0..8 {
                    for y in 0..8 {
                        if let Some(square_ent) = chessgrid.squares[x][y] {
                            if let Ok(tile_children) = children.get(square_ent) {
                                if tile_children.contains(&ent) {
                                    for child in tile_children.iter() {
                                        commands.entity(child).despawn();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
