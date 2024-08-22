use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::OnceLock;

use core_foundation::base::TCFType;
use core_foundation::bundle::{CFBundleGetBundleWithIdentifier, CFBundleGetFunctionPointerForName};
use core_foundation::string::CFString;
use icrate::objc2::rc::Id;
use icrate::objc2::{class, msg_send, msg_send_id};
use objc2_foundation::NSObject;
use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use raw_window_handle::RawWindowHandle;

use crate::context_attributes::ContextAttributes;

pub static IS_GL_SYMBOLS_LOADED: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Default)]
pub struct GLContextInner {
    context: Option<NSOpenGLContext>,
    sharegroup: NSOpenGLContext,
    view: Option<NSOpenGLView>,
    transfer_surface_info: crate::gl::TransferSurface,
}

unsafe impl Sync for GLContextInner {}

unsafe impl Send for GLContextInner {}
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPixelFormatAttribute {
    NSOpenGLPFAAllRenderers = 1,
    NSOpenGLPFATripleBuffer = 3,
    NSOpenGLPFADoubleBuffer = 5,
    NSOpenGLPFAStereo = 6,
    NSOpenGLPFAAuxBuffers = 7,
    NSOpenGLPFAColorSize = 8,
    NSOpenGLPFAAlphaSize = 11,
    NSOpenGLPFADepthSize = 12,
    NSOpenGLPFAStencilSize = 13,
    NSOpenGLPFAAccumSize = 14,
    NSOpenGLPFAMinimumPolicy = 51,
    NSOpenGLPFAMaximumPolicy = 52,
    NSOpenGLPFAOffScreen = 53,
    NSOpenGLPFAFullScreen = 54,
    NSOpenGLPFASampleBuffers = 55,
    NSOpenGLPFASamples = 56,
    NSOpenGLPFAAuxDepthStencil = 57,
    NSOpenGLPFAColorFloat = 58,
    NSOpenGLPFAMultisample = 59,
    NSOpenGLPFASupersample = 60,
    NSOpenGLPFASampleAlpha = 61,
    NSOpenGLPFARendererID = 70,
    NSOpenGLPFASingleRenderer = 71,
    NSOpenGLPFANoRecovery = 72,
    NSOpenGLPFAAccelerated = 73,
    NSOpenGLPFAClosestPolicy = 74,
    NSOpenGLPFARobust = 75,
    NSOpenGLPFABackingStore = 76,
    NSOpenGLPFAMPSafe = 78,
    NSOpenGLPFAWindow = 80,
    NSOpenGLPFAMultiScreen = 81,
    NSOpenGLPFACompliant = 83,
    NSOpenGLPFAScreenMask = 84,
    NSOpenGLPFAPixelBuffer = 90,
    NSOpenGLPFARemotePixelBuffer = 91,
    NSOpenGLPFAAllowOfflineRenderers = 96,
    NSOpenGLPFAAcceleratedCompute = 97,
    NSOpenGLPFAOpenGLProfile = 99,
    NSOpenGLPFAVirtualScreenCount = 128,
}

#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSOpenGLPFAOpenGLProfiles {
    NSOpenGLProfileVersionLegacy = 0x1000,
    NSOpenGLProfileVersion3_2Core = 0x3200,
    NSOpenGLProfileVersion4_1Core = 0x4100,
}

#[derive(Clone, Debug)]
pub struct NSOpenGLContext(Id<NSObject>);

impl NSOpenGLContext {
    pub fn new(format: NSOpenGLPixelFormat, share_context: Option<NSOpenGLContext>) -> Self {
        let cls = class!(NSOpenGLPixelFormat);
        let instance = unsafe { msg_send_id![cls, alloc] };
        match share_context {
            None => {
                let nil: *mut NSObject = std::ptr::null_mut();
                NSOpenGLContext(unsafe {
                    msg_send_id![
                        instance,
                        initWithFormat: &*format.0,
                        shareContext: nil
                    ]
                })
            }
            Some(share_context) => NSOpenGLContext(unsafe {
                msg_send_id![
                    instance,
                    initWithFormat: &*format.0,
                    shareContext: &*share_context.0
                ]
            }),
        }
    }

    pub fn make_current_context(&self) -> bool {
        let _: () = unsafe { msg_send![&self.0, makeCurrentContext] };
        // confirm ??
        true
    }

