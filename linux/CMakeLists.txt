# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "bbqr")
project(${PROJECT_NAME} LANGUAGES CXX)

# Rust is built and bundled by the Dart Native Assets hook.
set(bbqr_bundled_libraries "" PARENT_SCOPE)
