extern crate conrod;

fn main() {
    game_of_life::start();
}

mod game_of_life {
    use conrod::backend::glium::glium;
    use std;

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;


    pub fn start() {
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Game of Life")
            .with_dimensions((WIDTH, HEIGHT).into());

        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true);

        let mut events_loop = glium::glutin::EventsLoop::new();

        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let sixteen_ms = std::time::Duration::from_millis(16);

        loop {
            let loop_start = std::time::Instant::now();

            let mut events = Vec::new();
            events_loop.poll_events(|event| events.push(event));
            
            for event in events {
                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => return,
                        e => (), //println!("{:?}", e),
                    },
                    e => (), // println!("{:?}", e),
                }
            }

            let elapsed = std::time::Instant::now() - loop_start;
            if elapsed > sixteen_ms {
                std::thread::sleep(elapsed);
            }
        }
    }
}