# Install script for directory: /Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/usr/local")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/build/libbncsutil.1.4.1.dylib"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/build/libbncsutil.1.dylib"
    )
  foreach(file
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libbncsutil.1.4.1.dylib"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libbncsutil.1.dylib"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      execute_process(COMMAND "/usr/bin/install_name_tool"
        -id "libbncsutil.1.dylib"
        "${file}")
      if(CMAKE_INSTALL_DO_STRIP)
        execute_process(COMMAND "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/strip" "${file}")
      endif()
    endif()
  endforeach()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/build/libbncsutil.dylib")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libbncsutil.dylib" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libbncsutil.dylib")
    execute_process(COMMAND "/usr/bin/install_name_tool"
      -id "libbncsutil.1.dylib"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libbncsutil.dylib")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libbncsutil.dylib")
    endif()
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/bncsutil" TYPE FILE FILES
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/bncsutil.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/bsha1.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/buffer.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/cdkeydecoder.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/checkrevision.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/decodekey.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/file.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/keytables.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/libinfo.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/ms_stdint.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/mutil.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/mutil_types.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/nls.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/oldauth.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/pe.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/sha1.h"
    "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/src/bncsutil/stack.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/jilizart/Projects/Github/bncsutil-node/native/bncsutil/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
