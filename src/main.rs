// TODO: Add more parameters for controlling simulation
// TODO: Test on larger terminal and slow down simulation speed

use ascii_renderer::{renderer::Renderer, steam_renderer::{SteamRenderer, SteamRendererOptions}};

use std::{thread::sleep, time::Duration};

fn main() {
    let mut renderer = SteamRenderer::new(SteamRendererOptions {
        width: 40,
        height: 10,
        rise_speed: 0.1,
        wind: 0.0,
    });

    let max_frames = 200;
    let frames_per_particle = 4;

    let mut frames = Vec::<String>::with_capacity(max_frames);

    let mut frames_passed = 0;
    for _ in 0..max_frames {
        // Generate frame
        let frame = renderer.generate_frame();
        frames.push(frame);

        // Spawn new particles
        if frames_passed == frames_per_particle {
            renderer.spawn_particle();
            frames_passed = 0;
        };

        // Update simulation
        renderer.update_simulation();

        frames_passed += 1;
    }

    // Playback all frames at the specified fps
    let fps = 4.0;
    for frame in frames {
        // Clear terminal
        print!("\x1B[2J");

        print!("{}", frame);

        let duration = Duration::from_secs_f32(1.0 / fps);
        sleep(duration);
    }
}
