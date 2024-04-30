use std::collections::HashMap;

use rand::prelude::*;

use crate::{renderer::Renderer, vector2::Vector2};

pub struct SteamRenderer {
    pub options: SteamRendererOptions,

    current_max_id: u32,
    particles: HashMap<u32, SteamParticle>,
    frame_data: Vec<Vec<char>>,
}

pub struct SteamRendererOptions {
    pub width: usize,
    pub height: usize,

    pub rise_speed: f32,
    pub wind: f32,
}

pub struct SteamParticle {
    pub position: Vector2<f32>,
    pub left: bool,
    pub frames_between_flips: u32,
    pub lifespan: u32,

    frames: u32,
    frames_since_last_flip: u32,
}

impl Renderer for SteamRenderer {
    fn generate_frame(&mut self) -> String {
        // Clear previous frame data
        for line in self.frame_data.iter_mut() {
            line.fill(' ');
        }

        // Generate frame data
        for (_id, particle) in self.particles.iter_mut() {
            let col = particle.position.x as i32;
            let row = particle.position.y as i32;

            println!("Current coords: {:?}", particle.position);

            // Skip rendering particle if it is off-screen
            if col < 0 || col >= self.options.width as i32 {
                continue;
            }
            if row < 0 || row >= self.options.height as i32 {
                continue;
            }

            self.frame_data[row as usize][col as usize] = particle.to_ascii();
        }

        // Generate frame string
        let mut frame_string =
            String::with_capacity((self.options.width * self.options.height + 1) as usize);

        for line in self.frame_data.iter().rev() {
            for ch in line {
                frame_string.push(*ch);
            }
            frame_string.push('\n');
        }

        frame_string
    }

    fn update_simulation(&mut self) {
        let mut particles_to_delete = vec![];

        // Update positions
        for (_id, particle) in self.particles.iter_mut() {
            particle.position.x += self.options.wind;
            particle.position.y += self.options.rise_speed;

            let delete_particle = particle.update();

            if delete_particle{
                particles_to_delete.push(particle);
            }
        }

        // TODO: Delete particles
    }
}

impl SteamRenderer {
    pub fn new(options: SteamRendererOptions) -> Self {
        Self {
            current_max_id: 0,
            particles: HashMap::new(),
            frame_data: vec![vec![' '; options.width]; options.height],
            options,
        }
    }

    pub fn spawn_particle(&mut self) {
        let id = self.current_max_id;
        self.current_max_id += 1;

        let mut rng = thread_rng();
        let left = rng.gen_bool(0.5);
        let start_x = rng.gen::<f32>() * self.options.width as f32;
        let frames_between_flips = rng.gen_range(0..4);
        let lifespan = rng.gen_range(30..80);

        self.particles.insert(id, SteamParticle {
            position: Vector2::new(start_x, 0.0),

            left,
            frames_between_flips,
            lifespan,

            frames_since_last_flip: 0,
            frames: 0,
        });
    }
}

impl SteamParticle {
    /**
    Updates the particle's internal properties.
    This function is meant to be called on every frame.

     Returns a boolean indicating whether the particle should be deleted or not.
    */
    fn update(&mut self) -> bool {
        self.frames_since_last_flip += 1;
        self.frames += 1;

        if self.frames >= self.lifespan {
            return true;
        }

        if self.frames_since_last_flip > self.frames_between_flips {
            self.flip();
            self.frames_since_last_flip = 0;
        }

        false
    }

    fn to_ascii(&self) -> char {
        match self.position.y as i32 {
            0 => '|',
            _ => {
                if self.left {
                    '{'
                } else {
                    '}'
                }
            }
        }
    }

    fn flip(&mut self) {
        self.left = !self.left;
    }
}
