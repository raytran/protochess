//
// Created by raytran on 4/5/20.
//

#pragma once

#include "types.h"

namespace protochess_engine {
    class Rulebook {
    private:
        std::map<char, MovementPattern> customTranslateRules;
        std::map<char, MovementPattern> customCaptureRules;
    public:
        void setCustomTranslateRule(char c, MovementPattern mp);

        void deleteCustomTranslateRule(char c);

        void setCustomCaptureRule(char c, MovementPattern mp);

        void deleteCustomCaptureRule(char c);

        [[nodiscard]] const MovementPattern *getTranslatePattern(char c) const;

        [[nodiscard]] const MovementPattern *getCapturePattern(char c) const;

    };
}
