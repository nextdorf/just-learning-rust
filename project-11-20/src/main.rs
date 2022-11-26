use egui_winit::egui;
use egui_winit::winit::event_loop::EventLoop;
use egui_winit::winit::window::Window;
use egui_winit::winit;
use project_11_20::ui;
use project_11_20::wgpustate::WgpuState;
use project_11_20::util::VisualsColorMap;


fn setup_egui_winit(event_loop: &EventLoop<ui::MyEvent>) -> (Window, egui_winit::State, egui::Context){
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
  style.visuals = VisualsColorMap::with_rgba_to_srgba(Some(style.visuals))
    .map_state()
    .unwrap();
  egui_ctx.set_style(style);

  (window, win_state, egui_ctx)
}

fn main() {
  env_logger::init();
  let event_loop = winit::event_loop::EventLoopBuilder::<ui::MyEvent>::with_user_event().build();

  let (window, mut win_state, egui_ctx) = setup_egui_winit(&event_loop);
  let mut ui_state = ui::UI::default();
  let mut render_state = WgpuState::new(&window, ui::constants::ZOOM_100).unwrap();
  win_state.set_pixels_per_point(ui::constants::ZOOM_100);


  event_loop.run(move |event, window_target, control_flow|
    ui::handle_events(event, window_target, control_flow, &window, &mut win_state, &egui_ctx, &mut render_state, &mut ui_state)
  );


}


