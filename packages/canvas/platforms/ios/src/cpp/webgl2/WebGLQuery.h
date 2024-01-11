//
// Created by Osei Fortune on 30/04/2022.
//

#pragma once

#include "Caches.h"
#include "ObjectWrapperImpl.h"

class WebGLQuery: ObjectWrapperImpl {
public:
    WebGLQuery(uint32_t query) : query_(query) {}

    static v8::Local<v8::FunctionTemplate> GetCtor(v8::Isolate *isolate) {
        auto cache = Caches::Get(isolate);
        auto ctor = cache->WebGLQueryTmpl.get();
        if (ctor != nullptr) {
            return ctor->Get(isolate);
        }

        v8::Local<v8::FunctionTemplate> ctorTmpl = v8::FunctionTemplate::New(isolate);
        ctorTmpl->InstanceTemplate()->SetInternalFieldCount(1);
        ctorTmpl->SetClassName(ConvertToV8String(isolate, "WebGLQuery"));

        auto tmpl = ctorTmpl->InstanceTemplate();
        tmpl->SetInternalFieldCount(1);

        cache->WebGLQueryTmpl =
                std::make_unique<v8::Persistent<v8::FunctionTemplate>>(isolate, ctorTmpl);
        return ctorTmpl;
    }

    static v8::Local<v8::Object> NewInstance(v8::Isolate *isolate, WebGLQuery *query) {
        auto context = isolate->GetCurrentContext();
        v8::EscapableHandleScope scope(isolate);
        auto object = WebGLQuery::GetCtor(isolate)->GetFunction(
                context).ToLocalChecked()->NewInstance(context).ToLocalChecked();
        SetNativeType(isolate, object, NativeType::WebGLQuery);
        auto ext = v8::External::New(isolate, query);
        object->SetInternalField(0, ext);
        query->BindFinalizer(isolate, object);
        return scope.Escape(object);
    }

    static WebGLQuery *GetPointer(const v8::Local<v8::Object> &object) {
        auto ptr = object->GetInternalField(0).As<v8::External>()->Value();
        if (ptr == nullptr) {
            return nullptr;
        }
        return static_cast<WebGLQuery *>(ptr);
    }

    uint32_t GetQuery() {
        return this->query_;
    }

private:
    uint32_t query_;
};
