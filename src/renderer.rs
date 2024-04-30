pub trait Renderer {
    fn generate_frame(&mut self) -> String;
    fn update_simulation(&mut self);
}
