#pragma once

#include <napi.h>
#include "include/shared/chess.h"

class ProtochessEngine : public Napi::ObjectWrap<ProtochessEngine> {
private:
    static Napi::FunctionReference constructor;

    Napi::Value TakeTurn(const Napi::CallbackInfo &info);

    Napi::Value ToString(const Napi::CallbackInfo &info);

    Napi::Value ToPieceString(const Napi::CallbackInfo &info);

    Napi::Value ToBoardString(const Napi::CallbackInfo &info);

    double value_;

    protochess_engine::Chess *chess;
public:
    static Napi::Object Init(Napi::Env env, Napi::Object exports);

    ProtochessEngine(const Napi::CallbackInfo &info);

};
