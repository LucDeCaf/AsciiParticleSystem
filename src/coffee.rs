use crate::{
    renderer::Renderer,
    steam_renderer::{SteamRenderer, SteamRendererOptions},
};

pub struct Coffee {
    renderer: SteamRenderer,
}

impl Coffee {
    pub fn new(options: SteamRendererOptions) -> Self {
        Self {
            renderer: SteamRenderer::new(options),
        }
    }

    pub fn spawn_particle(&mut self) {
        self.renderer.spawn_particle();
    }
}

impl Renderer for Coffee {
    fn generate_frame(&mut self) -> String {
        const CUP: &str = "          _________________________
         : _ _ _ _ _ _ _ _ _ _ _ _ :
     ,---:\".\".\".\".\".\".\".\".\".\".\".\".\":
    : ,'\"`::.:.:.:.:.:.:.:.:.:.:.::'
    `.`.  `:-===-===-===-===-===-:'
      `.`-._:                   :
        `-.__`.               ,'
    ,--------`\"`-------------'--------.
     `\"--.__                   __.--\"'
            `\"\"-------------\"\"'";

        let mut frame = self.renderer.generate_frame();
        frame.push_str(CUP);

        frame
    }

    fn update_simulation(&mut self) {
        self.renderer.update_simulation()
    }
}
