use cna::Microsoft::Xna::Framework::{Color, Game, GameTime, Vector2, Vector3, Matrix};
use cna::Microsoft::Xna::Framework::Graphics::{GraphicsDeviceManager, SpriteBatch, Texture2D, BasicEffect};
use cna::Microsoft::Xna::Framework::Input::{Keyboard, Keys};
use cna::Result;

pub struct HelloGame {
    graphics: GraphicsDeviceManager,
    sprite_batch: Option<SpriteBatch>,
    logo: Option<Texture2D>,
    solid: Option<Texture2D>,
    cube_effect: Option<BasicEffect>,
    
    smoke_test: bool,
    drawn_frames: u32,
    animation_seconds: f32,
    renderer_banner_seconds: f32,
    
    velocity: Vector2,
    position: Vector2,
    supports_3d: bool,
    renderer_name: String,
}

impl HelloGame {
    pub fn new(smoke_test: bool) -> Self {
        let game = Self {
            graphics: GraphicsDeviceManager::new(&DummyGame),
            sprite_batch: None,
            logo: None,
            solid: None,
            cube_effect: None,
            smoke_test,
            drawn_frames: 0,
            animation_seconds: 0.0,
            renderer_banner_seconds: 5.0,
            velocity: Vector2::new(104.0, 74.0),
            position: Vector2::Zero(),
            supports_3d: true, // Assume 3D for now
            renderer_name: "CNA (Rust)".to_string(),
        };
        game
    }

    fn draw_2d_logo(&self) {
        if let (Some(batch), Some(logo)) = (&self.sprite_batch, &self.logo) {
            let motion = self.animation_seconds * 2.0;
            let _scale = 0.96 + 0.12 * motion.sin();
            
            batch.Begin();
            batch.Draw(logo, self.position, Color::WHITE);
            batch.End();
        }
    }

    fn draw_3d_cube(&self) {
        if let Some(effect) = &self.cube_effect {
            let vp = &self.graphics.GraphicsDevice.Viewport;
            let aspect = vp.Width as f32 / vp.Height as f32;
            
            let motion = self.animation_seconds * 2.0;
            let scale = 0.88 + 0.10 * (motion * 0.48).sin();
            
            let mut world = Matrix::CreateScale(scale);
            world = world * Matrix::CreateRotationY(motion * 0.55);
            world = world * Matrix::CreateRotationX(motion * 0.35);
            
            // In a real implementation, we would set these and call apply
            // effect.World = world;
            // effect.View = Matrix::CreateLookAt(Vector3::new(0.0, 0.0, 6.0), Vector3::Zero(), Vector3::Up());
            // effect.Projection = Matrix::CreatePerspectiveFieldOfView(0.7853982, aspect, 0.1, 100.0);
            effect.Apply();
            
            // Draw primitives here...
        }
    }

    fn draw_renderer_banner(&self) {
        if let (Some(batch), Some(_solid)) = (&self.sprite_batch, &self.solid) {
            let vp = &self.graphics.GraphicsDevice.Viewport;
            let name = self.renderer_name.to_uppercase();
            
            let glyph_cols = (name.len() as i32 * 6) - 1;
            let pixel_size = ((vp.Width - 48) / glyph_cols.max(1)).min(8).max(1);
            
            let text_w = glyph_cols * pixel_size;
            let text_h = 7 * pixel_size;
            let text_x = (vp.Width - text_w) / 2;
            let text_y = vp.Height - text_h - 24;
            
            batch.Begin();
            // Background
            batch.DrawRect(_solid, [ (text_x - 8) as f32, (text_y - 8) as f32, (text_w + 16) as f32, (text_h + 16) as f32], Color::new(255, 255, 255, 180));
            
            for (i, c) in name.chars().enumerate() {
                let rows = get_glyph_rows(c);
                let char_x = text_x + i as i32 * 6 * pixel_size;
                for row in 0..7 {
                    let row_data = rows[row];
                    for col in 0..5 {
                        if (row_data >> (4 - col)) & 1 == 1 {
                            let rect = [ (char_x + col * pixel_size) as f32, (text_y + row * pixel_size) as f32, pixel_size as f32, pixel_size as f32 ];
                            batch.DrawRect(_solid, rect, Color::BLACK);
                        }
                    }
                }
            }
            batch.End();
        }
    }
}

