use tge::error::GameResult;
use tge::math::{Position, Size};
use tge::engine::{Engine, EngineBuilder};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::game::Game;

const TITLE: &str = "Responsive";

struct App {
    view_size: Size,
    canvas: Canvas,
    sky: Texture,
    background_x: f32,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let view_size = Size::<f32>::new(320.0, 256.0);
        let canvas = Canvas::new(engine, Size::new(view_size.width.round() as u32, view_size.height.round() as u32))?;
        let sky = Texture::load(engine, "assets/sky.png")?;
        Ok(Self {
            view_size,
            canvas,
            sky,
            background_x: 0.0,
        })
    }

    fn draw_scene(&mut self, engine: &mut Engine) {
        let sky_size = self.sky.size();
        engine.graphics().draw_sprite(
            Some(&self.sky),
            SpriteDrawParams::default()
                .region((0.0, 0.0, sky_size.width as f32 * 2.0, sky_size.height as f32))
                .position((self.background_x, 0.0)),
        );
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);

        self.background_x -= 1.0;
        if self.background_x <= -(self.sky.size().width as f32) {
            self.background_x = 0.0;
        }

        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().set_canvas(Some(&self.canvas));
        engine.graphics().clear(Color::BLACK);

        self.draw_scene(engine);

        engine.graphics().set_canvas(None);
        engine.graphics().clear(Color::BLACK);

        let graphics_size = engine.graphics().size();
        let position;
        let scale;
        if graphics_size.width / graphics_size.height <= self.view_size.width / self.view_size.height {
            scale = graphics_size.width / self.view_size.width;
            position = Position::new(0.0, (graphics_size.height - self.view_size.height * scale) / 2.0);
        } else {
            scale = graphics_size.height / self.view_size.height;
            position = Position::new((graphics_size.width - self.view_size.width * scale) / 2.0, 0.0);
        }

        engine.graphics().draw_sprite(
            Some(&self.canvas),
            SpriteDrawParams::default()
                .position(position)
                .scale((scale, scale)),
        );

        Ok(())
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((320.0 * 2.0, 256.0 * 2.0)))
        .graphics_config(GraphicsConfig::new()
            .default_filter(Filter::new(
                FilterMode::Nearest,
                FilterMode::Nearest,
                None,
            )))
        .build()?
        .run_with(App::new)
}
