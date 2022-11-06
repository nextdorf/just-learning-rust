use wgpu::{TextureView, Sampler, Device, Queue, Extent3d,
    TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, 
    Origin3d, TextureAspect, ImageDataLayout, TextureViewDescriptor, 
    SamplerDescriptor, AddressMode, FilterMode};

pub struct Texture {
  pub texture: wgpu::Texture,
  pub view: TextureView,
  pub sampler: Sampler,
}

impl Texture {
  pub fn from_bytes(device: &Device, queue: &Queue, bytes: &[u8], label: &str) -> Result<Self, image::ImageError> {
    let img = image::load_from_memory(bytes)?;
    Self::from_image(device, queue, &img, Some(label))
  }

  pub fn from_image(device: &Device, queue: &Queue, img: &image::DynamicImage, label: Option<&str>) -> Result<Self, image::ImageError> {
    let rgba8 = img.to_rgba8();
    let img_dims = rgba8.dimensions();
    let texture_dims = Extent3d {
      width: img_dims.0,
      height: img_dims.1,
      depth_or_array_layers: 1,
    };
    let texture = device.create_texture(
      &TextureDescriptor {
        label,
        size: texture_dims,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8UnormSrgb,
        usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
    });
    queue.write_texture(
      wgpu::ImageCopyTextureBase {
        texture: &texture,
        mip_level: 0,
        origin: Origin3d::ZERO,
        aspect: TextureAspect::All
      },
      &rgba8,
      ImageDataLayout {
        offset: 0,
        bytes_per_row: std::num::NonZeroU32::new(img_dims.0*4),
        rows_per_image: std::num::NonZeroU32::new(img_dims.1),
      },
      texture_dims
    );
    let view = texture.create_view(
      &TextureViewDescriptor::default() );
    let sampler = device.create_sampler(
      &SamplerDescriptor {
        address_mode_u: AddressMode::ClampToEdge,
        address_mode_v: AddressMode::ClampToEdge,
        address_mode_w: AddressMode::ClampToEdge,
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Nearest,
        mipmap_filter: FilterMode::Nearest,
        ..SamplerDescriptor::default()
    });

    Ok(Self { texture, view, sampler })
  }

}


