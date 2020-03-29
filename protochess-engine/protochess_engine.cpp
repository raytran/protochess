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
                                InstanceMethod("toString", &ProtochessEngine::ToString),
                                InstanceMethod("toBoardString", &ProtochessEngine::ToBoardString),
                                InstanceMethod("toPieceString", &ProtochessEngine::ToPieceString)
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
    if (!(info.Length() == 5 || info.Length() == 3)) {
        Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
    }

    Napi::Object obj = Napi::Object::New(env);
    protochess_engine::TurnResult turnResult;
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

        turnResult = this->chess->takeTurn(x1.Int32Value(), y1.Int32Value(),
                                           x2.Int32Value(), y2.Int32Value(),
                                           whosTurn.Int32Value());

    } else {
        //Three arg version
        if (!info[0].IsString() || !info[1].IsString() || !info[2].IsNumber()) {
            Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
        }

        Napi::String from = info[0].As<Napi::String>();
        Napi::String to = info[1].As<Napi::String>();
        Napi::Number whosTurn = info[2].As<Napi::Number>();
        turnResult = this->chess->takeTurn(from, to, whosTurn);
    }

    //Process turn
    std::unordered_set<int> &playersInCheck = turnResult.checkedPlayers;
    std::unordered_set<int> &checkMatedPlayers = turnResult.checkmatedPlayers;
    Napi::Array PlayersInCheck = Napi::Array::New(env, playersInCheck.size());
    Napi::Array CheckMatedPlayers = Napi::Array::New(env, checkMatedPlayers.size());

    int index = 0;
    for (auto &p : playersInCheck) {
        PlayersInCheck[index] = p;
        index++;
    }

    index = 0;
    for (auto &p : checkMatedPlayers) {
        CheckMatedPlayers[index] = p;
        index++;
    }

    obj.Set("success", turnResult.successful);
    obj.Set("playersInCheck", PlayersInCheck);
    obj.Set("checkmatedPlayers", CheckMatedPlayers);
    obj.Set("nextToMove", turnResult.nextToMove);

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

Napi::Value ProtochessEngine::ToPieceString(const Napi::CallbackInfo &info) {
    Napi::Env env = info.Env();
    Napi::HandleScope scope(env);
    if (info.Length() != 0) {
        Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
    }
    return Napi::String::New(env, this->chess->toPlayerPieceString());
}

Napi::Value ProtochessEngine::ToBoardString(const Napi::CallbackInfo &info) {
    Napi::Env env = info.Env();
    Napi::HandleScope scope(env);
    if (info.Length() != 0) {
        Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
    }
    return Napi::String::New(env, this->chess->toBoardString());
}


// Initialize native add-on
Napi::Object Init(Napi::Env env, Napi::Object exports) {
    ProtochessEngine::Init(env, exports);
    return exports;
}

NODE_API_MODULE(NODE_GYP_MODULE_NAME, Init
)
