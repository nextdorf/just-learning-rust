#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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
  pub path: String,
  pub texture: Option<egui::TextureHandle>,
  pub scale: f32,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      path: "logo.png".to_owned(),
      texture: None,
      scale: 0.5
    }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Shows an image");
      ui.add(egui::Slider::new(&mut self.scale, 0.1..=1.0).text("scale"));

      if self.texture.is_some(){
        let tex = self.texture.as_ref().unwrap();
        ui.image(tex, tex.size_vec2()*self.scale);
      }
      ui.label("Pfad:");
      ui.text_edit_singleline(&mut self.path);
      if ui.button("Open").clicked(){
        let path = path::Path::new(self.path.as_str());
        if let Ok(img) = load_image_from_path(path) {
          let tex = ui.ctx().load_texture(
            self.path.clone(),
            img,
            egui::TextureFilter::Linear);
          let _tex = self.texture.insert(tex);
        }
      }

    });
  }
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