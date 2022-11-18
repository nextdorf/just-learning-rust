pub mod compute_example;
use std::env;
use wgpu::{self, Adapter, Device, Queue};

async fn run_main(adapter_idx: Option<usize>) -> (Device, Queue) {
  let instance = wgpu::Instance::new(
    wgpu::Backends::all() //Use the "best" backend-API available
  );

  let all_adapters = instance
    .enumerate_adapters(wgpu::Backends::all())
    .collect::<Vec<_>>();

  let adapter_store;
  let adapter: &Adapter = match adapter_idx {
    Some(idx) if idx < all_adapters.len() =>
      all_adapters.get(idx).unwrap(),
    _ => {
      adapter_store = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance, //Usually what you want for dedicated GPUs
        force_fallback_adapter: false,
        compatible_surface: None, //No render surface
      }).await;
    adapter_store.as_ref().unwrap()
    },
  };

  eprintln!("List all available adapters. Selected one is marked with \"*\":");
  for a in all_adapters.iter() {
    let info = a.get_info();
    let is_adapter = info == adapter.get_info();
    let wgpu::AdapterInfo {
      name, vendor, device, device_type, driver, driver_info, backend
      } = info;
    let selector = if is_adapter {"*"} else {"-"};
    let driver = if driver.len() > 0 {driver.as_str()} else {"Unknown driver name"};
    let driver_info = if driver_info.len() > 0 {driver_info.as_str()} else {"No driver info"};

    eprintln!("{} Name:   \t{name}\n  vendor: \t{vendor}\n  device: \t{device} ({:?})\n  Driver: \t{driver} - {driver_info}\n  Backend:\t{:?}", selector, device_type, backend);
  }

  let (device, queue) = adapter.request_device(
    &wgpu::DeviceDescriptor {
      label: Some("Device"),
      features: wgpu::Features::default(),
      limits: wgpu::Limits::default(),
    },
    None,
  ).await.unwrap();
  
  (device, queue)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let idx = args.get(1).and_then(|i_str| i_str.parse::<usize>().ok());
  let (device, queue) = pollster::block_on(run_main(idx));

  compute_example::run_shader(&device, &queue);
}
