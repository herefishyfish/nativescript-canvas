//
// Created by Osei Fortune on 29/04/2022.
//

#include "WEBGL_draw_buffersImpl.h"

WEBGL_draw_buffersImpl::WEBGL_draw_buffersImpl(WEBGL_draw_buffers *buffers) : buffers_(buffers) {

}

v8::CFunction WEBGL_draw_buffersImpl::fast_draw_buffers_webgl(
        v8::CFunction::Make(WEBGL_draw_buffersImpl::FastDrawBuffersWEBGL));

void WEBGL_draw_buffersImpl::DrawBuffersWEBGL(
        const v8::FunctionCallbackInfo<v8::Value> &args) {
    WEBGL_draw_buffersImpl *ptr = GetPointer(args.This());
    if (ptr == nullptr) {
        return;
    }
    auto isolate = args.GetIsolate();
    auto context = isolate->GetCurrentContext();

    auto value = args[0];
    if (value->IsArray()) {
        auto buffers = value.As<v8::Array>();
        auto len = buffers->Length();
        std::vector<uint32_t> buf;
        buf.reserve(len);
        for (int j = 0; j < len; ++j) {
            auto item = buffers->Get(context, j).ToLocalChecked();
            buf.push_back(item->Uint32Value(context).ToChecked());
        }
        canvas_native_webgl_draw_buffers_draw_buffers_webgl(buf.data(), buf.size(),
                                                            ptr->GetDrawBuffers());

    }

}
