mod wgpustate;
use egui_winit::egui;
use egui_winit::winit::dpi::PhysicalSize;
use egui_winit::winit::event_loop::EventLoop;
use egui_winit::winit::window::Window;
use egui_winit::winit::{self, event_loop::ControlFlow, event::Event, event};
use wgpustate::WgpuState;

enum MyEvent {
  RequestRedraw,
}

fn setup_egui_winit(event_loop: &EventLoop<MyEvent>) -> (Window, egui_winit::State, egui::Context){
  let window = winit::window::WindowBuilder::new()
    .with_decorations(false)
    .with_resizable(true)
    .with_transparent(true)
    .with_title("not eframe")
    .with_inner_size(winit::dpi::PhysicalSize {
      width: 512,
      height: 512,
    })
    .build(event_loop)
    .unwrap();

  let win_state = egui_winit::State::new(event_loop);
  let egui_ctx = egui::Context::default();

  (window, win_state, egui_ctx)
}

fn main() {
  env_logger::init();
  let event_loop = winit::event_loop::EventLoopBuilder::<MyEvent>::with_user_event().build();

  let (window, mut win_state, egui_ctx) = setup_egui_winit(&event_loop);

  let mut render_state = WgpuState::new(&window, 2.).unwrap();

  let mut test_var = 0;
  // egui_ctx.set_fonts(egui::FontDefinitions::default());
  egui_ctx.set_pixels_per_point(render_state.get_surface_scale());

  event_loop.run(move |event, _window_target, control_flow| {
    *control_flow = ControlFlow::Wait;
    
    match event {
      Event::WindowEvent { window_id, event } if window_id==window.id() => {
        // if let event::WindowEvent::MouseInput { state, button, .. } = event {
        //   eprintln!("Clicked: {:?} - {:?}", state, button)
        // }
        if !win_state.on_event(&egui_ctx, &event) {
          match event {
            event::WindowEvent::Resized(PhysicalSize { width, height}) =>
              render_state.resize(Some(width), Some(height), None),
            event::WindowEvent::CloseRequested | event::WindowEvent::Destroyed => 
              *control_flow = ControlFlow::Exit,
            event::WindowEvent::KeyboardInput { input, .. } => match input {
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Escape), state: event::ElementState::Pressed,.. } =>
              *control_flow = ControlFlow::Exit,
              _ => {}
              },
            event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => 
              render_state.resize(Some(new_inner_size.width), Some(new_inner_size.height), Some(scale_factor as _)),
            // winit::event::WindowEvent::ThemeChanged(_) => todo!(),
            _ => {}
          }
        }
      },
      Event::RedrawRequested(window_id) if window_id==window.id() => {
      // Event::RedrawRequested(..) | Event::UserEvent(MyEvent::RequestRedraw) => {
        let _did_render = render_state.redraw(|| {
          let raw_input = win_state.take_egui_input(&window);
          raw_input.pixels_per_point;
          let full_output = egui_ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
              ui.label("text text text text text text text text text text text text text text text text text text text text ");
              ui.separator();
              if ui.button(test_var.to_string()).clicked() {
                test_var += 1;
              }
            });
          });
          win_state.handle_platform_output(&window, &egui_ctx, full_output.platform_output);
          let paint_jobs = egui_ctx.tessellate(full_output.shapes);
          
          // let screen_discriptor = egui_wgpu::renderer::ScreenDescriptor {
          //   size_in_pixels: [surface_config.width, surface_config.height],
          //   pixels_per_point: window.scale_factor() as _,
          // };
          let texture_delta = full_output.textures_delta;
          
          // egui_rpass.update_texture(&device, &queue, id, texture_delta)
          (texture_delta, paint_jobs)
        }).and(Some(true)).unwrap_or(false);
      },
      Event::MainEventsCleared => window.request_redraw(),
      _ => {}
    }
  });

}


