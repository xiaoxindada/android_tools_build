set(target_name "squashfs_utils")

set(libsquashfs_utils_dir "${CMAKE_SOURCE_DIR}/src/extras/squashfs_utils")

set(cflags
    "-Wall"
)

set(libsquashfs_utils_srcs
    "${libsquashfs_utils_dir}/squashfs_utils.c"
)

if(NOT CMAKE_SYSTEM_NAME STREQUAL "Android")
    list(APPEND cflags 
        "-D_GNU_SOURCE"
        "-DSQUASHFS_NO_KLOG"
    )
endif()

add_library(${target_name} STATIC ${libsquashfs_utils_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libcutils_headers}
    ${squashfs_headers}
    ${libsquashfs_utils_headers}
)
target_link_libraries(${target_name} PUBLIC 
    cutils
)