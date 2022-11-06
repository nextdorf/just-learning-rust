use project_11_04::WinState;
use winit::{self, event_loop::{EventLoop, ControlFlow}, window::Window, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

fn main() { 
  pollster::block_on(run());
}

async fn run() {
  env_logger::init();
  let event_loop = EventLoop::new();
  let window = Window::new(&event_loop).unwrap();
  window.set_title("ඞඞඞඞඞ");

  let all_adapters = wgpu::Instance::new(wgpu::Backends::all())
    .enumerate_adapters(wgpu::Backends::all());
  for a in all_adapters {
    eprintln!("- {:?}", a.get_info())
  }
  eprintln!("-----");

  let mut state = WinState::new(&window).await;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;
    match event {
      Event::WindowEvent { window_id, ref event } if 
        window_id == window.id() => if !state.input(event) {
          match event {
              WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input: KeyboardInput {
                  state: ElementState::Pressed,
                  virtual_keycode: Some(VirtualKeyCode::Escape),
                  ..
                }, 
                ..
              } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => state.resize(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => state.resize(**new_inner_size),
            _ => ()
          }
      },
      Event::RedrawRequested(window_id) if window_id == window.id() => {
        state.update();
        match state.render() {
          Ok(_) => (),
          Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
          Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
          Err(e) => eprintln!("{:?}", e),
        }
      },
      Event::MainEventsCleared => window.request_redraw(),
      _ => ()
    }
  });

}