    pub fn current_context() -> Option<NSOpenGLContext> {
        let cls = class!(NSOpenGLContext);
        let context: Option<Id<NSObject>> = unsafe { msg_send_id![cls, currentContext] };
        context.map(NSOpenGLContext)
    }

    pub fn remove_current_context(&self) -> bool {
        unsafe {
            let cls = class!(NSOpenGLContext);
            let _: () = msg_send![cls, clearCurrentContext];
            true
        }
    }

    pub fn remove_if_current(&self) -> bool {
        unsafe {
            let cls = class!(NSOpenGLContext);
            let current: Option<Id<NSObject>> = msg_send_id![cls, currentContext];

            match current {
                Some(current) => {
                    let is_equal: bool = msg_send![&current, isEqual: &*self.0];
                    if is_equal {
                        let _: () = msg_send![cls, clearCurrentContext];
                        return true;
                    }
                    false
                }
                None => false,
            }
        }
    }
}

impl Default for NSOpenGLContext {
    fn default() -> Self {
        let format = NSOpenGLPixelFormat::init_with_attributes(&[
            NSOpenGLPixelFormatAttribute::NSOpenGLPFAAccelerated as u32,
            NSOpenGLPixelFormatAttribute::NSOpenGLPFADoubleBuffer as u32,
            NSOpenGLPixelFormatAttribute::NSOpenGLPFADepthSize as u32,
            24,
            NSOpenGLPixelFormatAttribute::NSOpenGLPFAColorSize as u32,
            24,
            NSOpenGLPixelFormatAttribute::NSOpenGLPFAAlphaSize as u32,
            8,
            NSOpenGLPixelFormatAttribute::NSOpenGLPFAStencilSize as u32,
            8,
            NSOpenGLPixelFormatAttribute::NSOpenGLPFAOpenGLProfile as u32,
            NSOpenGLPFAOpenGLProfiles::NSOpenGLProfileVersion3_2Core as u32,
            0,
        ]);
        NSOpenGLContext::new(format, None)
    }
}

#[derive(Clone, Debug)]
pub struct NSOpenGLPixelFormat(Id<NSObject>);

impl NSOpenGLPixelFormat {
    pub fn init_with_attributes(attribs: &[u32]) -> Self {
        let cls = class!(NSOpenGLPixelFormat);
        let instance = unsafe { msg_send_id![cls, alloc] };
        NSOpenGLPixelFormat(unsafe {
            msg_send_id![
                instance,
                initWithAttributes: attribs.as_ptr()
            ]
        })
    }
}

#[derive(Clone, Debug)]
pub struct NSOpenGLView(Id<NSObject>);
impl NSOpenGLView {
    pub fn new_with_frame_pixel_format(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        format: NSOpenGLPixelFormat,
    ) -> Self {
        unsafe {
            let cls = class!(NSOpenGLView);
            let instance = msg_send_id![cls, alloc];
            let point = objc2_foundation::CGPoint::new(x as f64, y as f64);
            let size = objc2_foundation::CGSize::new(width as f64, height as f64);
            let frame = objc2_foundation::CGRect::new(point, size);
            NSOpenGLView(msg_send_id![instance, initWithFrame: frame, pixelFormat: &*format.0])
        }
    }

    pub fn frame(&self) -> objc2_foundation::CGRect {
        let frame: objc2_foundation::CGRect = unsafe { msg_send![&self.0, frame] };
        frame
    }

    pub fn flush_buffer(&self) {
        let _: () = unsafe { msg_send![&self.0, flushBuffer] };
    }

    pub fn open_gl_context(&self) -> NSOpenGLContext {
        let context: Id<NSObject> = unsafe { msg_send_id![&self.0, openGLContext] };
        NSOpenGLContext(context)
    }

    pub fn set_open_gl_context(&self, context: Option<NSOpenGLContext>) {
        match context {
            Some(context) => {
                let _: () = unsafe { msg_send![&self.0, setOpenGLContext: &*context.0] };
            }
            None => {
                let nil: *mut NSObject = std::ptr::null_mut();
                let _: () = unsafe { msg_send![&self.0, setOpenGLContext: nil] };
            }
        }
    }

