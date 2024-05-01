use ascii_renderer::{
    renderer::Renderer,
    steam_renderer::{SteamRenderer, SteamRendererOptions},
};

use std::{thread::sleep, time::Duration};

fn main() {
    // It looks better without the escape characters
    let cup = "          _________________________
         : _ _ _ _ _ _ _ _ _ _ _ _ :
     ,---:\".\".\".\".\".\".\".\".\".\".\".\".\":
    : ,'\"`::.:.:.:.:.:.:.:.:.:.:.::'
    `.`.  `:-===-===-===-===-===-:'
      `.`-._:                   :
        `-.__`.               ,'
    ,--------`\"`-------------'--------.
     `\"--.__                   __.--\"'
            `\"\"-------------\"\"'";

    let mut renderer = SteamRenderer::new(SteamRendererOptions {
        width: 25,
        height: 8,
        offset: 10,
        rise_speed: 0.1,
        wind: 0.0,
        max_speed: 0.5,
    });

    // Pre-generate frames
    let max_frames = 200;
    let frames_per_particle = 2;

    let mut frames = Vec::<String>::with_capacity(max_frames);

    println!("Generating frames...");

    let mut frames_passed = 0;
    for _ in 0..max_frames {
        // Generate frame
        let frame = renderer.generate_frame();
        frames.push(frame);

        // Spawn new particle
        if frames_passed == frames_per_particle {
            renderer.spawn_particle();
            frames_passed = 0;
        };

        // Update simulation
        renderer.update_simulation();

        frames_passed += 1;
    }

    // Playback all frames at the specified fps
    let fps = 8.0;
    for frame in frames {
        // Clear terminal
        print!("\x1B[2J");

        print!("\x1B[2J{frame}{cup}");

        let duration = Duration::from_secs_f32(1.0 / fps);
        sleep(duration);
    }
}
