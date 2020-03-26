cmake_minimum_required(VERSION 3.5)

project(protochess_engine)

############################################################
# Create a library
############################################################

#Generate the shared library from the library sources
add_library(protochess_engine SHARED
        src/chess.cpp
        src/types.h src/bitsetUtil.cpp src/bitsetUtil.h src/player.cpp src/player.h src/movegen.cpp src/movegen.h src/Board.cpp src/Board.h src/piecerules.cpp src/piecerules.h src/Piece.cpp src/Piece.h)

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