    fn get_proc_address(&self, addr: &str) -> *const c_void {
        let symbol_name = CFString::new(addr);
        let framework_name = CFString::new("com.apple.opengles");
        unsafe {
            let framework = CFBundleGetBundleWithIdentifier(framework_name.as_concrete_TypeRef());
            CFBundleGetFunctionPointerForName(framework, symbol_name.as_concrete_TypeRef()).cast()
        }
    }
}

#[derive(Debug, Default)]
pub struct GLContext(Arc<RwLock<GLContextInner>>);

impl GLContext {
    // pointer has to
    pub fn as_raw_inner(&self) -> *const RwLock<GLContextInner> {
        Arc::into_raw(Arc::clone(&self.0))
    }

    pub fn from_raw_inner(raw: *const RwLock<GLContextInner>) -> Self {
        unsafe { Self(Arc::from_raw(raw)) }
    }
}

impl Clone for GLContext {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

fn parse_context_attributes(context_attrs: &ContextAttributes) -> NSOpenGLPixelFormat {
    let mut attributes = vec![
        NSOpenGLPixelFormatAttribute::NSOpenGLPFAAccelerated as u32,
        NSOpenGLPixelFormatAttribute::NSOpenGLPFADoubleBuffer as u32,
        NSOpenGLPixelFormatAttribute::NSOpenGLPFAOpenGLProfile as u32,
        NSOpenGLPFAOpenGLProfiles::NSOpenGLProfileVersion3_2Core as u32,
        NSOpenGLPixelFormatAttribute::NSOpenGLPFAColorSize as u32,
        24,
    ];

    if context_attrs.get_alpha() {
        attributes.push(NSOpenGLPixelFormatAttribute::NSOpenGLPFAAlphaSize as u32);
        attributes.push(8);
    }

    if context_attrs.get_depth() {
        attributes.push(NSOpenGLPixelFormatAttribute::NSOpenGLPFADepthSize as u32);
        attributes.push(24);
    }

    if context_attrs.get_stencil() {
        attributes.push(NSOpenGLPixelFormatAttribute::NSOpenGLPFAStencilSize as u32);
        attributes.push(8);
    }

    if !context_attrs.get_is_canvas() && context_attrs.get_antialias() {
        attributes.push(NSOpenGLPixelFormatAttribute::NSOpenGLPFAMultisample as u32);
        attributes.push(NSOpenGLPixelFormatAttribute::NSOpenGLPFASampleBuffers as u32);
        attributes.push(1u32);
        attributes.push(NSOpenGLPixelFormatAttribute::NSOpenGLPFASamples as u32);
        attributes.push(4);
    }

    NSOpenGLPixelFormat::init_with_attributes(attributes.as_slice())
}

#[cfg(target_os = "macos")]
impl GLContext {
    pub fn set_surface(&mut self, view: NonNull<c_void>) -> bool {
        let view: Option<Id<NSObject>> = unsafe { Id::retain(view.as_ptr() as _) };
        match view {
            None => false,
            Some(id) => {
                self.0.write().view = Some(NSOpenGLView(id));
                true
            }
        }
    }

    pub fn set_surface_with_window(&mut self, window: RawWindowHandle) -> bool {
        match window {
            RawWindowHandle::AppKit(handle) => self.set_surface(handle.ns_view),
            _ => false,
        }
    }

    pub fn create_shared_window_context(
        context_attrs: &mut ContextAttributes,
        view: NonNull<c_void>,
        context: &GLContext,
    ) -> Option<GLContext> {
        let view = unsafe { Id::<NSObject>::retain(view.as_ptr() as _) };
        match view {
            None => None,
            Some(view) => {
                let view = NSOpenGLView(view);
                GLContext::create_window_context_with_gl_view(context_attrs, view, Some(context))
            }
        }
    }

    pub fn create_window_context(
        context_attrs: &mut ContextAttributes,
        view: NonNull<c_void>,
    ) -> Option<GLContext> {
        let view = unsafe { Id::<NSObject>::retain(view.as_ptr() as _) };
        match view {
            None => None,
            Some(view) => {
                let view = NSOpenGLView(view);
                GLContext::create_window_context_with_gl_view(context_attrs, view, None)
            }
        }
    }

