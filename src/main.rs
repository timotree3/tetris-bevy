use std::cmp::Ordering;
use std::time::Duration;

use bevy::app::App;
use bevy::prelude::{
    AssetServer, BuildChildren, Camera2dBundle, Changed, ClearColor, Color, Commands, Component,
    DespawnRecursiveExt, DetectChanges, Entity, Input, KeyCode, NodeBundle, Query, Res, ResMut,
    State, SystemSet, TextBundle, Transform, Vec3, With,
};
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::text::{TextAlignment, TextStyle};
use bevy::time::{Time, Timer};
use bevy::ui::{AlignItems, JustifyContent, PositionType, Size, Style, UiColor, Val};
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use tetrominoes::Tetromino;

mod tetrominoes;

const CELL_SIZE: usize = 30;
const ROWS: usize = 20;
const COLUMNS: usize = 10;
const GRID_START_X: f32 = -((COLUMNS * CELL_SIZE) as f32) / 2.0;
const GRID_START_Y: f32 = -((ROWS * CELL_SIZE) as f32) / 2.0;
const BACKGROUND: Color = Color::GRAY;
const GRID_BACKGROUND: Color = Color::BLACK;

pub struct Score(u32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    GameOver,
    Playing,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Tile {
    x: i8,
    y: i8,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct FallingSegment {
    x_offset: i8,
    y_offset: i8,
}

impl FallingSegment {
    fn rotate_clockwise(self) -> FallingSegment {
        FallingSegment {
            x_offset: self.y_offset,
            y_offset: -self.x_offset,
        }
    }

    fn rotate_counterclockwise(self) -> FallingSegment {
        FallingSegment {
            x_offset: -self.y_offset,
            y_offset: self.x_offset,
        }
    }
}

#[derive(Component)]
struct GameOverText;

// Have some extra rows at the top in case a piece is placed above the screen
struct FullGrid([[bool; COLUMNS]; ROWS + 4]);
impl FullGrid {
    fn empty() -> FullGrid {
        FullGrid([[false; COLUMNS]; ROWS + 4])
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Tetris".to_string(),
            width: 500.0,
            height: 700.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(BACKGROUND))
        .add_plugins(DefaultPlugins)
        .add_state(GameState::Playing)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_game))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(fall)
                .with_system(handle_input)
                .with_system(clear_rows)
                .with_system(update_translation)
                .with_system(check_loss),
        )
        .add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(show_gameover))
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(check_restart))
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(hide_gameover))
        .run();
}

fn tile_sprite(x: i8, y: i8, color: Color) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(
                GRID_START_X + x as f32 * CELL_SIZE as f32,
                GRID_START_Y + y as f32 * CELL_SIZE as f32,
                0.0,
            ),
            scale: Vec3::new(CELL_SIZE as f32, CELL_SIZE as f32, 0.0),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Grid background
    let width = COLUMNS as f32 * CELL_SIZE as f32;
    let height = ROWS as f32 * CELL_SIZE as f32;
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: GRID_BACKGROUND,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(
                GRID_START_X + (width / 2.0) - (CELL_SIZE as f32 / 2.0),
                GRID_START_Y + (height / 2.0) - (CELL_SIZE as f32 / 2.0),
                0.0,
            ),
            scale: Vec3::new(width, height, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    // Rng
    commands.insert_resource(SmallRng::from_entropy());
}

fn start_game(
    mut commands: Commands,
    mut rng: ResMut<SmallRng>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    tiles: Query<Entity, With<Tile>>,
) {
    keyboard_input.reset_all();
    tiles.for_each(|entity| commands.entity(entity).despawn_recursive());

    commands.insert_resource(FallTimer(Timer::new(
        Duration::from_secs_f32(1.0 / 5.0),
        true,
    )));
    commands.insert_resource(FullGrid::empty());
    commands.insert_resource(Score(0));

    spawn(&mut commands, &mut rng);
}

fn show_gameover(score: Res<Score>, asset_server: Res<AssetServer>, mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: UiColor(Color::NONE),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    format!(
                        "Game Over! Score: {}\n Press any key to play again",
                        score.0
                    ),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER),
            );
        })
        .insert(GameOverText);
}

fn hide_gameover(mut commands: Commands, text: Query<Entity, With<GameOverText>>) {
    text.for_each(|entity| commands.entity(entity).despawn_recursive());
}

fn check_restart(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keyboard_input.get_just_pressed().len() != 0 {
        game_state.set(GameState::Playing).unwrap();
    }
}

struct FallTimer(Timer);

fn is_full(x: i8, y: i8, full_grid: &FullGrid) -> bool {
    y < ROWS as i8 && full_grid.0[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()]
}

fn can_fit(mut segments: impl Iterator<Item = Tile>, full_grid: &FullGrid) -> bool {
    segments.all(|Tile { x, y }| in_bounds(x, y) && !is_full(x, y, full_grid))
}

fn can_fall(segments: impl Iterator<Item = Tile>, full_grid: &FullGrid) -> bool {
    can_fit(
        segments.map(|Tile { x, y }| Tile { x, y: y - 1 }),
        full_grid,
    )
}

fn spawn(commands: &mut Commands, rng: &mut SmallRng) {
    let focal_x = 6;
    let focal_y = ROWS;
    let tetromino = Tetromino::random(rng);
    for segment in tetromino.shape {
        let x = (focal_x as i8) + segment.x_offset;
        let y = (focal_y as i8) + segment.y_offset;
        commands
            .spawn()
            .insert_bundle(tile_sprite(x, y, tetromino.color))
            .insert(Tile { x, y })
            .insert(segment);
    }
}

