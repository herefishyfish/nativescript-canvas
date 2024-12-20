use std::os::raw::c_void;

use canvas_c::WebGLState;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jint, jlong, JNI_TRUE};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn nativeTexImage2D(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    target: jint,
    level: jint,
    internalformat: jint,
    format: jint,
    type_: jint,
    bitmap: JObject,
    flip_y: jboolean,
) {
    if context == 0 {
        return;
    }

    let context = context as *mut WebGLState;
    let context = unsafe { &mut *context };

    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);

    if let Some((bytes, info)) = bytes {
        context.get_inner().make_current();
        let width = info.width();
        let height = info.height();
        unsafe {
            if flip_y == JNI_TRUE {
                let mut buffer = bytes;
                canvas_webgl::utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    info.stride() as usize,
                    height as usize,
                );

                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    0,
                    format as u32,
                    type_ as u32,
                    buffer.as_ptr() as *const c_void,
                );
            } else {
                gl_bindings::TexImage2D(
                    target as u32,
                    level,
                    internalformat,
                    width as i32,
                    height as i32,
                    0,
                    format as u32,
                    type_ as u32,
                    bytes.as_ptr() as *const c_void,
                );
            }
        }
    }
}

#[no_mangle]
pub extern "system" fn nativeTexSubImage2D(
    env: JNIEnv,
    _: JClass,
    context: jlong,
    level: jint,
    target: jint,
    xoffset: jint,
    yoffset: jint,
    format: jint,
    type_: jint,
    bitmap: JObject,
    flip_y: jboolean,
) {
    if context == 0 {
        return;
    }

    let context = context as *mut WebGLState;
    let context = unsafe { &mut *context };

    let bytes = crate::utils::image::get_bytes_from_bitmap(&env, bitmap);

    if let Some((bytes, info)) = bytes {
        unsafe {
            context.get_inner().make_current();
            let width = info.width();
            let height = info.height();
            if flip_y == JNI_TRUE {
                let mut buffer = bytes;
                canvas_webgl::utils::gl::flip_in_place(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    info.stride() as usize,
                    height as usize,
                );

                gl_bindings::TexSubImage2D(
                    target as u32,
                    level,
                    xoffset,
                    yoffset,
                    width as i32,
                    height as i32,
                    format as u32,
                    type_ as u32,
                    buffer.as_ptr() as *const c_void,
                );
            } else {
                gl_bindings::TexSubImage2D(
                    target as u32,
                    level,
                    xoffset,
                    yoffset,
                    width as i32,
                    height as i32,
                    format as u32,
                    type_ as u32,
                    bytes.as_ptr() as *const c_void,
                );
            }
        }
    }
}
