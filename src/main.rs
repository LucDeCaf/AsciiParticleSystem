// TODO: Stop allocating new array for frame_data on every frame
// TODO: Add more parameters for controlling simulation
// TODO: Test on larger terminal and slow down simulation speed

mod vector2;
use vector2::Vector2;

use std::{thread::sleep, time::Duration, vec};

use rand::prelude::*;

struct Particle {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    influence: f32,
}

struct Renderer {
    viewport_width: i32,
    viewport_height: i32,
    particles: Vec<Particle>,
    frames: Vec<String>,
}

impl Renderer {
    fn new(width: i32, height: i32) -> Self {
        Self {
            viewport_width: width,
            viewport_height: height,
            particles: vec![],
            frames: vec![],
        }
    }

    fn as_ascii(&self, weight: f32) -> char {
        let weight = weight.floor() as i32;

        let ascii = match weight {
            0 => ' ',
            1..=2 => '.',
            3..=4 => '*',
            _ => '#',
        };

        ascii
    }

    fn add_particle(&mut self, x: f32, y: f32) -> &mut Particle {
        // Prevent placing particles outside the bounding box of the simulation
        if x > self.viewport_width as f32 || x < 0.0 {
            panic!("particle.position.x must be within bounding box");
        }
        if y < 0.0 || y > self.viewport_height as f32 {
            panic!("particle.position.y must be within bounding box");
        }

        self.particles.push(Particle {
            position: Vector2::new(x, y),
            velocity: Vector2::new(0.0, 0.0),
            influence: 1.0,
        });

        self.particles.last_mut().unwrap()
    }

    fn process_frame(&mut self) {
        // Create 2D array to store frame data
        let mut frame_data = Vec::with_capacity(self.viewport_height as usize);

        for _ in 0..self.viewport_height {
            let line = vec![0.0; self.viewport_width as usize];
            frame_data.push(line);
        }

        let mut x;
        let mut y;

        for particle in self.particles.iter_mut() {
            // Apply velocity
            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;

            // Clamp positions inside of bounding box
            particle.position.x = particle.position.x.clamp(0.0, self.viewport_width as f32 - 0.01);
            particle.position.y = particle.position.y.clamp(0.0, self.viewport_height as f32 - 0.01);
            
            // Update data in closest cell
            x = particle.position.x.floor() as usize;
            y = particle.position.y.floor() as usize;

            frame_data[y][x] += particle.influence;
        }

        // Generate frame as a single string
        let mut frame_string = String::new();

        for line in frame_data {
            for weight in line {
                frame_string.push(self.as_ascii(weight))
            }
            frame_string.push('\n');
        }


        self.frames.push(frame_string);
    }
}

fn main() {
    let width = 16;
    let height = 6;

    let mut manager = Renderer::new(width, height);

    let max_frames = 100;
    let frames_per_particle = 1;

    let mut frames_passed = 0;

    for _ in 0..max_frames {
        if frames_passed == frames_per_particle {
            let x_offset = thread_rng().gen::<f32>();
            let y_offset = thread_rng().gen::<f32>();
            manager.add_particle(x_offset * manager.viewport_width as f32, y_offset * manager.viewport_height as f32);
            frames_passed = 0;
        };

        manager.process_frame();

        frames_passed += 1;
    }

    let fps = 24.0;
    for frame in manager.frames {
        // Clear terminal
        print!("\x1B[2J");

        println!("{}", frame);

        // Wait a little bit to make it watchable
        let duration = Duration::from_secs_f32(1.0 / fps);
        sleep(duration);
    }
}
