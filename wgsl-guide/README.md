# WGPU + WGSL
Das hier soll ein Guide für Rust's Crate [WGPU](https://docs.rs/wgpu/latest/wgpu/) und der Shadersprache [WGSL](https://gpuweb.github.io/gpuweb/wgsl/#compound-statement-section) werden. Dabei geht es in der erster Linie nicht um Codebeispiele oder Best Practices für gängige Probleme, sondern eher einen überschaubaren Überblick für allgemeines Programmieren mit der GPU zu geben. Vorwissen in Sachen Shader- oder GPU-Programmierung ist nicht nötig. Grundkenntnisse in Rust und Low-level programmierung wären allerdings schon gut. Für einen anwendungsnäheren Guide kann ich [Sotrh's Repository](https://github.com/sotrh/learn-wgpu) empfehlen. Anders als ich geht er*sie auch auf Kompilierung nach Wasm ein. 

# Guide 
## Was ist ein Shader und warum braucht man das?
Ein Shader ist im Grunde ein Programm auf der GPU. In WGSL gibt es zwei Arten von Shader-programmen: __Ausgeführte Renderpipelines__ und __Ausgeführte Computepipelines__. In einer Renderpipeline schreibt man die Art von GPU-Programmen, die man vor allem in Spielen sieht und an die die meisten Leute vermutlich bei dem Begriff _Shader_ denken. Diese Art von Shader nimmt typischerweiße Vertices und Texturen und rendert das Ganze in einem 3D Koordinatensystem. Ein Computeshader hingegen ist eher die Art von Programm, die man mit extrem paralleler Programmierung assoziiert. 

In beiden Fällen gibt es bei der Ausführung eines Shaders im Grunde 4 Phasen:
* __Shader module wird erstellt:__
Anders als man es möglicherweiße erwarten würde, werden Shader nicht notwendigerweiße bei der Kompilierung des Rust-programmes mitkompiliert. Stattdessen muss man sich als Programmierer entweder selbst darum kümmern, dass das Program den Shader in einem binären Format lädt/enthält, oder es zur Laufzeit kompilieren. Der Grund dafür ist, dass es in Sachen Grafikkartenarchitektur kaum herstellerübergreifende Standards gibt. Selbst verschiedene Generationen vom selben Hersteller unterscheiden sich teilweiße stark. Damit man als Programmierer nun nicht die Shader an duzende Grafikkarten anpassen muss, spricht man nicht direkt die Hardware an, sondern eine API, wie Vulkan/Metal/OpenGL/Direct X/etc. Um uns das Leben noch einfacher zu machen (und ggf GPU-Anwendungen im Browser ausführen zu können) nutzen wir __WGPU__ und __WGSL__, da so die Implementierung plattformunabhängig ist. 

* __Pipeline wird erstellt:__
Da die GPU idR eine seperate Processing Unit ist, mit eigenem dediziertem RAM, müssen alle Resourcen vom Stack/Heap in den Grafikspeicher rüberkopiert werden und ggf werden auch Resourcen zurückkopiert (teilweise spricht man auch vom Hoch- und Runterladen). Oft haben Grafikkarten spezielle Optimierungen um bspw gängige Berechnungen wie Antialiasing für Texturen sehr effizient durchführen zu können. Um von all dem Gebrauch zu machen muss vor der Ausführung festgelegt werden, welche Daten wie wohin kopiert werden und das ganze nach Möglichkeit in einem einzigen Kopiervorgang pro Shaderausführung, da die Synchronisierung zwischen CPU und GPU ansonsten teuer werden kann. Die __Pipeline__ representiert ein Objekt, das genau diese Informationen festlegt.

* __Shader wird ausgeführt:__
Der Shader läuft auf der GPU. Diese Phase wird ausgeführt wenn _draw_ für Render Shader bzw _dispatch_ für Compute Shader aufgerufen wird.

* __Shader wird beendet:__
Daten werden ggf zurückkopiert.

## Allgemeine erste Schritte
Bevor man das Shader module und die Pipeline erstellen kann, muss man etwas Boilercode schreiben um einen Handle für die Grafikkarte zu haben. Das erste was wir im Rustcode generieren ist eine `instance: wgpu::Instance`. Es ist ein Handle um einerseits einen Handle, `adapter: wgpu::Adapter`, für die Hardware im System zu finden und andererseits um eine Zeichenoberfläche, `surface: wgpu::Surface`, zu definieren. Die Oberfläche ist idR das Handle für die GUI, ist aber optional.
```toml
#cargo.toml
[package]
name = "wgsl-guide"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
wgpu = "*"
pollster = "*"
```

```rust
//main.rs
use std::env;
use wgpu;

async fn run_main(adapter_idx: Option<usize>) {
  let instance = wgpu::Instance::new(
    wgpu::Backends::all() //Use the "best" backend-API available
  );

  let all_adapters = instance
    .enumerate_adapters(wgpu::Backends::all())
    .collect::<Vec<_>>();

  let adapter_store;
  let adapter: &wgpu::Adapter = match adapter_idx {
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
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let idx = args.get(1).and_then(|i_str| i_str.parse::<usize>().ok());
  pollster::block_on(run_main(idx))
}
```
Auf meinem Rechner erhalte ich:
```
List all available adapters. Selected one is marked with "*"
* Name:   	AMD Radeon RX 550 / 550 Series (RADV POLARIS12)
  vendor: 	4098
  device: 	27039 (DiscreteGpu)
  Driver: 	radv - Mesa 22.2.3
  Backend:	Vulkan
- Name:   	AMD Radeon RX 550 / 550 Series
  vendor: 	4098
  device: 	27039 (DiscreteGpu)
  Driver: 	AMD proprietary driver - No driver info
  Backend:	Vulkan
- Name:   	AMD Radeon RX 550 / 550 Series (polaris12, LLVM 14.0.6, DRM 3.48, 6.0.8-1-MANJARO)
  vendor: 	4098
  device: 	0 (Other)
  Driver: 	Unknown driver name - No driver info
  Backend:	Gl
```
Wie man sehen kann findet `wgpu` einerseits die Opensource Treiber, aber auch die Proparitären. Meine möglichen Backends sind Vulkan und OpenGL. Genau das habe ich erwartet.

`pollster` ist eine Library um asynchrone Funktionen auszurühren.

Das erste was wir in der main machen, ist es den ersten Parameter als `usize` zu parsen und damit `run_main` aufrufen. In `run_main` rufe ich zuerst
```rust
let instance = wgpu::Instance::new(
  wgpu::Backends::all() //Use the "best" backend-API available
);
let all_adapters = instance
  .enumerate_adapters(wgpu::Backends::all())
  .collect::<Vec<_>>();
```
auf. `wgpu::Backends::all()` gibt an, dass ich alle verfügbaren Backends akzeptiere und `all_adapters` listet alle gefundenen Adapter auf. Falls `adapter_idx` gesetzt wurde, wird der entsprechende Adapter ausgewählt und in `adapter: wgpu::Adapter` gespeichert. Andernfalls wird über
```rust
instance.request_adapter(&wgpu::RequestAdapterOptions {
  power_preference: wgpu::PowerPreference::HighPerformance, //Usually what you want for dedicated GPUs
  force_fallback_adapter: false,
  compatible_surface: None, //No render surface
}).await;
```
der "passendste" Adapter ausgewählt. Für `power_preference` und `force_fallback_adapter` siehe die Dokumentation. `compatible_surface` setzen wir aktuell auf `None`, aber später zeigen wir man das Ganze direkt als Window widget einbindet. Zum Schluss geben wir alle Adapter im Errorstream aus.

Nachdem der Adapter ausgewählt ist, fehlt nur noch
```rust
let (device, queue) = adapter.request_device(
  &wgpu::DeviceDescriptor {
    label: Some("Device Descriptor"),
    features: wgpu::Features::default(),
    limits: wgpu::Limits::default(),
  },
  None,
).await.unwrap();
```
`device` definiert die GPU zusammen mit der gewählten API und Features. `queue` representiert den Beginn der Shader Pipeline. Damit haben wir alles fertig um einen Shader zu schreiben.

## Compute Pipeline
Für die meisten sind Compute Shader vermutlich weniger interessant und nichiger als die allbekannten Render Shader, allerdings sind sie sehr viel einfacher zu verstehen. Von daher erkläre ich sie zuerst.

