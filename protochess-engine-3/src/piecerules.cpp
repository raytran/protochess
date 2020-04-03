#include "piecerules.h"
//
// Created by raytran on 2/15/10.
//

#include "piecerules.h"

namespace protochess_engine::piecerules {
    std::map<char, MovementPattern> moveRules = {
            //BLACK PIECES
            {'p', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(),
                          std::vector<Location>({
                                                        {0, -1},
                                                        {0, -2}
                                                })
                  }
            },
            //black pawn that moved
            {',', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>({
                                                               {0, -1}
                                                       }),
                          std::vector<Location>()
                  }
            },

            {'n', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {2,  1},
                                          {2,  -1},
                                          {-2, 1},
                                          {-2, -1},
                                          {1,  2},
                                          {1,  -2},
                                          {-1, 2},
                                          {-1, -2}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'k', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {0,  1},
                                          {0,  -1},
                                          {-1, 0},
                                          {1,  0},
                                          {1,  1},
                                          {1,  -1},
                                          {-1, 1},
                                          {-1, -1}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'b', {false,
                          false,
                          false,
                          false,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'q', {true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'r', {true,
                          true,
                          true,
                          true,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },
            //WHITE PIECES
            {'P', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(),
                          std::vector<Location>({
                                                        {0, 1},
                                                        {0, 2}
                                                })
                  }
            },


            //White pawn that moved
            {'.', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>({
                                                               {0, 1}
                                                       }),
                          std::vector<Location>()
                  }
            },

            {'N', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {2,  1},
                                          {2,  -1},
                                          {-2, 1},
                                          {-2, -1},
                                          {1,  2},
                                          {1,  -2},
                                          {-1, 2},
                                          {-1, -2}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'K', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {0,  1},
                                          {0,  -1},
                                          {-1, 0},
                                          {1,  0},
                                          {1,  1},
                                          {1,  -1},
                                          {-1, 1},
                                          {-1, -1}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'B', {false,
                          false,
                          false,
                          false,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'Q', {true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'R', {true,
                          true,
                          true,
                          true,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            }
    };


    //Defines how standard pieces capture enemies
    std::map<char, MovementPattern> captureRules = {
            //BLACK PIECES
            {'p', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>({
                                                               {-1, -1},
                                                               {1,  -1}
                                                       }),
                          std::vector<Location>()
                  }
            },

            {'n', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {2,  1},
                                          {2,  -1},
                                          {-2, 1},
                                          {-2, -1},
                                          {1,  2},
                                          {1,  -2},
                                          {-1, 2},
                                          {-1, -2}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'k', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {0,  1},
                                          {0,  -1},
                                          {-1, 0},
                                          {1,  0},
                                          {1,  1},
                                          {1,  -1},
                                          {-1, 1},
                                          {-1, -1}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'b', {false,
                          false,
                          false,
                          false,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'q', {true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'r', {true,
                          true,
                          true,
                          true,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },
            //WHITE PIECES
            {'P', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>({
                                                               {-1, 1},
                                                               {1,  1}
                                                       }),
                          std::vector<Location>()
                  }
            },

            {'N', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {2,  1},
                                          {2,  -1},
                                          {-2, 1},
                                          {-2, -1},
                                          {1,  2},
                                          {1,  -2},
                                          {-1, 2},
                                          {-1, -2}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'K', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(
                                  {
                                          {0,  1},
                                          {0,  -1},
                                          {-1, 0},
                                          {1,  0},
                                          {1,  1},
                                          {1,  -1},
                                          {-1, 1},
                                          {-1, -1}
                                  }),
                          std::vector<Location>()
                  }
            },

            {'B', {false,
                          false,
                          false,
                          false,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'Q', {true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          true,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            },

            {'R', {true,
                          true,
                          true,
                          true,
                          false,
                          false,
                          false,
                          false,
                          std::unordered_set<Location>(),
                          std::vector<Location>()
                  }
            }
    };
}
