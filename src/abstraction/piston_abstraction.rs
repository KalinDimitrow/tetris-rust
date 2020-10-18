use crate::abstraction::abstraction_layer::*;
use crate::states::state_machine::StateMachine;
use crate::resources::*;
use piston_window::*;
use std::path::PathBuf;
use std::error;

const GAME_NAME: &str = "Tetris";
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1280;

const ASSET_DIRECTORY : &str = "assets";
const GAME_FONT : &str = "TetrisFont2.ttf";
const PARENT_DEPTH : find_folder::ParentsDepth = 3;
const KIDS_DEPTH : find_folder::KidsDepth = 3;

type TextureHandle<T> = crate::abstraction::abstraction_layer::TextureHandle<T>;
pub struct PistonRenderContext<'a, 'g2d, 'args, 'device> {
    c: Context,
    g: &'g2d mut G2d<'a>,
    arguments: &'args RenderArgs,
    device: &'device gfx_device_gl::Device,
}

impl<'a, 'g2d, 'args, 'device> PistonRenderContext<'a, 'g2d, 'args, 'device> {
    fn new(
        c: Context,
        g: &'g2d mut G2d<'a>,
        arguments: &'args RenderArgs,
        device: &'device mut gfx_device_gl::Device,
    ) -> PistonRenderContext<'a, 'g2d, 'args, 'device> {
        PistonRenderContext{c, g, arguments, device }
    }
}

impl<'a, 'g2d, 'args, 'device> RenderContext for PistonRenderContext<'a, 'g2d, 'args, 'device> {
    type TextureHandle = G2dTexture;
    type FontHandle = i32;
    fn draw_image<T>(&mut self, handle : &TextureHandle<T>, transform : &Transform) where T : RenderContext {
        image(&handle.handle, self.c.transform.trans(transform.x, transform.y), self.g);
    }

    fn draw_text<T>(&self, text : &str, font : &Font<T>, transform : &Transform, color : &Color) where T : RenderContext {
        // text::Text::new_color([1.0, 1.0, 0.0, 1.0], 32)
        //     .draw(
        //         text,
        //         &mut resources.font,
        //         &c.draw_state,
        //         c.transform
        //             .trans(SCORE_TEXT_POSITION_X as f64, SCORE_TEXT_POSITION_Y as f64),
        //         g,
        //     )
        //     .unwrap();
    }

    fn font(&self, path : &str) -> Self::FontHandle {
        0
    }
}

pub struct PistonAbstraction {
    window: PistonWindow,
    assets: PathBuf,
    // resource_loader : ResourceLoader<PistonAbstraction::Ctx>
    // resource_loader : ResourceLoader<abstraction::piston_abstraction::PistonAbstraction as Trait>::Ctx
}

impl PistonAbstraction {
    pub fn new() -> Result<PistonAbstraction, Box<dyn error::Error>> {
        let assets = find_folder::Search::ParentsThenKids(PARENT_DEPTH, KIDS_DEPTH)
            .for_folder(ASSET_DIRECTORY)?;

        let mut window = WindowSettings::new(GAME_NAME, [WINDOW_WIDTH, WINDOW_HEIGHT])
            .exit_on_esc(false)
            .graphics_api(OpenGL::V3_2)
            .build()?;
        Ok(PistonAbstraction {
            window,
            assets,
        })
    }

    fn loop_handler(&mut self, game : &StateMachine, resources : &Resources, loop_arg: Loop, event: Event) -> bool {
        match loop_arg {
            Loop::Update(update_args) => {
                // if !self.update(update_args, event.clone()) {
                //     return true;
                // }
            }

            Loop::Render(render_args) => {
                self.window.draw_2d(&event, |c, g, device| {
                    clear([1.0; 4], g);
                    let mut ctx = PistonRenderContext::new(c, g, &render_args, device);
                    game.render(&mut ctx, resources);
                });
            }

            _ => {}
        }

        false
    }

    fn input_handler(&mut self, game : &StateMachine, input: Input, time: Option<TimeStamp>) {
        // self.logic.handle_input(input, time, &mut self.data);
    }
}

struct PistonResourceLoader<'a> {
    window: &'a mut PistonWindow
}

impl PistonResourceLoader<'_> {
    fn new(piston_window : &mut PistonWindow) -> PistonResourceLoader {
        PistonResourceLoader{window : piston_window}
    }
}

impl<'a> ResourceLoader<<crate::abstraction::piston_abstraction::PistonAbstraction as crate::abstraction::abstraction_layer::AbstractionLayer>::Ctx> for PistonResourceLoader<'a> {
    // fn texture(&self, path : &str) -> TextureHandle<T> {
    //
    // }
}

impl AbstractionLayer for PistonAbstraction {
    type Ctx = PistonRenderContext<'static, 'static, 'static, 'static>;
    fn run(&mut self, game : &StateMachine, resources : &Resources) {
        while let Some(event) = self.window.next() {
            match event {
                Event::Loop(_loop) => {
                    if self.loop_handler(game, resources, _loop, event) {
                        return;
                    }
                }

                Event::Input(args, time) => {
                    self.input_handler(game, args, time);
                }

                _ => {}
            }

            // if game.data.running == false {
            //     break;
            // }
        }
    }

    fn load_texture(&mut self, path : &str) ->TextureHandle<Self::Ctx> {
        let handle = Texture::from_path(
            &mut self.window.create_texture_context(),
            self.assets.join(String::from(path)),
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

        TextureHandle::<Self::Ctx> {handle}
    }

    // fn resource_loader(&self) -> &ResourceLoader<Self::Ctx> {
    //     &self.resource_loader
    // }
}