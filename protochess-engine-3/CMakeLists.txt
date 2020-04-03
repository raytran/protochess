cmake_minimum_required(VERSION 3.17)
project(protochess_engine_3)

set(CMAKE_CXX_STANDARD 20)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif ()

set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

#set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O0")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O0")

#Requires boost
find_package(Boost 1.36.0 REQUIRED)
include_directories(${Boost_INCLUDE_DIRS})

add_library(protochess_engine_3 SHARED src/protochess_engine.cpp include/protochess_engine.h src/types.h src/position.cpp src/position.h src/bitboardutil.cpp src/bitboardutil.h src/movegen.cpp src/movegen.h src/attacktables.cpp src/attacktables.h src/piecerules.cpp src/piecerules.h src/rankfile.h)


# Add an executable with the above sources
add_executable(main_binary
        main.cpp
        main.cpp)

# link the new protochess_engine target with the hello_binary target
target_link_libraries(main_binary
        PRIVATE
        protochess_engine_3
        )