    pub(crate) fn create_window_context_with_gl_view(
        context_attrs: &mut ContextAttributes,
        view: NSOpenGLView,
        shared_context: Option<&GLContext>,
    ) -> Option<GLContext> {
        let view_copy = view.clone();
        IS_GL_SYMBOLS_LOADED.get_or_init(move || {
            gl_bindings::load_with(|symbol| view_copy.get_proc_address(symbol).cast());
            true
        });

        let share_group = match shared_context {
            Some(context) => {
                let inner = context.0.read();
                inner.sharegroup.clone()
            }
            _ => {
                let format = NSOpenGLPixelFormat::init_with_attributes(&[
                    NSOpenGLPixelFormatAttribute::NSOpenGLPFAAccelerated as u32,
                    NSOpenGLPixelFormatAttribute::NSOpenGLPFADoubleBuffer as u32,
                    NSOpenGLPixelFormatAttribute::NSOpenGLPFADepthSize as u32,
                    24,
                    NSOpenGLPixelFormatAttribute::NSOpenGLPFAColorSize as u32,
                    24,
                    NSOpenGLPixelFormatAttribute::NSOpenGLPFAAlphaSize as u32,
                    8,
                    NSOpenGLPixelFormatAttribute::NSOpenGLPFAStencilSize as u32,
                    8,
                    NSOpenGLPixelFormatAttribute::NSOpenGLPFAOpenGLProfile as u32,
                    NSOpenGLPFAOpenGLProfiles::NSOpenGLProfileVersion3_2Core as u32,
                    0,
                ]);
                NSOpenGLContext::new(format, None)
            }
        };

        let format = parse_context_attributes(context_attrs);

        let context = NSOpenGLContext::new(format, Some(share_group.clone()));

        // if context.is_none() {
        //     return None;
        // }

        view.set_open_gl_context(Some(context.clone()));

        let inner = GLContextInner {
            context: Some(context),
            sharegroup: share_group,
            view: Some(view),
            transfer_surface_info: Default::default(),
        };

        Some(GLContext(Arc::new(RwLock::new(inner))))
    }

    pub fn create_offscreen_context(
        context_attrs: &mut ContextAttributes,
        width: i32,
        height: i32,
    ) -> Option<GLContext> {
        let format = parse_context_attributes(context_attrs);
        let view =
            NSOpenGLView::new_with_frame_pixel_format(0., 0., width as f32, height as f32, format);

        GLContext::create_window_context_with_gl_view(context_attrs, view, None)
    }

    fn has_extension(extensions: &str, name: &str) -> bool {
        !extensions.split(' ').into_iter().any(|s| s == name)
    }

    pub fn has_gl2support() -> bool {
        true
    }

    pub fn set_vsync(&self, _sync: bool) -> bool {
        true
    }

    pub fn make_current(&self) -> bool {
        let inner = self.0.read();
        inner
            .view
            .as_ref()
            .map(|view| view.open_gl_context().make_current_context())
            .unwrap_or_default()
    }

    pub fn remove_if_current(&self) -> bool {
        let inner = self.0.read();
        inner
            .view
            .as_ref()
            .map(|view| view.open_gl_context().remove_if_current())
            .unwrap_or_default()
    }

    pub fn swap_buffers(&self) -> bool {
        let inner = self.0.read();
        inner
            .view
            .as_ref()
            .map(|view| {
                view.flush_buffer();
                true
            })
            .unwrap_or_default()
    }

    pub fn get_surface_width(&self) -> i32 {
        let inner = self.0.read();
        inner
            .view
            .as_ref()
            .map(|view| view.frame().size.width as i32)
            .unwrap_or_default()
    }

    pub fn get_surface_height(&self) -> i32 {
        let inner = self.0.read();
        inner
            .view
            .as_ref()
            .map(|view| view.frame().size.height as i32)
            .unwrap_or_default()
    }

    pub fn get_surface_dimensions(&self) -> (i32, i32) {
        let inner = self.0.read();
        inner
            .view
            .as_ref()
            .map(|view| {
                let frame = view.frame();
                (frame.size.width as i32, frame.size.height as i32)
            })
            .unwrap_or_default()
    }

    pub fn get_transfer_surface_info(&self) -> MappedRwLockReadGuard<crate::gl::TransferSurface> {
        RwLockReadGuard::map(self.0.read(), |v| &v.transfer_surface_info)
    }

    pub fn get_transfer_surface_info_mut(
        &self,
    ) -> MappedRwLockWriteGuard<crate::gl::TransferSurface> {
        RwLockWriteGuard::map(self.0.write(), |v| &mut v.transfer_surface_info)
    }
}