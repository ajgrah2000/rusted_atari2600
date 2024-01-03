use sdl2::event;
use sdl2::pixels;
use sdl2::render;
use sdl2::video;

use std::fmt;

pub struct WindowSize {
    pub frame_width: u16,
    pub frame_height: u16,
    pub console_width: u16,
    pub console_height: u16,
    pub fullscreen: bool,
}

impl WindowSize {
    pub fn new(frame_width: u16, frame_height: u16, console_width: u16, console_height: u16, fullscreen: bool) -> Self {
        Self {
            frame_width,
            frame_height,
            console_width,
            console_height,
            fullscreen,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Colour {
    // Simple RGB store and conversion at a per colour level.
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn convert_rgb444(&self, dst: &mut [u8]) {
        // RGB444
        dst[0] = (self.g & 0xF0) | (self.b >> 4);
        dst[1] = self.r >> 4;
    }

    pub fn convert_rgb24(&self, dst: &mut [u8]) {
        dst[0] = self.r;
        dst[1] = self.g;
        dst[2] = self.b;
    }

    pub fn convert_rgb888(&self, dst: &mut [u8]) {
        dst[0] = self.b;
        dst[1] = self.g;
        dst[2] = self.r;
    }
}

impl fmt::Display for Colour {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        write!(dest, "R:{} G:{} B:{}", self.r, self.g, self.b)
    }
}

pub struct SDLUtility {}

impl SDLUtility {
    pub const PIXEL_FORMAT: pixels::PixelFormatEnum = pixels::PixelFormatEnum::RGB888;

    pub fn bytes_per_pixel() -> u16 {
        SDLUtility::PIXEL_FORMAT.byte_size_per_pixel() as u16
    }

    pub fn create_canvas(sdl_context: &mut sdl2::Sdl, name: &str, frame_width: u16, frame_height: u16, fullscreen: bool) -> render::Canvas<video::Window> {
        let video_subsystem = sdl_context.video().unwrap();
        let mut renderer = video_subsystem.window(name, frame_width as u32, frame_height as u32);

        // Just playing with if statement (to toggle full screen)
        let window = if fullscreen { renderer.fullscreen() } else { renderer.position_centered().resizable() };

        window.build().map_err(|e| e.to_string()).unwrap().into_canvas().accelerated().build().map_err(|e| e.to_string()).unwrap()
    }

    pub fn texture_creator(canvas: &render::Canvas<video::Window>) -> render::TextureCreator<video::WindowContext> {
        canvas.texture_creator()
    }

    pub fn create_texture(texture_creator: &render::TextureCreator<video::WindowContext>, pixel_format: pixels::PixelFormatEnum, frame_width: u16, frame_height: u16) -> render::Texture {
        texture_creator.create_texture_streaming(pixel_format, frame_width as u32, frame_height as u32).map_err(|e| e.to_string()).unwrap()
    }

    pub fn handle_events(event: &event::Event, window_size: &mut WindowSize) {
        // Handle window events.
        if let event::Event::Window {
            win_event: event::WindowEvent::Resized(w, h), ..
        } = event
        {
            window_size.frame_width = *w as u16;
            window_size.frame_height = *h as u16;
        }
    }
}
