//
// Created by raytran on 2/15/10.
//

#include <boost/serialization/unordered_set.hpp>
#include "piecerules.h"

namespace protochess_engine {
    namespace piecerules {
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
                              std::unordered_set<Location>({
                                                                   {0, -1},
                                                                   {0, -2}
                                                           })
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
                                      })
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
                                      })
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
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
                                                                   {0, 1},
                                                                   {0, 2}
                                                           })
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
                                      })
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
                                      })
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
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
                                                           })
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
                                      })
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
                                      })
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
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
                                                           })
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
                                      })
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
                                      })
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
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
                              std::unordered_set<Location>()
                      }
                }
        };
    }
}
