use egui_winit::{
  self, 
  winit::{event_loop::{ControlFlow, EventLoopWindowTarget}, event::{self, Event}, window::Window, dpi::PhysicalSize}, 
  State,
  egui,
};
use epaint::TextureHandle;

use crate::wgpustate::WgpuState;

pub enum MyEvent {
  RequestRedraw,
  Rescale(f32),
}



pub struct UIHandler<'a> {
  win_state: &'a mut State,
  window: &'a Window,
  egui_ctx: &'a egui::Context,
  render_state: &'a mut WgpuState,

  ctrl_modifier: bool,

  ui: UI,
}

pub struct UI {
  img_hnd: Option<Vec<TextureHandle>>,
  test_var: usize,
}
impl UI {
  pub fn ui(&mut self, ctx: &egui::Context) {
    let img_hnd = self.img_hnd.get_or_insert_with(|| vec![
      ctx.load_texture("uv_texture",
        (|| {
          let size = [256, 256];
          let mut rgba = Vec::with_capacity(size[0]*size[1]*4);
          for j in 0..size[1] {
            for i in 0..size[0] {
              let r = ((i as f32) / ((size[0]-1) as f32) * 255.).round() as _;
              let g = ((j as f32) / ((size[1]-1) as f32) * 255.).round() as _;
              rgba.push(r);
              rgba.push(g);
              rgba.push(0);
              rgba.push(255);
            }
          }
          
          egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_slice())
        })(),
        egui::TextureFilter::Linear),
      ctx.load_texture("sample_texture",
        egui::ColorImage::example(),
        egui::TextureFilter::Linear),
    ]);
  
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.label("text text text text text text text text text text text text text text text text text text text text ");
      ui.separator();
      if ui.button(self.test_var.to_string()).clicked() {
        self.test_var += 1;
      }
      ui.separator();
      ui.horizontal(|ui| {
        for tex in img_hnd.iter() {
          ui.image(tex.id(), tex.size_vec2());
        }
      });
    });
  }
}



impl<'a> UIHandler<'a> {
  pub fn new(win_state: &'a mut State, window: &'a Window, egui_ctx: &'a egui::Context, render_state: &'a mut WgpuState) -> Self {
    // egui_ctx.set_fonts(egui::FontDefinitions::default());
    win_state.set_pixels_per_point(render_state.get_surface_scale());


    Self { win_state, window, egui_ctx, render_state, ctrl_modifier: false,
      ui: UI { img_hnd: None, test_var: 0 }
    }
  }

  pub fn handle_event(&mut self, event: Event<MyEvent>, window_target: &EventLoopWindowTarget<MyEvent>, control_flow: &mut ControlFlow) {
    *control_flow = ControlFlow::Wait;
    
    match event {
      Event::WindowEvent { window_id, event } if window_id==self.window.id() => {
        if !self.win_state.on_event(&self.egui_ctx, &event) {
          match event {
            // event::WindowEvent::ModifiersChanged(changed_mod) if changed_mod.contains(event::ModifiersState::CTRL) =>
            //   ctrl_modifier = !ctrl_modifier,
            event::WindowEvent::Resized(PhysicalSize { width, height}) =>
              self.render_state.resize(Some(width), Some(height), None),
            event::WindowEvent::CloseRequested | event::WindowEvent::Destroyed => 
              *control_flow = ControlFlow::Exit,
            event::WindowEvent::KeyboardInput { input, .. } => match input {
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Escape), state: event::ElementState::Pressed,.. } =>
                *control_flow = ControlFlow::Exit,
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::LControl), state,.. } =>
                self.ctrl_modifier = state == event::ElementState::Pressed,
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Up), state: event::ElementState::Pressed,.. } if self.ctrl_modifier => {
                let scale_factor = self.win_state.pixels_per_point() * 1.25;
                self.win_state.set_pixels_per_point(scale_factor);
                self.render_state.resize(None, None, Some(scale_factor));
              },
              event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::Down), state: event::ElementState::Pressed,.. } if self.ctrl_modifier => {
                let scale_factor = self.win_state.pixels_per_point() / 1.25;
                self.win_state.set_pixels_per_point(scale_factor);
                self.render_state.resize(None, None, Some(scale_factor));
              },
              _ => {}
            },
            event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
              self.render_state.resize(Some(new_inner_size.width), Some(new_inner_size.height), Some(scale_factor as _));
            },
            // winit::event::WindowEvent::ThemeChanged(_) => todo!(),
            _ => {}
          }
        }
      },
      Event::RedrawRequested(window_id) if window_id != self.window.id() => { },
      Event::RedrawRequested(..) | Event::UserEvent(MyEvent::RequestRedraw) => {
        self.render_state.update_window_size_bind_group();
    
        let _did_render = self.render_state.redraw(|| {
          let raw_input = self.win_state.take_egui_input(self.window);
          let full_output = self.egui_ctx.run(raw_input, |ctx| self.ui.ui(ctx));

          self.win_state.handle_platform_output(self.window, self.egui_ctx, full_output.platform_output);
          let paint_jobs = self.egui_ctx.tessellate(full_output.shapes);
          let texture_delta = full_output.textures_delta;
          
          (texture_delta, paint_jobs)
        }).and(Some(true)).unwrap_or(false);
      },
      Event::MainEventsCleared => self.window.request_redraw(),
      _ => {}
    }
  }
}

