//
// Created by Osei Fortune on 17/07/2024.
//

#ifndef CANVAS_ANDROID_GPURENDERBUNDLEENCODERIMPL_H
#define CANVAS_ANDROID_GPURENDERBUNDLEENCODERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"

class GPURenderBundleEncoderImpl : ObjectWrapperImpl {

public:
    explicit GPURenderBundleEncoderImpl(const CanvasGPURenderBundleEncoder *encoder);

    ~GPURenderBundleEncoderImpl() {
        canvas_native_webgpu_render_bundle_encoder_release(this->GetEncoder());
    }

    const CanvasGPURenderBundleEncoder *GetEncoder();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPURenderBundleEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPURenderBundleEncoderImpl *encoder) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPURenderBundleEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(encoder, NativeType::GPURenderBundleEncoder);
        object->SetAlignedPointerInInternalField(0, encoder);
        encoder->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void Draw(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexed(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Finish(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetIndexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetVertexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    const CanvasGPURenderBundleEncoder *encoder_;
};


#endif //CANVAS_ANDROID_GPURENDERBUNDLEENCODERIMPL_H