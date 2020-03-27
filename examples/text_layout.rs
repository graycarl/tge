use tge::error::GameResult;
use tge::engine::{Engine, EngineBuilder};
use tge::event::{KeyAction, Event};
use tge::window::WindowConfig;
use tge::graphics::*;
use tge::keyboard::KeyCode;
use tge::game::Game;

const TITLE: &str = "Text Layout";

struct App {
    font: Font,
    text: String,
}

impl App {

    fn new(engine: &mut Engine) -> GameResult<Self> {
        let font = Font::load(engine, "assets/roboto-fonts/Roboto-Regular.ttf")?;
        Ok(Self {
            font,
            text: "Hello, world!\nPlease input something..".to_owned(),
        })
    }

}

impl Game for App {

    fn update(&mut self, engine: &mut Engine) -> GameResult {
        let title = format!("{} - FPS: {}", TITLE, engine.timer().real_time_fps().round());
        engine.window().set_title(title);
        Ok(())
    }

    fn render(&mut self, engine: &mut Engine) -> GameResult {
        engine.graphics().clear(Color::WHITE);

        let graphics_size = engine.graphics().size();
        engine.graphics().draw_text(
            &self.font,
            &self.text,
            TextDrawParams::default()
                .text_size(40.0)
                .wrap_width(graphics_size.width)
                .color(Color::BLACK),
        );

        Ok(())
    }

    fn event(&mut self, _: &mut Engine, event: Event) -> GameResult<bool> {
        match event {
            Event::ReceiveChar(character) => {
                if !character.is_control() {
                    self.text.push(character);
                }
            }
            Event::KeyboardInput { key, action, .. } => {
                if action == KeyAction::Down {
                    match key {
                        KeyCode::Enter => {
                            self.text.push('\n');
                        }
                        KeyCode::Backspace => {
                            self.text.pop();
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        Ok(false)
    }

}

fn main() -> GameResult {
    EngineBuilder::new()
        .window_config(WindowConfig::new()
            .title(TITLE)
            .inner_size((800, 600)))
        .build()?
        .run_with(App::new)
}
