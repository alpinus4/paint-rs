use ggez::event::{self, EventHandler};
use ggez::graphics::{self, CanvasLoadOp, Color};
use ggez::{Context, ContextBuilder, GameError, GameResult};
use glam::*;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("Paint", "Mateusz Grzonka")
        .window_setup(ggez::conf::WindowSetup::default().title("Paint!"))
        .build()
        .expect("Error creating ggez context!");

    let paint = MyGame::new(&mut ctx);

    event::run(ctx, event_loop, paint);
}

struct Segment {
    from: Vec2,
    to: Vec2,
}

struct MyGame {
    current_segments: Vec<Segment>,
    all_segments: Vec<Segment>,
    drawing: bool,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            current_segments: Vec::new(),
            all_segments: Vec::new(),
            drawing: false,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);

        let mb = &mut graphics::MeshBuilder::new();
            for segment in &self.all_segments {
                mb.line(
                    &[segment.from, segment.to],
                    4.0,
                    Color::new(1.0, 0.0, 0.0, 1.0),
                )?;
            }

            let line_mesh = graphics::Mesh::from_data(ctx, mb.build());

            canvas.draw(&line_mesh, graphics::DrawParam::default());

        if self.drawing {
            let mb = &mut graphics::MeshBuilder::new();
            for segment in &self.current_segments {
                mb.line(
                    &[segment.from, segment.to],
                    4.0,
                    Color::new(1.0, 0.0, 0.0, 1.0),
                )?;
            }

            let line_mesh = graphics::Mesh::from_data(ctx, mb.build());

            canvas.draw(&line_mesh, graphics::DrawParam::default());
        }

        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        let mut segment = Segment {
            from: Vec2::new(0.0, 0.0),
            to: Vec2::new(0.0, 0.0),
        };
        if !self.drawing {
            self.drawing = true;
            segment.from = Vec2::new(_x, _y);
        } else {
            if self.current_segments.len() == 0 {
                segment.from = Vec2::new(_x, _y);
            } else {
                let last = self.current_segments.last().unwrap();
                segment.from = last.to;
            }
        }
        segment.to = Vec2::new(_x, _y);
        self.current_segments.push(segment);
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        self.drawing = false;
        self.all_segments.append(&mut self.current_segments);
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        _dx: f32,
        _dy: f32,
    ) -> Result<(), GameError> {
        if self.drawing {
            let mut segment = Segment {
                from: Vec2::new(0.0, 0.0),
                to: Vec2::new(0.0, 0.0),
            };
            if self.current_segments.len() == 0 {
                segment.from = Vec2::new(_x, _y);
            } else {
                let last = self.current_segments.last().unwrap();
                segment.from = last.to;
            }
            segment.to = Vec2::new(_x, _y);
            self.current_segments.push(segment);
        }
        Ok(())
    }
}
