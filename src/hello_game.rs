use cna::Microsoft::Xna::Framework::{Color, Game, GameTime, Vector2};
// Note: These modules and types are placeholders for when the binding is complete
// use cna::Microsoft::Xna::Framework::Graphics::{GraphicsDeviceManager, SpriteBatch, Texture2D};
use cna::Result;

pub struct HelloGame {
    // graphics: GraphicsDeviceManager,
    // sprite_batch: Option<SpriteBatch>,
    // logo: Option<Texture2D>,
    position: Vector2,
}

impl HelloGame {
    pub fn new() -> Self {
        Self {
            // graphics: GraphicsDeviceManager::new(),
            // sprite_batch: None,
            // logo: None,
            position: Vector2::default(),
        }
    }
}

impl Game for HelloGame {
    fn initialize(&mut self) -> Result<()> {
        // self.graphics.set_preferred_back_buffer_width(1280);
        // self.graphics.set_preferred_back_buffer_height(720);
        // self.graphics.apply_changes();
        Ok(())
    }

    fn load_content(&mut self) -> Result<()> {
        // self.sprite_batch = Some(SpriteBatch::new());
        // self.logo = Some(self.content.load::<Texture2D>("logo")?);
        Ok(())
    }

    fn update(&mut self, time: &GameTime) -> Result<()> {
        let total_seconds = time.total.as_secs_f32();
        self.position.x = 640.0 + 200.0 * total_seconds.sin();
        self.position.y = 360.0 + 200.0 * total_seconds.cos();
        Ok(())
    }

    fn draw(&mut self, _time: &GameTime) -> Result<()> {
        // self.graphics_device().clear(Color::CORNFLOWER_BLUE);
        // if let (Some(batch), Some(logo)) = (&self.sprite_batch, &self.logo) {
        //     batch.begin();
        //     batch.draw(logo, self.position, Color::WHITE);
        //     batch.end();
        // }
        Ok(())
    }
}
