// TODO: Add more parameters for controlling simulation
// TODO: Test on larger terminal and slow down simulation speed

mod renderer;
mod vector2;
use renderer::{Renderer, RendererOptions};

use std::{thread::sleep, time::Duration};

use rand::prelude::*;

fn main() {
    let mut manager = Renderer::new(RendererOptions {
        width: 32,
        height: 10,
    });

    let max_frames = 200;
    let frames_per_particle = 1;

    let mut frames_passed = 0;

    for _ in 0..max_frames {
        if frames_passed == frames_per_particle {
            let x_offset = thread_rng().gen::<f32>();
            let y_offset = thread_rng().gen::<f32>();

            manager.add_default_particle(
                x_offset * manager.options.width as f32,
                y_offset * manager.options.height as f32,
            );
            frames_passed = 0;
        };

        manager.process_frame();

        frames_passed += 1;
    }

    // Playback all frames
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
