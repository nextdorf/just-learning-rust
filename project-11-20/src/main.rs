mod wgpustate;
mod util;
use egui_winit::egui;
use egui_winit::winit::dpi::PhysicalSize;
use egui_winit::winit::event_loop::EventLoop;
use egui_winit::winit::window::Window;
use egui_winit::winit::{self, event_loop::ControlFlow, event::Event, event};
use wgpustate::WgpuState;

enum MyEvent {
  RequestRedraw,
  Rescale(f32),
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

  let mut style = (*egui_ctx.style()).clone();
  style.visuals = util::VisualsColorMap::with_rgba_to_srgba(Some(style.visuals))
    .map_state()
    .unwrap();
  egui_ctx.set_style(style);

  (window, win_state, egui_ctx)
}

fn main() {
  env_logger::init();
  let event_loop = winit::event_loop::EventLoopBuilder::<MyEvent>::with_user_event().build();

  let (window, mut win_state, egui_ctx) = setup_egui_winit(&event_loop);

  let mut render_state = WgpuState::new(&window, 1.5).unwrap();

  let mut test_var = 0;
  let mut img_hnd = None;
  let mut ctrl_modifier = false;
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
            // event::WindowEvent::ModifiersChanged(changed_mod) if changed_mod.contains(event::ModifiersState::CTRL) =>
            //   ctrl_modifier = !ctrl_modifier,
            event::WindowEvent::Resized(PhysicalSize { width, height}) =>
              render_state.resize(Some(width), Some(height), None),
            event::WindowEvent::CloseRequested | event::WindowEvent::Destroyed => 
              *control_flow = ControlFlow::Exit,
            event::WindowEvent::KeyboardInput { input, .. } => match input {
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Escape), state: event::ElementState::Pressed,.. } =>
                *control_flow = ControlFlow::Exit,
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::LControl), state,.. } =>
                ctrl_modifier = state == event::ElementState::Pressed,
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Up), state: event::ElementState::Pressed,.. } if ctrl_modifier =>
                // render_state.resize(None, None, Some(render_state.get_surface_scale()*1.2)),
                egui_ctx.set_pixels_per_point(render_state.get_surface_scale()*1.2),
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Down), state: event::ElementState::Pressed,.. } if ctrl_modifier =>
                // egui_ctx.set_pixels_per_point(render_state.get_surface_scale()/1.2),
                {
                  //TODO: use mutable
                  let q = egui_ctx.input();
                  q.events.push(egui::Event::Zoom(render_state.get_surface_scale()/1.2));

                },
              _ => {}
              },
            event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => 
              render_state.resize(Some(new_inner_size.width), Some(new_inner_size.height), Some(scale_factor as _)),
            // winit::event::WindowEvent::ThemeChanged(_) => todo!(),
            _ => {}
          }
        }
      },
      Event::RedrawRequested(window_id) if window_id != window.id() => { },
      Event::RedrawRequested(..) | Event::UserEvent(MyEvent::RequestRedraw) => {
        let _did_render = render_state.redraw(|| {
          let raw_input = win_state.take_egui_input(&window);
          let full_output = egui_ctx.run(raw_input, |ctx| {
            let tex = img_hnd.get_or_insert_with(|| {
              ctx.load_texture("texture", egui::ColorImage::example(), egui::TextureFilter::Linear)
            });

            egui::CentralPanel::default().show(ctx, |ui| {
              ui.label("text text text text text text text text text text text text text text text text text text text text ");
              ui.separator();
              if ui.button(test_var.to_string()).clicked() {
                test_var += 1;
              }
              ui.separator();
              ui.image(tex.id(), tex.size_vec2());
            });
          });
          win_state.handle_platform_output(&window, &egui_ctx, full_output.platform_output);
          let paint_jobs = egui_ctx.tessellate(full_output.shapes);
          let texture_delta = full_output.textures_delta;
          
          (texture_delta, paint_jobs)
        }).and(Some(true)).unwrap_or(false);
      },
      Event::MainEventsCleared => window.request_redraw(),
      _ => {}
    }
  });

}


