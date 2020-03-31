#include <napi.h>
#include <boost/lexical_cast.hpp>
#include "../include/shared/protochess_engine.h"
#include "../include/shared/protochess_engine_js.h"

#include <boost/dynamic_bitset.hpp>
#include <iostream>

namespace protochess_engine {
    Napi::FunctionReference ProtochessEngineJS::constructor;

    Napi::Object ProtochessEngineJS::Init(Napi::Env env, Napi::Object exports) {
        Napi::HandleScope scope(env);

        Napi::Function func =
                DefineClass(env,
                            "ProtochessEngineJS",
                            {
                                    InstanceMethod("takeTurn", &ProtochessEngineJS::TakeTurn),
                                    InstanceMethod("toString", &ProtochessEngineJS::ToString),
                                    InstanceMethod("getState", &ProtochessEngineJS::GetState),
                                    InstanceMethod("perft", &ProtochessEngineJS::Perft)
                            });

        constructor = Napi::Persistent(func);
        constructor.SuppressDestruct();

        exports.Set("ProtochessEngineJS", func);
        return exports;
    }

    ProtochessEngineJS::ProtochessEngineJS(const Napi::CallbackInfo &info)
            : Napi::ObjectWrap<ProtochessEngineJS>(info) {
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
        this->chess = new protochess_engine::ProtochessEngine();
        this->chess->buildClassicSet();
    }

    Napi::Value ProtochessEngineJS::TakeTurn(const Napi::CallbackInfo &info) {

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

    Napi::Value ProtochessEngineJS::ToString(const Napi::CallbackInfo &info) {
        Napi::Env env = info.Env();
        Napi::HandleScope scope(env);
        if (info.Length() != 0) {
            Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
        }
        return Napi::String::New(env, this->chess->toString());
    }

    Napi::Value ProtochessEngineJS::GetState(const Napi::CallbackInfo &info) {
        Napi::Env env = info.Env();
        Napi::HandleScope scope(env);
        if (info.Length() != 0) {
            Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
        }
        Napi::Object obj = Napi::Object::New(env);
        Napi::Object playersObj = Napi::Object::New(env);
        for (auto &x:this->chess->gameState.getPlayerMap()) {
            Napi::Object playerObj = Napi::Object::New(env);
            for (auto &y:x.second.getPieces()) {
                Napi::Object pieceObj = Napi::Object::New(env);
                pieceObj.Set("charRep", std::string(1, y.second->getCharRep()));
                pieceObj.Set("x", uint32_t(y.second->getLocation().x));
                pieceObj.Set("y", uint32_t(y.second->getLocation().y));
                playerObj.Set(y.second->getIdString(), pieceObj);
            }
            playersObj.Set(uint32_t(x.first), playerObj);
        }

        obj.Set("players", playersObj);


        Napi::Object board = Napi::Object::New(env);

        board.Set("width", this->chess->gameState.getBoard().getWidth());
        board.Set("height", this->chess->gameState.getBoard().getHeight());

        std::vector<Tile> tiles = this->chess->gameState.getBoard().getTiles();
        Napi::Object Tiles = Napi::Array::New(env, tiles.size());
        for (int i = 0; i < tiles.size(); i++) {
            Napi::Object tile = Napi::Object::New(env);
            Tile x = tiles[i];
            tile.Set("x", uint32_t(x.location.x));
            tile.Set("y", uint32_t(x.location.y));
            tile.Set("charRep", std::string(1, x.charRep));
            Tiles[i] = tile;
        }
        board.Set("tiles", Tiles);


        obj.Set("board", board);
        return obj;
    }

    Napi::Value ProtochessEngineJS::Perft(const Napi::CallbackInfo &info) {
        Napi::Env env = info.Env();
        Napi::HandleScope scope(env);
        if (info.Length() != 1 || !info[0].IsNumber()) {
            Napi::TypeError::New(env, "Invalid args.").ThrowAsJavaScriptException();
        }
        return Napi::Number::New(env, this->chess->perft(info[0].As<Napi::Number>()));
    }


    // Initialize native add-on
    Napi::Object Init(Napi::Env env, Napi::Object exports) {
        ProtochessEngineJS::Init(env, exports);
        return exports;
    }

    NODE_API_MODULE(NODE_GYP_MODULE_NAME, Init)
}
