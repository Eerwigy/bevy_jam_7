use crate::{
    AppState,
    assets::{SpritesBgCollection, SpritesFgCollection},
};
use bevy::prelude::*;

const DARK: Color = Color::hsl(200.0, 1.0, 0.25);
const LIGHT: Color = Color::hsl(200.0, 1.0, 0.5);
const HOVER: Color = Color::hsl(200.0, 1.0, 0.8);
const LEGAL: Color = Color::hsl(100.0, 0.5, 0.8);

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Main), setup);
    app.add_systems(Update, interact.run_if(in_state(AppState::Main)));
    app.register_type::<GridCoords>();
}

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

fn setup(mut commands: Commands, fg: Res<SpritesFgCollection>, bg: Res<SpritesBgCollection>) {
    let chessboard = commands
        .spawn((
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
            for y in 0_i32..8 {
                for x in 0_i32..8 {
                    p.spawn((
                        Name::new("Board Square"),
                        TileGrid,
                        GridCoords::new(x, y),
                        Node {
                            width: percent(100.0),
                            height: percent(100.0),
                            grid_row: GridPlacement::start(x as i16 + 1),
                            grid_column: GridPlacement::start(y as i16 + 1),
                            overflow: Overflow::visible(),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::End,
                            ..default()
                        },
                        BackgroundColor(if (x + y) % 2 == 0 { LIGHT } else { DARK }),
                        Interaction::None,
                        ZIndex(10),
                        children![
                            (
                                Name::new("Piece Node Bg"),
                                Node {
                                    width: percent(100.0),
                                    position_type: PositionType::Absolute,
                                    ..default()
                                },
                                ImageNode {
                                    image: bg.pawn.clone(),
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
                                    image: fg.pawn.clone(),
                                    ..default()
                                }
                            ),
                        ],
                    ));
                }
            }
        })
        .id();

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
            children![(Name::new("Character"),),],
        ))
        .add_child(chessboard);
}

fn interact(
    mut query: Query<
        (&Interaction, &GridCoords, &mut BackgroundColor),
        (With<TileGrid>, Changed<Interaction>),
    >,
) {
    for (inter, grid, mut bg) in &mut query {
        match inter {
            Interaction::Pressed => {
                println!("{} clicked", grid.0);
                bg.0 = if grid.0.element_sum() % 2 == 0 {
                    LIGHT
                } else {
                    DARK
                };
            }
            Interaction::Hovered => {
                bg.0 = HOVER;
            }
            Interaction::None => {
                bg.0 = if grid.0.element_sum() % 2 == 0 {
                    LIGHT
                } else {
                    DARK
                };
            }
        }
    }
}
