#![allow(dead_code)]

use skia_safe::{surfaces, AlphaType, Color, ColorType, ISize, ImageInfo, Rect};

use crate::context::paths::path::Path;
use crate::context::text_styles::text_direction::TextDirection;
use crate::context::{Context, State, SurfaceData, SurfaceEngine, SurfaceState};

const GR_GL_RGB565: u32 = 0x8D62;
const GR_GL_RGBA8: u32 = 0x8058;

impl Context {
    pub fn new(
        width: f32,
        height: f32,
        density: f32,
        alpha: bool,
        font_color: i32,
        ppi: f32,
        direction: TextDirection,
    ) -> Self {
        let mut state = State::default();
        state.direction = direction;

        let color_type = if alpha {
            ColorType::RGBA8888
        } else {
            ColorType::RGB565
        };

        let alpha_type = if alpha {
            AlphaType::Unpremul
        } else {
            AlphaType::Premul
        };

        let info = ImageInfo::new(
            ISize::new(width as i32, height as i32),
            color_type,
            alpha_type,
            None,
        );

        let surface = surfaces::raster(&info, None, None).unwrap();
        let bounds = Rect::from_wh(width, height);

        Context {
            direct_context: None,
            #[cfg(feature = "gl")]
            gl_context: None,
            #[cfg(feature = "metal")]
            metal_context: None,
            #[cfg(feature = "metal")]
            metal_texture_info: None,
            #[cfg(feature = "vulkan")]
            vulkan_context: None,
            #[cfg(feature = "vulkan")]
            vulkan_texture: None,
            surface_data: SurfaceData {
                bounds,
                ppi,
                scale: density,
                engine: SurfaceEngine::CPU,
                state: Default::default(),
                is_opaque: !alpha,
            },
            surface,
            path: Path::default(),
            state,
            state_stack: vec![],
            font_color: Color::new(font_color as u32),
            surface_state: SurfaceState::None,
        }
    }

    pub fn resize(
        context: &mut Context,
        width: f32,
        height: f32,
        density: f32,
        alpha: bool,
        ppi: f32,
    ) {
        let color_type = if alpha {
            ColorType::RGBA8888
        } else {
            ColorType::RGB565
        };

        let alpha_type = if alpha {
            AlphaType::Unpremul
        } else {
            AlphaType::Premul
        };

        let bounds = Rect::from_wh(width, height);
        let info = if bounds.is_empty() {
            let mut width = width;
            if width <= 0. {
                width = 1.
            }
            let mut height = height;

            if height <= 0. {
                height = 1.
            }

            ImageInfo::new(ISize::new(width as i32, height as i32), color_type, alpha_type, None)
        } else {
            ImageInfo::new(
                ISize::new(width as i32, height as i32),
                color_type,
                alpha_type,
                None,
            )
        };

        if let Some(surface) = surfaces::raster(&info, None, None) {
            context.surface = surface;
            context.surface_data.is_opaque = !alpha;
            context.surface_data.state = Default::default();
            context.surface_data.bounds = bounds;
            context.surface_data.scale = density;
            context.surface_data.ppi = ppi;
            context.surface_state = SurfaceState::None;
            context.path = Path::default();
            context.reset_state();
        }
    }

    pub fn flush_buffer(context: &mut Context, width: i32, height: i32, density: f32, buffer: &mut [u8]) {
        if context.surface_data.bounds.is_empty() {
            return;
        }
        let info = ImageInfo::new(
            ISize::new(width, height),
            ColorType::RGBA8888,
            AlphaType::Unknown,
            None,
        );
        let mut surface = surfaces::wrap_pixels(&info, buffer, None, None).unwrap();
        {
            let canvas = surface.canvas();
            let mut paint = skia_safe::Paint::default();
            paint.set_anti_alias(true);
            paint.set_style(skia_safe::PaintStyle::Fill);
            paint.set_blend_mode(skia_safe::BlendMode::Clear);
            canvas.draw_rect(
                Rect::from_xywh(
                    0f32,
                    0f32,
                    info.width() as f32,
                    info.height() as f32,
                ),
                &paint,
            );
        }

        context.draw_on_surface(&mut surface);
    }
}
