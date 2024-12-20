//
// Created by Osei Fortune on 01/07/2024.
//

#ifndef CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H
#define CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H

#include "Common.h"
#include "Helpers.h"
#include "ObjectWrapperImpl.h"
#include "GPUUtils.h"

class GPURenderPassEncoderImpl : ObjectWrapperImpl {
public:
    explicit GPURenderPassEncoderImpl(const CanvasGPURenderPassEncoder *pass);

    ~GPURenderPassEncoderImpl() {
        canvas_native_webgpu_render_pass_encoder_release(this->GetPass());
    }

    const CanvasGPURenderPassEncoder *GetPass();

    static void Init(v8::Local<v8::Object> canvasModule, v8::Isolate *isolate);

    static GPURenderPassEncoderImpl *GetPointer(const v8::Local<v8::Object> &object);

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate);

    static v8::Local<v8::Object>
    NewInstance(v8::Isolate *isolate, GPURenderPassEncoderImpl *encoder) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = GPURenderPassEncoderImpl::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(encoder, NativeType::GPURenderPassEncoder);
        object->SetAlignedPointerInInternalField(0, encoder);
        encoder->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static void GetLabel(v8::Local<v8::Name> name,
                         const v8::PropertyCallbackInfo<v8::Value> &info);

    static void BeginOcclusionQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void Draw(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexed(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void DrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MultiDrawIndexedIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void MultiDrawIndirect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void End(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void EndOcclusionQuery(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void ExecuteBundles(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void InsertDebugMarker(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PopDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void PushDebugGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBindGroup(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetBlendConstant(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetIndexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetPipeline(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetScissorRect(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetStencilReference(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetVertexBuffer(const v8::FunctionCallbackInfo<v8::Value> &args);

    static void SetViewport(const v8::FunctionCallbackInfo<v8::Value> &args);


private:
    const CanvasGPURenderPassEncoder *pass_;
};


#endif //CANVAS_ANDROID_GPURENDERPASSENCODERIMPL_H