fn fall(
    time: Res<Time>,
    mut rng: ResMut<SmallRng>,
    mut timer: ResMut<FallTimer>,
    mut segment_ents: Query<(Entity, &mut Tile, &FallingSegment)>,
    mut commands: Commands,
    mut full_grid: ResMut<FullGrid>,
) {
    let times = timer.0.tick(time.delta()).times_finished_this_tick();
    for _ in 0..times {
        if can_fall(segment_ents.iter().map(|(_, t, _)| *t), &full_grid) {
            for (_, mut tile, _) in &mut segment_ents {
                tile.y -= 1;
            }
        } else {
            for (entity, tile, _) in &segment_ents {
                commands.entity(entity).remove::<FallingSegment>();
                full_grid.0[usize::try_from(tile.y).unwrap()][usize::try_from(tile.x).unwrap()] =
                    true;
            }
            spawn(&mut commands, &mut rng);
        }
    }
}

fn lines_to_score(lines: u8) -> u32 {
    match lines {
        1 => 100,
        2 => 300,
        3 => 500,
        4 => 800,
        _ => panic!("At most 4 lines can be cleared at once"),
    }
}

fn clear_rows(
    mut score: ResMut<Score>,
    mut full_grid: ResMut<FullGrid>,
    mut tiles: Query<(Entity, &mut Tile)>,
    mut commands: Commands,
) {
    if !full_grid.is_changed() {
        return;
    }
    let mut cleared = 0;
    for y in (0..ROWS).rev() {
        if full_grid.0[y] == [true; COLUMNS] {
            full_grid.0[y..].rotate_left(1);
            *full_grid.0.last_mut().unwrap() = [false; COLUMNS];
            for (entity, mut tile) in &mut tiles {
                match tile.y.cmp(&(y as i8)) {
                    Ordering::Less => {}
                    Ordering::Equal => commands.entity(entity).despawn(),
                    Ordering::Greater => {
                        tile.y -= 1;
                    }
                }
            }
            cleared += 1
        }
    }
    if cleared != 0 {
        score.0 += lines_to_score(cleared)
    }
}

fn check_loss(full_grid: Res<FullGrid>, mut game_state: ResMut<State<GameState>>) {
    if !full_grid.is_changed() {
        return;
    }

    if full_grid.0[ROWS..].iter().any(|row| *row != [false; 10]) {
        game_state.set(GameState::GameOver).unwrap();
    }
}

fn in_bounds(x: i8, y: i8) -> bool {
    // pieces are allowed to move above the screen,
    // you just lose if the piece is *placed* above the screen.
    (0..COLUMNS as i8).contains(&x) && (0..).contains(&y)
}

fn update_segment(
    tile: &mut Tile,
    segment: &mut FallingSegment,
    left: bool,
    right: bool,
    z: bool,
    x: bool,
) {
    let mut focal_point_x = tile.x - segment.x_offset;
    let focal_point_y = tile.y - segment.y_offset;
    if left {
        focal_point_x -= 1;
    }
    if right {
        focal_point_x += 1;
    }
    if z {
        *segment = segment.rotate_counterclockwise();
    }
    if x {
        *segment = segment.rotate_clockwise();
    }
    *tile = Tile {
        x: focal_point_x + segment.x_offset,
        y: focal_point_y + segment.y_offset,
    }
}

fn handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Tile, &mut FallingSegment)>,
    full_grid: Res<FullGrid>,
    mut fall_timer: ResMut<FallTimer>,
) {
    let left = keyboard_input.just_pressed(KeyCode::Left);
    let right = keyboard_input.just_pressed(KeyCode::Right);
    let z = keyboard_input.just_pressed(KeyCode::Z);
    let x = keyboard_input.just_pressed(KeyCode::X);
    if keyboard_input.just_pressed(KeyCode::Down) {
        let new_duration = fall_timer.0.duration() / 3;
        fall_timer.0.set_duration(new_duration);
    }
    if keyboard_input.just_released(KeyCode::Down) {
        let new_duration = fall_timer.0.duration() * 3;
        fall_timer.0.set_duration(new_duration);
    }
    if !left && !right && !z && !x {
        return;
    }
    let new_segments = query.iter().map(|(tile, segment)| {
        let mut new_tile = *tile;
        let mut new_segment = *segment;
        update_segment(&mut new_tile, &mut new_segment, left, right, z, x);
        new_tile
    });
    if can_fit(new_segments, &full_grid) {
        for (mut tile, mut segment) in &mut query {
            let mut new_tile = *tile;
            let mut new_segment = *segment;
            update_segment(&mut new_tile, &mut new_segment, left, right, z, x);
            if new_tile != *tile {
                *tile = new_tile;
            }
            if new_segment != *segment {
                *segment = new_segment;
            }
        }
    }
}

fn update_translation(mut tiles: Query<(&mut Transform, &Tile), Changed<Tile>>) {
    tiles.par_for_each_mut(COLUMNS, |(mut transform, tile)| {
        transform.translation.x = GRID_START_X + (tile.x as f32 * CELL_SIZE as f32);
        transform.translation.y = GRID_START_Y + (tile.y as f32 * CELL_SIZE as f32);
    })
}
