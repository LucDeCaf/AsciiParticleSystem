use std::{thread::sleep, time::Duration};

use rand::prelude::*;

struct Particle {
    x: f32,
    y: f32,
    influence: f32,
    velocity: (f32, f32),
}

struct ParticleManager {
    width: i32,
    height: i32,
    wind: f32,

    particles: Vec<Particle>,
    frames: Vec<String>,
}

impl ParticleManager {
    fn new(width: i32, height: i32, wind: f32) -> Self {
        Self {
            width,
            height,
            wind,
            particles: vec![],
            frames: vec![],
        }
    }

    fn as_ascii(&self, weight: f32) -> char {
        let weight = weight.floor() as i32;

        let ascii = match weight {
            0..=2 => ' ',
            3..=4 => '.',
            5..=6 => '*',
            _ => '#',
        };

        ascii
    }

    fn add_particle(&mut self, x: f32) {
        if x > self.width as f32 {
            panic!("Particle.x cannot be greater than ParticleManager.width");
        }

        if x < 0.0 {
            panic!("Particle.x cannot be less than zero");
        }

        self.particles.push(Particle {
            x,
            y: 0.0,
            influence: 1.0,
            velocity: (0.0, 0.0),
        })
    }

    fn process_frame(&mut self) {
        let mut frame_data = Vec::with_capacity(self.height as usize);

        for _ in 0..self.height {
            let mut line = Vec::with_capacity(self.width as usize);
            line.fill(0.0);

            frame_data.push(line);
        }

        let mut x;
        let mut y;

        for particle in self.particles.iter_mut() {
            // Apply velocity
            particle.x += particle.velocity.0.clamp(0.0, self.width as f32 - 1.0);
            particle.y += particle.velocity.1.clamp(0.0, self.height as f32 - 1.0);

            // Update data in closest cell
            x = particle.x.round() as usize;
            y = particle.y.round() as usize;
            frame_data[y][x] += particle.influence / 10.0;
        }

        // Generate frame as a single string
        let mut frame_string = String::new();

        for line in frame_data {
            for weight in line {
                frame_string.push(self.as_ascii(weight))
            }
        }

        self.frames.push(frame_string);
    }
}

fn main() {
    let width = 16;
    let height = 6;
    let wind = 0.0;

    let mut manager = ParticleManager::new(width, height, wind);

    manager.add_particle(0.0);

    let mut counter = 0;

    let max_frames = 1;
    for i in 0..max_frames {
        manager.process_frame();

        counter += 1;

        if counter == 10 {
            let offset = thread_rng().gen::<f32>();
            manager.add_particle(offset * width as f32);
            counter = 0;
        }
    }

    let fps = 4.0;
    for frame in manager.frames {
        // Clear terminal
        print!("\x1B[2J");

        println!("{}", frame);

        // Wait a little bit to make it watchable
        sleep(Duration::from_secs_f32(1.0 / fps));
    }
}
