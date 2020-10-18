use std::ops::FnOnce;
use crate::states::state_machine::StateMachine;
use crate::resources::*;

pub struct Transform {
    pub x : f64,
    pub y : f64,
}

pub struct Color {

}

pub struct TextureHandle<T> where T : RenderContext {
    pub handle : T::TextureHandle,
}

// impl<T> TextureHandle<T> where T : RenderContext {
//     pub fn load(path : &str, ctx : &T) -> Self {
//         Self{ handle : ctx.texture(path)}
//     }
// }

pub struct Font<T> where T : RenderContext {
    handle : T::FontHandle,
}

// impl<T> Font<T> where T : RenderContext {
//     pub fn load(path : &str, ctx : &T) -> Self {
//         Self{ handle : ctx.font(path)}
//     }
// }

pub trait ResourceLoader<T> where T : RenderContext {
    // fn texture(&self, path : &str) -> TextureHandle<T>;
}

pub trait RenderContext {
    type TextureHandle;
    type FontHandle;
    fn draw_image<T>(&mut self, image : &TextureHandle<T>, transform : &Transform) where T : RenderContext;
    fn draw_text<T>(&self, text : &str, font : &Font<T>, transform : &Transform, color : &Color) where T : RenderContext;
    fn font(&self, path : &str) -> Self::FontHandle;
}

pub trait AbstractionLayer {
    type Ctx : RenderContext;
    fn run(&mut self, game : &StateMachine, resources : &Resources);
    fn load_texture(&mut self, path : &str) ->TextureHandle<Self::Ctx>;
    // fn resource_loader(&self) -> &ResourceLoader<Self::Ctx>;
}

// struct Gosho {
//     a : Texture<Gosho>
// }

// impl Gosho {
//     fn new() -> Gosho {
//         Gosho {a : Texture::<Self> {handle : 5}}
//     }
// }
//
// impl RenderContext for Gosho {
//     type TextureHandle = usize;
//     fn print(&self) {
//         println!("Gosho handle {}", self.a.handle);
//     }
// }

// trait Something {
//     type TextureHandle;
// }
//
// struct Pesho {
//
// }
//
// impl Something for Pesho {
//     type TextureHandle = i32;
// }



// struct ImputState {
//
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn generics_test() {
//
//     }
// }