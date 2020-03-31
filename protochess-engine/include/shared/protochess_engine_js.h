#pragma once

#include <napi.h>
#include "protochess_engine.h"

/*
 * JS Bindings for the engine
 */
namespace protochess_engine {
    class ProtochessEngineJS : public Napi::ObjectWrap<ProtochessEngineJS> {
    private:
        static Napi::FunctionReference constructor;

        Napi::Value Perft(const Napi::CallbackInfo &info);

        Napi::Value TakeTurn(const Napi::CallbackInfo &info);

        Napi::Value ToString(const Napi::CallbackInfo &info);

        Napi::Value GetState(const Napi::CallbackInfo &info);

        protochess_engine::ProtochessEngine *chess;
    public:
        static Napi::Object Init(Napi::Env env, Napi::Object exports);

        ProtochessEngineJS(const Napi::CallbackInfo &info);

    };
}
