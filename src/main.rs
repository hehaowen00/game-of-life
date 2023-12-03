use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawMode, FillOptions, Mesh, Rect, StrokeOptions};
use ggez::{event, graphics, timer, ContextBuilder, GameError, GameResult};

const CELL_SIZE: (f32, f32) = (20.0, 20.0);

pub struct GameState {
    grid: Vec<bool>,
    width: usize,
    height: usize,
    fps: u32,
    running: bool,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        GameState {
            grid: vec![false; height as usize * width as usize],
            width: 50,
            height: 50,
            fps: 10,
            running: true,
        }
    }

    fn set_index(&mut self, x: usize, y: usize) {
        let pos = self.get_index(x, y);
        self.grid[pos] = true;
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }
}

impl EventHandler<GameError> for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(self.fps) {
            // y
            let mut coords: Vec<usize> = Vec::new();
            for i in 0..self.height {
                // x
                for j in 0..self.width {
                    let width = self.width as f32;
                    let height = self.height as f32;
                    let left = if j == 0 {
                        (width - 1.0) as usize
                    } else {
                        (j as f32 - 1.0) as usize
                    };
                    let right = if j == self.width - 1 {
                        (0.0) as usize
                    } else {
                        (j as f32 + 1.0) as usize
                    };
                    let up = if i == 0 {
                        (height - 1.0) as usize
                    } else {
                        (i as f32 - 1.0) as usize
                    };
                    let down = if i == self.height - 1 {
                        (0.0) as usize
                    } else {
                        (i as f32 + 1.0) as usize
                    };
                    let neighbours = self.grid[self.get_index(left, up)] as u8
                        + self.grid[self.get_index(j, up)] as u8
                        + self.grid[self.get_index(right, up)] as u8
                        + self.grid[self.get_index(left, i)] as u8
                        + self.grid[self.get_index(right, i)] as u8
                        + self.grid[self.get_index(left, down)] as u8
                        + self.grid[self.get_index(j, down)] as u8
                        + self.grid[self.get_index(right, down)] as u8;

                    if self.grid[self.get_index(j, i)] && (neighbours < 2 || neighbours > 3) {
                        coords.push(self.get_index(j, i));
                    } else if !self.grid[self.get_index(j, i)] && neighbours == 3 {
                        coords.push(self.get_index(j, i));
                    }
                }
            }

            for grid in coords {
                self.grid[grid] ^= true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([1.0, 1.0, 1.0, 1.0]));

        for i in 0..self.height {
            for j in 0..self.width {
                let mut draw = DrawMode::Stroke(StrokeOptions::DEFAULT);
                if self.grid[self.get_index(j, i)] {
                    draw = DrawMode::Fill(FillOptions::DEFAULT);
                }
                let rect = Mesh::new_rectangle(
                    ctx,
                    draw,
                    Rect::new(
                        j as f32 * CELL_SIZE.1,
                        i as f32 * CELL_SIZE.0,
                        CELL_SIZE.0,
                        CELL_SIZE.1,
                    ),
                    Color::BLACK,
                )?;
                canvas.draw(&rect, Vec2::new(0.0, 0.0))
            }
        }

        canvas.finish(ctx)
    }
}

fn main() -> GameResult<()> {
    let width = 50;
    let height = 50;
    let mut game = GameState::new(width, height);

    game.set_index(6, 6);
    game.set_index(6, 7);
    game.set_index(6, 8);
    game.set_index(5, 8);
    game.set_index(4, 7);

    let (ctx, evt_loop) = ContextBuilder::new("Game of Life", "hehaowen00")
        .window_setup(WindowSetup::default().title("Game of Life"))
        .window_mode(
            WindowMode::default()
                .dimensions(width as f32 * CELL_SIZE.0, height as f32 * CELL_SIZE.1)
                .resizable(false),
        )
        .build()?;

    event::run(ctx, evt_loop, game);
}
