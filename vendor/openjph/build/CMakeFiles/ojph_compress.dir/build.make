# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.18

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:


#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:


# Disable VCS-based implicit rules.
% : %,v


# Disable VCS-based implicit rules.
% : RCS/%


# Disable VCS-based implicit rules.
% : RCS/%,v


# Disable VCS-based implicit rules.
% : SCCS/s.%


# Disable VCS-based implicit rules.
% : s.%


.SUFFIXES: .hpux_make_needs_suffix_list


# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:

.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/local/Cellar/cmake/3.18.2/bin/cmake

# The command to remove a file.
RM = /usr/local/Cellar/cmake/3.18.2/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /Users/nik/lexray/openjph-ffi/vendor/openjph

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /Users/nik/lexray/openjph-ffi/vendor/openjph/build

# Include any dependencies generated for this target.
include CMakeFiles/ojph_compress.dir/depend.make

# Include the progress variables for this target.
include CMakeFiles/ojph_compress.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/ojph_compress.dir/flags.make

CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.o: CMakeFiles/ojph_compress.dir/flags.make
CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.o: ../src/apps/ojph_compress/ojph_compress.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/nik/lexray/openjph-ffi/vendor/openjph/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.o -c /Users/nik/lexray/openjph-ffi/vendor/openjph/src/apps/ojph_compress/ojph_compress.cpp

CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/nik/lexray/openjph-ffi/vendor/openjph/src/apps/ojph_compress/ojph_compress.cpp > CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.i

CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/nik/lexray/openjph-ffi/vendor/openjph/src/apps/ojph_compress/ojph_compress.cpp -o CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.s

CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.o: CMakeFiles/ojph_compress.dir/flags.make
CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.o: ../src/apps/others/ojph_img_io.cpp
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/Users/nik/lexray/openjph-ffi/vendor/openjph/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.o"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -o CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.o -c /Users/nik/lexray/openjph-ffi/vendor/openjph/src/apps/others/ojph_img_io.cpp

CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.i"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /Users/nik/lexray/openjph-ffi/vendor/openjph/src/apps/others/ojph_img_io.cpp > CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.i

CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.s"
	/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /Users/nik/lexray/openjph-ffi/vendor/openjph/src/apps/others/ojph_img_io.cpp -o CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.s

# Object files for target ojph_compress
ojph_compress_OBJECTS = \
"CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.o" \
"CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.o"

# External object files for target ojph_compress
ojph_compress_EXTERNAL_OBJECTS =

../bin/ojph_compress: CMakeFiles/ojph_compress.dir/src/apps/ojph_compress/ojph_compress.cpp.o
../bin/ojph_compress: CMakeFiles/ojph_compress.dir/src/apps/others/ojph_img_io.cpp.o
../bin/ojph_compress: CMakeFiles/ojph_compress.dir/build.make
../bin/ojph_compress: ../bin/libopenjph.dylib
../bin/ojph_compress: CMakeFiles/ojph_compress.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/Users/nik/lexray/openjph-ffi/vendor/openjph/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Linking CXX executable ../bin/ojph_compress"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/ojph_compress.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/ojph_compress.dir/build: ../bin/ojph_compress

.PHONY : CMakeFiles/ojph_compress.dir/build

CMakeFiles/ojph_compress.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/ojph_compress.dir/cmake_clean.cmake
.PHONY : CMakeFiles/ojph_compress.dir/clean

CMakeFiles/ojph_compress.dir/depend:
	cd /Users/nik/lexray/openjph-ffi/vendor/openjph/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /Users/nik/lexray/openjph-ffi/vendor/openjph /Users/nik/lexray/openjph-ffi/vendor/openjph /Users/nik/lexray/openjph-ffi/vendor/openjph/build /Users/nik/lexray/openjph-ffi/vendor/openjph/build /Users/nik/lexray/openjph-ffi/vendor/openjph/build/CMakeFiles/ojph_compress.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : CMakeFiles/ojph_compress.dir/depend