struct DummyGame;
impl Game for DummyGame {}

impl Game for HelloGame {
    fn Initialize(&mut self) -> Result<()> {
        let vp = &self.graphics.GraphicsDevice.Viewport;
        self.position = Vector2::new(vp.Width as f32 / 2.0, vp.Height as f32 / 2.0);
        println!("cna-rust-template: renderer {}", self.renderer_name);
        Ok(())
    }

    fn LoadContent(&mut self) -> Result<()> {
        let device = &self.graphics.GraphicsDevice;
        self.sprite_batch = Some(SpriteBatch::new(device));
        
        // In real app: self.logo = Some(self.content.load::<Texture2D>("logo")?);
        self.logo = Some(Texture2D { Width: 256, Height: 256 });
        self.solid = Some(Texture2D { Width: 1, Height: 1 });
        
        if self.supports_3d {
            let mut effect = BasicEffect::new(device);
            effect.TextureEnabled = true;
            effect.Texture = self.logo.clone();
            self.cube_effect = Some(effect);
        }
        
        Ok(())
    }

    fn Update(&mut self, time: &GameTime) -> Result<()> {
        let dt = time.elapsed.as_secs_f32();
        self.animation_seconds += dt;
        
        if Keyboard::GetState().IsKeyDown(Keys::Escape) {
            self.Exit()?;
        }
        
        if !self.supports_3d {
            let movement_delta = dt * 2.0;
            self.position.X += self.velocity.X * movement_delta;
            self.position.Y += self.velocity.Y * movement_delta;
            
            let vp = &self.graphics.GraphicsDevice.Viewport;
            let logo_size = 256.0;
            
            let min_x = logo_size / 2.0;
            let min_y = logo_size / 2.0;
            let max_x = vp.Width as f32 - min_x;
            let max_y = vp.Height as f32 - min_y;
            
            if self.position.X < min_x {
                self.position.X = min_x;
                self.velocity.X = self.velocity.X.abs();
            } else if self.position.X > max_x {
                self.position.X = max_x;
                self.velocity.X = -self.velocity.X.abs();
            }
            
            if self.position.Y < min_y {
                self.position.Y = min_y;
                self.velocity.Y = self.velocity.Y.abs();
            } else if self.position.Y > max_y {
                self.position.Y = max_y;
                self.velocity.Y = -self.velocity.Y.abs();
            }
        }
        
        Ok(())
    }

    fn Draw(&mut self, _time: &GameTime) -> Result<()> {
        self.graphics.GraphicsDevice.Clear(Color::CORNFLOWER_BLUE);
        
        if self.supports_3d {
            self.draw_3d_cube();
        } else {
            self.draw_2d_logo();
        }
        
        if self.animation_seconds < self.renderer_banner_seconds {
            self.draw_renderer_banner();
        }
        
        if self.smoke_test {
            self.drawn_frames += 1;
            if self.drawn_frames >= 3 {
                println!("cna-rust-template: smoke test drew {} frames; exiting", self.drawn_frames);
                self.Exit()?;
            }
        }
        
        Ok(())
    }
}

fn get_glyph_rows(c: char) -> [u8; 7] {
    match c {
        'A' => [0x04, 0x0A, 0x11, 0x11, 0x1F, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1C, 0x12, 0x11, 0x11, 0x11, 0x12, 0x1C],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0F],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E],
        'J' => [0x07, 0x02, 0x02, 0x02, 0x02, 0x12, 0x0C],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x11, 0x19, 0x15, 0x13, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x1B, 0x11],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '(' => [0x02, 0x04, 0x08, 0x08, 0x08, 0x04, 0x02],
        ')' => [0x08, 0x04, 0x02, 0x02, 0x02, 0x04, 0x08],
        '-' => [0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04],
        _ => [0x1F; 7],
    }
}
