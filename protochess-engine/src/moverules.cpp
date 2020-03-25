//
// Created by raytran on 3/25/20.
//

#include <boost/serialization/vector.hpp>
#include "moverules.h"

namespace moverules {
    std::map<char, MovementPattern> rules = {
            {'p', {false,
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
                                          {3,  2},
                                          {3,  -2},
                                          {-3, 2},
                                          {-3, -2},
                                          {2,  3},
                                          {2,  -3},
                                          {-2, 3},
                                          {-2, -3}
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
                                          {1,  1},
                                          {1,  -1},
                                          {-1, 1},
                                          {-1, -1},
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
            }
    };
}
