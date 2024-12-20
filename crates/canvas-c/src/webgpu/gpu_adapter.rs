use crate::buffers::StringBuffer;
use crate::webgpu::gpu_device::{ErrorSinkRaw, DEFAULT_DEVICE_LOST_HANDLER};
use crate::webgpu::gpu_queue::QueueId;
use std::sync::Arc;
use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
};

use super::{
    gpu::CanvasWebGPUInstance, gpu_adapter_info::CanvasGPUAdapterInfo, gpu_device::CanvasGPUDevice,
    gpu_queue::CanvasGPUQueue, gpu_supported_limits::CanvasGPUSupportedLimits, prelude::*,
};

//use wgpu_core::gfx_select;

pub struct CanvasGPUAdapter {
    pub(crate) instance: Arc<CanvasWebGPUInstance>,
    pub(crate) adapter: wgpu_core::id::AdapterId,
    pub(crate) is_fallback_adapter: bool,
    pub(crate) features: Vec<&'static str>,
    pub(crate) limits: wgt::Limits,
}

impl Drop for CanvasGPUAdapter {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let global = self.instance.global();
            global.adapter_drop(self.adapter);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_adapter_reference(adapter: *const CanvasGPUAdapter) {
    if adapter.is_null() {
        return;
    }

    Arc::increment_strong_count(adapter);
}

#[no_mangle]
pub unsafe extern "C" fn canvas_native_webgpu_adapter_release(adapter: *const CanvasGPUAdapter) {
    if adapter.is_null() {
        return;
    }

    Arc::decrement_strong_count(adapter);
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_get_features(
    adapter: *const CanvasGPUAdapter,
) -> *mut StringBuffer {
    if adapter.is_null() {
        return std::ptr::null_mut();
    }
    let adapter = unsafe { &*adapter };
    let buffer = StringBuffer::from(adapter.features.clone());
    Box::into_raw(Box::new(buffer))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_is_fallback_adapter(
    adapter: *const CanvasGPUAdapter,
) -> bool {
    if adapter.is_null() {
        return false;
    }
    let adapter = unsafe { &*adapter };
    adapter.is_fallback_adapter
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_get_limits(
    adapter: *const CanvasGPUAdapter,
) -> *mut CanvasGPUSupportedLimits {
    if adapter.is_null() {
        return Box::into_raw(Box::new(wgt::Limits::default().into()));
    }
    let adapter = unsafe { &*adapter };
    Box::into_raw(Box::new(adapter.limits.clone().into()))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_request_adapter_info(
    adapter: *const CanvasGPUAdapter,
) -> *const CanvasGPUAdapterInfo {
    if adapter.is_null() {
        return std::ptr::null_mut();
    }
    let adapter = unsafe { &*adapter };

    let adapter_id = adapter.adapter;
    let global = adapter.instance.global();

    let info = global.adapter_get_info(adapter_id);

    Arc::into_raw(Arc::new(CanvasGPUAdapterInfo::new(info)))
}

#[no_mangle]
pub extern "C" fn canvas_native_webgpu_adapter_request_device(
    adapter: *const CanvasGPUAdapter,
    label: *const c_char,
    required_features: *const *const c_char,
    required_features_length: usize,
    required_limits: *const CanvasGPUSupportedLimits,
    callback: extern "C" fn(*mut c_char, *const CanvasGPUDevice, *mut c_void),
    callback_data: *mut c_void,
) {
    if adapter.is_null() {
        callback(std::ptr::null_mut(), std::ptr::null_mut(), callback_data);
        return;
    }

    let adapter = unsafe { &*adapter };
    let features = parse_required_features(required_features, required_features_length);

    let limits = if required_limits.is_null() {
        wgt::Limits::default()
    } else {
        unsafe { *required_limits }.into()
    };

    let label = ptr_into_label(label);

    let callback = callback as i64;
    let callback_data = callback_data as i64;
    let adapter_id = adapter.adapter;
    let instance = Arc::clone(&adapter.instance);
    std::thread::spawn(move || {
        let descriptor = wgt::DeviceDescriptor {
            label: label.clone(),
            required_features: features,
            required_limits: limits,
            memory_hints: Default::default(),
        };

        let instance_copy = Arc::clone(&instance);

        let global = instance.global();

        let callback = unsafe {
            std::mem::transmute::<*const i64, fn(*mut c_char, *const CanvasGPUDevice, *mut c_void)>(
                callback as _,
            )
        };
        let callback_data = callback_data as *mut c_void;

        match global.adapter_request_device(
            adapter_id,
            &descriptor,
            None,
            None,
            None,
        ) {
            Ok((device, queue)) => {
                let error_sink = Arc::new(parking_lot::Mutex::new(ErrorSinkRaw::new(
                    DEFAULT_DEVICE_LOST_HANDLER,
                )));

                let queue = Arc::new(CanvasGPUQueue {
                    label: descriptor.label,
                    queue: Arc::new(QueueId {
                        id: queue,
                        instance: Arc::clone(&instance_copy),
                    }),
                    error_sink: error_sink.clone(),
                });

                let ret = Arc::into_raw(Arc::new(CanvasGPUDevice {
                    label,
                    device,
                    queue,
                    user_data: std::ptr::null_mut(),
                    instance: instance_copy,
                    error_sink,
                }));
                callback(std::ptr::null_mut(), ret, callback_data);
            }
            Err(error) => {
                let error = error.to_string();
                let ret = CString::new(error).unwrap().into_raw();
                callback(ret, std::ptr::null_mut(), callback_data);
            }
        }
    });
}
