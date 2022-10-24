#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// mod video;
// mod bz2_binding;
use std::path;

use eframe::egui;

fn main() {
  let options = eframe::NativeOptions::default();
  eframe::run_native(
    "My egui App",
    options,
    Box::new(|_cc| Box::new(MyApp::default())),
  );
}

struct MyApp {
  pub img_path: String,
  pub img_texture: Option<egui::TextureHandle>,
  pub img_scale: f32,

  pub video_path: String,
  pub video_texture: Option<egui::TextureHandle>,
  pub video_scale: f32,
  pub video_skip_frames: i32,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      img_path: "logo.png".to_owned(),
      img_texture: None,
      img_scale: 0.5,

      video_path: "".to_owned(),
      video_texture: None,
      video_scale: 0.2,
      video_skip_frames: 0
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      self.ui_image(ctx, ui);
      ui.separator();
      self.ui_video_fram(ctx, ui);
    });
  }

}
impl MyApp {
  fn ui_image(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
    ui.heading("Shows an image");
    ui.add(egui::Slider::new(&mut self.img_scale, 0.1..=1.0).text("scale"));

    if self.img_texture.is_some(){
      let tex = self.img_texture.as_ref().unwrap();
      ui.image(tex, tex.size_vec2()*self.img_scale);
    }
    ui.label("Pfad:");
    ui.text_edit_singleline(&mut self.img_path);
    if ui.button("Open").clicked(){
      let path = path::Path::new(self.img_path.as_str());
      if let Ok(img) = load_image_from_path(path) {
        let tex = ui.ctx().load_texture(
          self.img_path.clone(),
          img,
          egui::TextureFilter::Linear);
        let _tex = self.img_texture.insert(tex);
      }
    }

  }
  
  fn ui_video_fram(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui){
    ui.heading("Renders a frame from a video file");
    ui.add(egui::Slider::new(&mut self.video_scale, 0.1..=1.0).text("scale"));

    if self.video_texture.is_some(){
      let tex = self.video_texture.as_ref().unwrap();
      ui.image(tex, tex.size_vec2()*self.video_scale);
    }
    ui.label("Pfad:");
    ui.text_edit_singleline(&mut self.video_path);
    if ui.button("Open").clicked(){
      // let path = path::Path::new(self.video_path.as_str());
      if let Ok(img) = load_frame_from_path(self.video_path.as_str(), self.video_skip_frames) {
        let tex = ui.ctx().load_texture(
          self.video_path.clone(),
          img,
          egui::TextureFilter::Linear);
        let _tex = self.video_texture.insert(tex);
      }
    }
  }

}

fn load_frame_from_path(path: &str, skip_frames: i32) -> Result<egui::ColorImage, image::ImageError>{
  let frm;
  match video::Frame::from(path, skip_frames) {
    Some(_frm) => frm = _frm,
    None => return Err(image::ImageError::IoError(std::io::Error::new(
      std::io::ErrorKind::Other, 
      format!("Couldn't open frame {} from {}", skip_frames, path)
    )))
  }
  let (y,u,v) = (frm.channel(0), frm.channel(1), frm.channel(2));
  let size = [frm.width() as _, frm.height() as _];
  // egui::ColorImage::from
  let img_buf = image::ImageBuffer::from_fn(size[0] as _, size[1] as _, |a,b| {
    let i: usize = (a as usize)+size[0]*(b as usize);
    let (r,g,b) = (y[i],y[i],y[i]);
    image::Rgba([r,g,b,0 as _])
  });
  Ok(egui::ColorImage::from_rgba_unmultiplied(
    size,
    img_buf.as_flat_samples().as_slice(),
  ))
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
  let image = image::io::Reader::open(path)?.decode()?;
  let size = [image.width() as _, image.height() as _];
  let image_buffer = image.to_rgba8();
  let pixels = image_buffer.as_flat_samples();
  Ok(egui::ColorImage::from_rgba_unmultiplied(
    size,
    pixels.as_slice(),
  ))
}