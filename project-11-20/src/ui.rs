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

pub mod constants {
  pub const ZOOM_100: f32 = 1.25;
  pub const ZOOM_PLUS: f32 = 1.125;
}

pub struct UI {
  img_hnd: Option<Vec<TextureHandle>>,
  test_var: usize,
  pub lctrl_modifier: bool,
  pub rctrl_modifier: bool,
}

impl Default for UI {
  fn default() -> Self {
    Self { img_hnd: None, test_var: 0, lctrl_modifier: false, rctrl_modifier: false }
  }
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

  pub fn ctrl_modifier(&self) -> bool {
    self.lctrl_modifier || self.rctrl_modifier
  }
}


pub fn handle_events(event: Event<MyEvent>, _window_target: &EventLoopWindowTarget<MyEvent>, control_flow: &mut ControlFlow,
  window: &Window, win_state: &mut State, ctx: &egui::Context, render_state: &mut WgpuState, ui_state: &mut UI)
{
  *control_flow = ControlFlow::Wait;
    
  match event {
    Event::WindowEvent { window_id, event } if window_id==window.id() => {
      if !win_state.on_event(&ctx, &event) {
        match event {
          event::WindowEvent::Resized(PhysicalSize { width, height}) =>
            render_state.resize(Some(width), Some(height), None, win_state),
          event::WindowEvent::CloseRequested | event::WindowEvent::Destroyed => 
            *control_flow = ControlFlow::Exit,
          event::WindowEvent::KeyboardInput { input, .. } => {
            if let event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::LControl), state,.. } = input {
              ui_state.lctrl_modifier = state == event::ElementState::Pressed;
            }
            else if let event::KeyboardInput { virtual_keycode: Some(event::VirtualKeyCode::RControl), state,.. } = input {
              ui_state.rctrl_modifier = state == event::ElementState::Pressed;
            }
            else if let event::KeyboardInput { virtual_keycode: Some(keycode), state: event::ElementState::Pressed,.. } = input {
              match keycode {
                event::VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                _ => {}
              }
            }
            if ui_state.ctrl_modifier() {
              if let event::KeyboardInput { virtual_keycode: Some(keycode), state: event::ElementState::Pressed,.. } = input {
                match keycode {
                  event::VirtualKeyCode::Plus | event::VirtualKeyCode::NumpadAdd => {
                    let scale_factor = win_state.pixels_per_point() * constants::ZOOM_PLUS;
                    render_state.resize(None, None, Some(scale_factor), win_state);
                  },
                  event::VirtualKeyCode::Minus | event::VirtualKeyCode::NumpadSubtract => {
                    let scale_factor = win_state.pixels_per_point() / constants::ZOOM_PLUS;
                    render_state.resize(None, None, Some(scale_factor), win_state);
                  },
                  event::VirtualKeyCode::Key0 | event::VirtualKeyCode::Numpad0 =>
                    render_state.resize(None, None, Some(constants::ZOOM_100), win_state),
                  _ => {}
                }
              }
            }
          },
          event::WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {
            render_state.resize(Some(new_inner_size.width), Some(new_inner_size.height), Some(scale_factor as _), win_state);
          },
          _ => {}
        }
      }
    },
    Event::RedrawRequested(window_id) if window_id != window.id() => { },
    Event::RedrawRequested(..) | Event::UserEvent(MyEvent::RequestRedraw) => {
      render_state.update_window_size_bind_group(false);
  
      let _did_render = render_state.redraw(|| {
        let raw_input = win_state.take_egui_input(&window);
        let full_output = ctx.run(raw_input, |ctx| ui_state.ui(ctx));

        win_state.handle_platform_output(&window, &ctx, full_output.platform_output);
        let paint_jobs = ctx.tessellate(full_output.shapes);
        let texture_delta = full_output.textures_delta;
        
        (texture_delta, paint_jobs)
      }).and(Some(true)).unwrap_or(false);
    },
    Event::MainEventsCleared => window.request_redraw(),
    _ => {}
  }
}

