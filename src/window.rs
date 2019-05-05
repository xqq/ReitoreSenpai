use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use crate::tracer::trace;

pub fn window_main_loop() {
    let texture_width: u32 = 1920;
    let texture_height: u32 = 1080;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("ReitoreSenpai", texture_width, texture_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::ABGR8888, texture_width, texture_height)
        .unwrap();

    canvas.clear();
    canvas.present();

    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let (threads, duration) = trace(buffer, pitch, texture_width, texture_height);
        let window = canvas.window_mut();
        window.set_title(format!("ReitoreSenpai  threads: {}, elapsed: {}", threads, duration.as_float_secs()).as_ref()).unwrap();
    }).unwrap();

    canvas.clear();
    canvas.copy_ex(&texture, None, Some(Rect::new(0, 0, texture_width, texture_height)),
                   0.0, None, false, false).unwrap();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
    }
}
