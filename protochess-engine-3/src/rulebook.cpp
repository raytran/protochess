//
// Created by raytran on 4/5/20.
//

#include "rulebook.h"
#include "piecerules.h"

namespace protochess_engine {
    void Rulebook::setCustomTranslateRule(char c, protochess_engine::MovementPattern mp) {
        customTranslateRules.insert({c, mp});
    }

    void Rulebook::deleteCustomTranslateRule(char c) {
        customTranslateRules.erase(c);
    }

    void Rulebook::setCustomCaptureRule(char c, MovementPattern mp) {
        customCaptureRules.insert({c, mp});
    }

    void Rulebook::deleteCustomCaptureRule(char c) {
        customCaptureRules.erase(c);
    }

    const MovementPattern *Rulebook::getTranslatePattern(char c) const {
        //Check custom rules
        if (customTranslateRules.contains(c)) {
            return &customTranslateRules.at(c);
        } else if (piecerules::moveRules.contains(c)) {
            return &piecerules::moveRules.at(c);
        } else {
            std::cerr << "WARNING: no capture rule defined; using king-style";
            return &piecerules::moveRules.at('k');
        }
    }

    const MovementPattern *Rulebook::getCapturePattern(char c) const {
        //Check internal rules
        //Check custom rules
        if (customCaptureRules.contains(c)) {
            return &customCaptureRules.at(c);
        } else if (piecerules::captureRules.contains(c)) {
            return &piecerules::captureRules.at(c);
        } else {
            //Eventually check player-defined rules
            std::cerr << "WARNING: no capture rule defined; using king-style";
            return &piecerules::captureRules.at('k');
        }
    }
}
