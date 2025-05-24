use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create window
    let window = video_subsystem
        .window("Rust Video Player", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    // Create renderer
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let mut running = true;

    // Simple animation variables
    let mut x_pos = 0;

    while running {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    println!("Play/Pause functionality would go here");
                },
                _ => {}
            }
        }

        // Clear screen
        canvas.set_draw_color(Color::RGB(30, 30, 60));
        canvas.clear();

        // Draw something (simulating video)
        canvas.set_draw_color(Color::RGB(255, 100, 100));
        canvas.fill_rect(Rect::new(x_pos, 250, 100, 100))?;

        // Update position for animation
        x_pos = (x_pos + 2) % 700;

        // Present to screen
        canvas.present();
    }

    Ok(())
}