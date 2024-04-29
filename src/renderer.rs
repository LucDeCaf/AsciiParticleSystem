use crate::vector2::Vector2;

pub struct Particle {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub influence: f32,
}

pub struct Renderer {
    pub options: RendererOptions,
    pub particles: Vec<Particle>,
    pub frames: Vec<String>,

    // For internal computation
    frame_data: Vec<Vec<f32>>,
}

pub struct RendererOptions {
    pub width: i32,
    pub height: i32,
}

impl Renderer {
    pub fn new(options: RendererOptions) -> Self {
        if options.width < 0 || options.height < 0 {
            panic!("Renderer.width and Renderer.height must be greater than zero");
        }

        let width = options.width;
        let height = options.height;

        Self {
            options,
            particles: vec![],
            frames: vec![],
            frame_data: vec![vec![0.0; width as usize]; height as usize],
        }
    }

    pub fn as_ascii(&self, weight: f32) -> char {
        let weight = weight.floor() as i32;

        let ascii = match weight {
            0 => ' ',
            1..=2 => '.',
            3..=4 => '*',
            _ => '#',
        };

        ascii
    }

    pub fn add_default_particle(&mut self, x: f32, y: f32) -> &mut Particle {
        // Prevent placing particles outside the bounding box of the simulation
        if x < 0.0 || x > self.options.width as f32 || y < 0.0 || y > self.options.height as f32 {
            panic!("particle.position must be within bounding box");
        }

        self.particles.push(Particle {
            position: Vector2::new(x, y),
            velocity: Vector2::new(0.0, 0.0),
            influence: 1.0,
        });

        self.particles.last_mut().unwrap()
    }

    pub fn process_frame(&mut self) {
        // Reset frame_data
        for line in self.frame_data.iter_mut() {
            line.fill(0.0);
        }

        let mut x;
        let mut y;

        for particle in self.particles.iter_mut() {
            // Apply velocity
            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;

            // Clamp positions inside of bounding box
            particle.position.x = particle
                .position
                .x
                .clamp(0.0, self.options.width as f32 - 0.01);
            particle.position.y = particle
                .position
                .y
                .clamp(0.0, self.options.height as f32 - 0.01);

            // Update data in closest cell
            x = particle.position.x.floor() as usize;
            y = particle.position.y.floor() as usize;

            self.frame_data[y][x] += particle.influence;
        }

        // Generate frame as a single string
        let mut frame_string = String::new();

        for line in self.frame_data.iter() {
            for weight in line {
                frame_string.push(self.as_ascii(*weight))
            }
            frame_string.push('\n');
        }

        self.frames.push(frame_string);
    }
}
