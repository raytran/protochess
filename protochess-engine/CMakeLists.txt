cmake_minimum_required(VERSION 3.5)
project(protochess_engine_js)

if (NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif ()

set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

#Requires boost
find_package(Boost 1.36.0 REQUIRED)
include_directories(${Boost_INCLUDE_DIRS})

# Build a shared library named after the project from the files in `src/`
file(GLOB SOURCE_FILES "include/shared/*.h" "src/*.cpp" "src/*.h")
add_library(${PROJECT_NAME} SHARED ${SOURCE_FILES})

# Gives our library file a .node extension without any "lib" prefix
set_target_properties(${PROJECT_NAME} PROPERTIES PREFIX "" SUFFIX ".node")

# Essential include files to build a node addon,
# You should add this line in every CMake.js based project
target_include_directories(${PROJECT_NAME} PRIVATE ${CMAKE_JS_INC})

# Essential library files to link to a node addon
# You should add this line in every CMake.js based project
target_link_libraries(${PROJECT_NAME} ${CMAKE_JS_LIB})

# Include N-API wrappers
execute_process(COMMAND node -p "require('node-addon-api').include"
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE NODE_ADDON_API_DIR
        )
string(REPLACE "\n" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})
string(REPLACE "\"" "" NODE_ADDON_API_DIR ${NODE_ADDON_API_DIR})
target_include_directories(${PROJECT_NAME} PRIVATE ${NODE_ADDON_API_DIR})

#[[

############################################################
# Create a library
############################################################

#Generate the shared library from the library sources
add_library(protochess_engine SHARED
        src/chess.cpp
        src/types.h src/bitsetutil.cpp src/bitsetutil.h src/player.cpp src/player.h src/movegen.cpp src/movegen.h src/board.cpp src/board.h src/piecerules.cpp src/piecerules.h src/piece.cpp src/piece.h src/gamestate.cpp src/gamestate.h src/rankfile.h)

add_library(protochess::engine ALIAS protochess_engine)

find_package(Boost 1.36.0 REQUIRED)
include_directories(${Boost_INCLUDE_DIRS})

target_include_directories(protochess_engine
        PUBLIC
        ${PROJECT_SOURCE_DIR}/include
        )

############################################################
# Create an executable
############################################################

# Add an executable with the above sources
add_executable(main_binary
        main.cpp
        )

# link the new protochess_engine target with the hello_binary target
target_link_libraries(main_binary
        PRIVATE
        protochess::engine
        )

 ]]
