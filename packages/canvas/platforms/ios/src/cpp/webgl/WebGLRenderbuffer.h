//
// Created by Osei Fortune on 27/04/2022.
//

#pragma once

#include "Helpers.h"

class WebGLRenderbuffer {
public:
    WebGLRenderbuffer(uint32_t renderbuffer) : renderbuffer_(renderbuffer) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLRenderbufferTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLRenderbuffer"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        cache->WebGLRenderbufferTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLRenderbuffer *buffer) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLRenderbuffer::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WebGLRenderbuffer);
        auto ext = v8::External::New(isolate, buffer);
        object->SetInternalField(0, ext);
        return scope.Escape(object);
    }

    static WebGLRenderbuffer *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLRenderbuffer *>(ptr);
    }

    uint32_t GetRenderBuffer() {
        return this->renderbuffer_;
    }

private:
    uint32_t renderbuffer_;
};
