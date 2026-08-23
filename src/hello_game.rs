use std::fs::File;

use cna::extensions::graphics::RendererInfoExt;
use cna::Microsoft::Xna::Framework::Graphics::{
    BlendState, GraphicsResource, SpriteBatch, SpriteSortMode, Texture2D,
};
use cna::Microsoft::Xna::Framework::Input::{GamePad, Keyboard, Keys, Mouse};
use cna::Microsoft::Xna::Framework::{Color, Game, GameContext, GameTime, PlayerIndex, Vector2};
use cna::Result;

pub struct HelloGame {
    sprite_batch: Option<SpriteBatch>,
    logo: Option<Texture2D>,
    position: Vector2,
    velocity: Vector2,
}

impl HelloGame {
    pub const fn new() -> Self {
        Self {
            sprite_batch: None,
            logo: None,
            position: Vector2::Zero,
            velocity: Vector2::from_x_and_y(104.0, 74.0),
        }
    }
}

impl Game for HelloGame {
    fn LoadContent(&mut self, game: &mut GameContext<'_>) -> Result<()> {
        let device = game.GraphicsDevice()?;
        let mut logo_file =
            File::open("Content/logo.png").map_err(|error| cna::CnaError::Native {
                code: 5,
                message: format!("cannot open Content/logo.png: {error}"),
            })?;
        let logo = Texture2D::FromStreamWithGraphicsDeviceAndStream(&device, &mut logo_file)?;
        let viewport = device.Viewport()?;
        self.position = Vector2::from_x_and_y(
            (viewport.Width() - logo.Width()) as f32 * 0.5,
            (viewport.Height() - logo.Height()) as f32 * 0.5,
        );
        self.sprite_batch = Some(SpriteBatch::new(&device)?);
        self.logo = Some(logo);

        let renderer = device.renderer_info()?;
        println!(
            "cna-rust-template: renderer={} 3d={} depth_stencil={} max_texture={}",
            renderer.name,
            renderer.supports_3d,
            renderer.supports_depth_stencil,
            renderer.max_texture_dimension,
        );
        Ok(())
    }

    fn Update(&mut self, game: &mut GameContext<'_>, time: &GameTime) -> Result<()> {
        if Keyboard::GetState(game)?.IsKeyDown(Keys::Escape) {
            return game.Exit();
        }
        let _mouse = Mouse::GetState(game)?;
        let _gamepad = GamePad::GetState(game, PlayerIndex::One)?;
        let Some(logo) = &self.logo else {
            return Ok(());
        };
        let viewport = game.GraphicsDevice()?.Viewport()?;
        let elapsed = time.ElapsedGameTime().TotalSeconds() as f32;
        self.position += self.velocity * elapsed.min(0.1);

        let maximum_x = (viewport.Width() - logo.Width()).max(0) as f32;
        let maximum_y = (viewport.Height() - logo.Height()).max(0) as f32;
        if self.position.X < 0.0 {
            self.position.X = 0.0;
            self.velocity.X = self.velocity.X.abs();
        } else if self.position.X > maximum_x {
            self.position.X = maximum_x;
            self.velocity.X = -self.velocity.X.abs();
        }
        if self.position.Y < 0.0 {
            self.position.Y = 0.0;
            self.velocity.Y = self.velocity.Y.abs();
        } else if self.position.Y > maximum_y {
            self.position.Y = maximum_y;
            self.velocity.Y = -self.velocity.Y.abs();
        }
        Ok(())
    }

    fn Draw(&mut self, game: &mut GameContext<'_>, _time: &GameTime) -> Result<()> {
        game.GraphicsDevice()?
            .ClearWithColor(Color::CornflowerBlue)?;
        if let (Some(batch), Some(logo)) = (&mut self.sprite_batch, &self.logo) {
            let alpha_blend = BlendState::AlphaBlend;
            batch.BeginWithSortModeAndBlendState(SpriteSortMode::Deferred, &alpha_blend)?;
            if let Err(error) = batch.Draw(logo, self.position, Color::White) {
                let _ = batch.End();
                return Err(error);
            }
            batch.End()?;
        }
        Ok(())
    }

    fn UnloadContent(&mut self, _game: &mut GameContext<'_>) -> Result<()> {
        if let Some(batch) = &mut self.sprite_batch {
            batch.DisposeWithNoArguments()?;
        }
        if let Some(logo) = &mut self.logo {
            logo.DisposeWithNoArguments()?;
        }
        self.sprite_batch = None;
        self.logo = None;
        Ok(())
    }
}
