struct Transform {
    x : f64,
    y : f64,
}

struct Texture {

}

struct Color {

}

trait RenderContext {
    fn draw_image(&self, image : &Texture, transform : &Transform);
    fn draw_text(&self, text : &str, transform : &Transform, color : &Color);
}

struct ImputState {
    
}