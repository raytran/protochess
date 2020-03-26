//
// Created by raytran on 2/15/10.
//

#include <boost/serialization/vector.hpp>
#include "moverules.h"

namespace moverules {
    std::map<char, MovementPattern> rules = {
            //BLACK PIECES
            {'p', {false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          false,
                          std::vector<Location>({{0, -1}})
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
                          std::vector<Location>(
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
                          std::vector<Location>(
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
                          std::vector<Location>({{0, 1}})
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
                          std::vector<Location>(
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
                          std::vector<Location>(
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
                          std::vector<Location>()
                  }
            }
    };
}
