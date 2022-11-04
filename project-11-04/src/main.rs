use winit::{self, event_loop::{EventLoop, ControlFlow}, window::Window, event::{Event, WindowEvent}};

fn main() {
  env_logger::init();
  let event_loop = EventLoop::new();
  let window = Window::new(&event_loop).unwrap();
  window.set_title("ඞඞඞඞඞ");

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;
    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, ..} =>
        *control_flow = ControlFlow::Exit,
      _ => ()
    }
  });

}
