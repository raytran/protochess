{
  "targets": [
    {
      "target_name": "protochess_engine",
      "cflags!": [ "-fno-exceptions" ],
      "cflags_cc!": [ "-fno-exceptions" ],
      "sources": [
      "protochess_engine.cpp",
        "src/chess.cpp",
        "src/types.h",
         "src/bitsetutil.cpp",
          "src/bitsetutil.h",
           "src/player.cpp",
            "src/player.h",
             "src/movegen.cpp",
              "src/movegen.h",
               "src/board.cpp",
                "src/board.h",
                 "src/piecerules.cpp",
                  "src/piecerules.h",
                   "src/piece.cpp",
                    "src/piece.h",
                     "src/gamestate.cpp",
                      "src/gamestate.h",
                       "src/rankfile.h"


       ],
      "include_dirs": [
        "<!@(node -p \"require('node-addon-api').include\")"
      ],

      'cflags!': [ '-fno-exceptions' ],
      'cflags_cc!': [ '-fno-exceptions' ],
      'xcode_settings': {
        'GCC_ENABLE_CPP_EXCEPTIONS': 'YES',
        'CLANG_CXX_LIBRARY': 'libc++',
        'MACOSX_DEPLOYMENT_TARGET': '10.7',
      },
      'msvs_settings': {
        'VCCLCompilerTool': { 'ExceptionHandling': 1 },
      },
    }
  ]
}
