use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResolution};

const CELL_SIZE: (f32, f32) = (10.0, 10.0);

pub struct Game {
    grid: Grid,
}

#[derive(Default, Resource)]
pub struct CellMap {
    pub grid: Vec<Entity>,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
        }
    }

    pub fn grid(&mut self) -> &mut Grid {
        &mut self.grid
    }
}

#[derive(Default, Resource)]
pub struct GridDelta {
    pub xs: Vec<(usize, bool)>,
}

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game of Life".to_string(),
                mode: WindowMode::Windowed,
                resolution: WindowResolution::new(
                    self.grid.width as f32 * CELL_SIZE.0,
                    self.grid.height as f32 * CELL_SIZE.1,
                ),
                ..default()
            }),
            ..default()
        });

        app.insert_resource(self.grid.clone())
            .insert_resource(CellMap {
                grid: Vec::with_capacity(self.grid.width * self.grid.height),
            })
            .init_resource::<GridDelta>()
            .add_plugins((default_plugins, bevy_framepace::FramepacePlugin))
            .add_systems(Startup, (set_fps, init))
            .add_systems(Update, (step, draw));
    }
}

#[derive(Component)]
struct Camera;

fn init(mut commands: Commands, grid: Res<Grid>, mut cell_map: ResMut<CellMap>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                grid.width as f32 * CELL_SIZE.0 / 2.0,
                grid.height as f32 * CELL_SIZE.1 / 2.0,
                0.0,
            ),
            ..default()
        },
        Camera,
    ));

    for x in 0..grid.width {
        for y in 0..grid.height {
            let color = if grid.grid[x + y * grid.width] {
                Color::rgb(1.0, 1.0, 1.0)
            } else {
                Color::rgb(0.0, 0.0, 0.0)
            };
            let entity = commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(CELL_SIZE.0, CELL_SIZE.1)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * 10.0,
                        y as f32 * 10.0,
                        0.0,
                    )),
                    ..default()
                })
                .id();

            cell_map.grid.push(entity);
        }
    }
}

fn set_fps(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = bevy_framepace::Limiter::from_framerate(10.0);
}

fn step(mut grid: ResMut<Grid>, mut delta: ResMut<GridDelta>) {
    let mut coords: Vec<usize> = Vec::new();

    for i in 0..grid.height {
        // x
        for j in 0..grid.width {
            let width = grid.width as f32;
            let height = grid.height as f32;

            let left = if j == 0 {
                (width - 1.0) as usize
            } else {
                (j as f32 - 1.0) as usize
            };

            let right = if j == grid.width - 1 {
                (0.0) as usize
            } else {
                (j as f32 + 1.0) as usize
            };

            let up = if i == 0 {
                (height - 1.0) as usize
            } else {
                (i as f32 - 1.0) as usize
            };

            let down = if i == grid.height - 1 {
                (0.0) as usize
            } else {
                (i as f32 + 1.0) as usize
            };

            let neighbours = grid.grid[grid.get_index(left, up)] as u8
                + grid.grid[grid.get_index(j, up)] as u8
                + grid.grid[grid.get_index(right, up)] as u8
                + grid.grid[grid.get_index(left, i)] as u8
                + grid.grid[grid.get_index(right, i)] as u8
                + grid.grid[grid.get_index(left, down)] as u8
                + grid.grid[grid.get_index(j, down)] as u8
                + grid.grid[grid.get_index(right, down)] as u8;

            if grid.grid[grid.get_index(j, i)] && (neighbours < 2 || neighbours > 3) {
                coords.push(grid.get_index(j, i));
            } else if !grid.grid[grid.get_index(j, i)] && neighbours == 3 {
                coords.push(grid.get_index(j, i));
            }
        }
    }

    for pos in coords {
        grid.grid[pos] ^= true;
        delta.xs.push((pos, grid.grid[pos]));
    }
}

fn draw(
    mut commands: Commands,
    delta: Res<GridDelta>,
    grid: Res<Grid>,
    mut cell_map: ResMut<CellMap>,
) {
    for (i, state) in delta.xs.iter() {
        let color = if *state {
            Color::rgb(1.0, 1.0, 1.0)
        } else {
            Color::rgb(0.0, 0.0, 0.0)
        };

        let x = i % grid.width;
        let y = i / grid.width;

        let transform =
            Transform::from_translation(Vec3::new(x as f32 * 10.0, y as f32 * 10.0, 0.0));
        let entity = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(CELL_SIZE.0, CELL_SIZE.1)),
                    ..default()
                },
                transform,
                ..default()
            })
            .id();

        cell_map.grid[*i] = entity;
    }
}

#[derive(Clone, Default, Resource)]
pub struct Grid {
    pub grid: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn set_index(&mut self, x: usize, y: usize) {
        let pos = self.get_index(x, y);
        self.grid[pos] = true;
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }
}
