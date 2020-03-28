#include <napi.h>
#include "include/shared/chess.h"
#include "protochess_engine.h"

Napi::FunctionReference ProtochessEngine::constructor;

Napi::Object ProtochessEngine::Init(Napi::Env env, Napi::Object exports) {
    Napi::HandleScope scope(env);

    Napi::Function func =
            DefineClass(env,
                        "ProtochessEngine",
                        {
                                InstanceMethod("takeTurn", &ProtochessEngine::TakeTurn),
                                InstanceMethod("toString", &ProtochessEngine::ToString)
                        });

    constructor = Napi::Persistent(func);
    constructor.SuppressDestruct();

    exports.Set("ProtochessEngine", func);
    return exports;
}

ProtochessEngine::ProtochessEngine(const Napi::CallbackInfo &info)
        : Napi::ObjectWrap<ProtochessEngine>(info) {
    Napi::Env env = info.Env();
    Napi::HandleScope scope(env);

    int length = info.Length();

    if (!(length == 0 || length == 2)) {
        Napi::TypeError::New(env, "Wrong argument length.").ThrowAsJavaScriptException();
        return;
    }

    if (length == 2 && (!info[0].IsNumber() || !info[1].IsNumber())) {
        Napi::TypeError::New(env, "Number expected").ThrowAsJavaScriptException();
        return;
    }
    this->chess = new protochess_engine::Chess();
    this->chess->buildClassicSet();
}

Napi::Value ProtochessEngine::TakeTurn(const Napi::CallbackInfo &info) {

    Napi::Env env = info.Env();
    Napi::HandleScope scope(env);
    if (!(info.Length() == 5 || info.Length() == 2)) {
        Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
    }

    Napi::Object obj = Napi::Object::New(env);
    //Four arg version
    if (info.Length() == 5) {
        if (!info[0].IsNumber() || !info[1].IsNumber() || !info[2].IsNumber() || !info[3].IsNumber()) {
            Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
        }
        Napi::Number x1 = info[0].As<Napi::Number>();
        Napi::Number y1 = info[1].As<Napi::Number>();
        Napi::Number x2 = info[2].As<Napi::Number>();
        Napi::Number y2 = info[3].As<Napi::Number>();
        Napi::Number whosTurn = info[4].As<Napi::Number>();
        protochess_engine::TurnResult turnResult = this->chess->takeTurn(x1.Int32Value(), y1.Int32Value(),
                                                                         x2.Int32Value(), y2.Int32Value(),
                                                                         whosTurn.Int32Value());
        obj.Set("success", turnResult.successful);
        obj.Set("nextToMove", turnResult.nextToMove);
    }

    return obj;
}

Napi::Value ProtochessEngine::ToString(const Napi::CallbackInfo &info) {
    Napi::Env env = info.Env();
    Napi::HandleScope scope(env);
    if (info.Length() != 0) {
        Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
    }
    return Napi::String::New(env, this->chess->toString());
}

// Initialize native add-on
Napi::Object Init(Napi::Env env, Napi::Object exports) {
    ProtochessEngine::Init(env, exports);
    return exports;
}

NODE_API_MODULE(NODE_GYP_MODULE_NAME, Init
)
