extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::cmp::min;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Falling blocks please", 800, 600)
        .fullscreen_desktop()
        .position_centered()
        .build()
        .unwrap();
    let screen_size = window.size();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    const PLAYING_SIZE: (u32, u32) = (10, 30);
    let mut blocks: [u8; (PLAYING_SIZE.0 * PLAYING_SIZE.1) as usize] = [0; (PLAYING_SIZE.0 * PLAYING_SIZE.1) as usize];
    blocks[0] = 1;
    blocks[1] = 2;
    blocks[25] = 1;

    //Calculate how big we can draw the given playing field
    let x_max_side = screen_size.0 / PLAYING_SIZE.0;
    let y_max_side = screen_size.1 / (PLAYING_SIZE.1 + 1); //add one so we have space for the blocks yet to be dropped
    let side_len = min(x_max_side, y_max_side);
    //and an offset to start our playing field at
    let playing_field_offset = ((screen_size.0 - (side_len * PLAYING_SIZE.0)) / 2, screen_size.1 - (side_len * PLAYING_SIZE.1));

    let mut cassette_offset = 0;
    let mut cassette: [u8; PLAYING_SIZE.0 as usize] = [0; PLAYING_SIZE.0 as usize];
    cassette[0] = 1;

    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },

                //Arrows
                Event::KeyDown { keycode: Some(Keycode::Left), .. } |
                Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    cassette_offset = (cassette_offset + 1) % PLAYING_SIZE.0
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } |
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    cassette_offset = (cassette_offset + PLAYING_SIZE.0 - 1) % PLAYING_SIZE.0
                },
                //Drop
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    //you can only drop a block if the top row has space
                    for x in 0..PLAYING_SIZE.0 {
                        if cassette[((x + cassette_offset) % PLAYING_SIZE.0) as usize] != 0 && blocks[(x % PLAYING_SIZE.0) as usize] == 0 {
                            blocks[(x % PLAYING_SIZE.0) as usize] = cassette[((x + cassette_offset) % PLAYING_SIZE.0) as usize];
                            cassette[((x + cassette_offset) % PLAYING_SIZE.0) as usize] = 0;
                        }
                    }
                },

                _ => {}
            }
        }

        //Background
        canvas.set_draw_color(Color::RGB(169, 169, 169));
        canvas.fill_rect(sdl2::rect::Rect::new(playing_field_offset.0 as i32, 0, PLAYING_SIZE.0 * side_len, screen_size.1)).unwrap();

        //Cassette
        canvas.set_draw_color(Color::RGB(0, 169, 0));
        for x in 0..PLAYING_SIZE.0 {
            let colour_type = cassette[((x + cassette_offset) % PLAYING_SIZE.0) as usize];
            if colour_type != 0 {
            canvas.fill_rect(sdl2::rect::Rect::new(playing_field_offset.0 as i32 + (x as i32) * side_len as i32,
                                                                playing_field_offset.1 as i32 - side_len as i32,
                                                                side_len - 1, side_len - 1)).unwrap();
            }
        }

        //Blocks
        canvas.set_draw_color(Color::RGB(169, 0, 0));
        for y_index in 0..PLAYING_SIZE.1 {
            for x in 0..PLAYING_SIZE.0 {
                let mut y = PLAYING_SIZE.1 - y_index - 1; //go from the bottom up, so blocks only fall one block each game tick
                let colour_type = blocks[(x + y * PLAYING_SIZE.0) as usize];
                if colour_type != 0 {
                    //make them fall
                    if (i % 15 == 0) && (y != PLAYING_SIZE.1 - 1) && blocks[(x + (y + 1) * PLAYING_SIZE.0) as usize] == 0 {
                        blocks[(x + (y + 1) * PLAYING_SIZE.0) as usize] = colour_type;
                        blocks[(x + y * PLAYING_SIZE.0) as usize] = 0;
                        y += 1; //we just moved it down so render it on the layer below
                    }
                    //render it
                    canvas.fill_rect(sdl2::rect::Rect::new(playing_field_offset.0 as i32 + (x as i32) * side_len as i32,
                                                            playing_field_offset.1 as i32 + (y as i32) * side_len as i32,
                                                            side_len - 1, side_len - 1)).unwrap();
                }
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1000000000u32/60));
    }
}