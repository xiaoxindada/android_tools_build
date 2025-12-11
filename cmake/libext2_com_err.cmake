set(target_name "ext2_com_err")

set(libext2_com_err_srcs_dir "${CMAKE_SOURCE_DIR}/src/e2fsprogs/lib/et")

set(e2fsprogs_cflags
        "-Wall"
        # Some warnings that Android's build system enables by default are not
        # supported by upstream e2fsprogs.  When such a warning shows up
        # disable it below.  Please don't disable warnings that upstream
        # e2fsprogs is supposed to support; for those fix the code instead.
        "-Wno-pointer-arith"
        "-Wno-sign-compare"
        "-Wno-type-limits"
        "-Wno-typedef-redefinition"
        "-Wno-unused-parameter"
        "-Wno-unused-but-set-variable"
        "-Wno-macro-redefined"
        "-Wno-sign-compare" #Better keep compare
)

set(libext2_com_err_srcs
        "${libext2_com_err_srcs_dir}/error_message.c"
        "${libext2_com_err_srcs_dir}/et_name.c"
        "${libext2_com_err_srcs_dir}/init_et.c"
        "${libext2_com_err_srcs_dir}/com_err.c"
        "${libext2_com_err_srcs_dir}/com_right.c"
)

add_library(${target_name} STATIC ${libext2_com_err_srcs})
target_compile_options(${target_name} PUBLIC ${e2fsprogs_cflags})
target_include_directories(${target_name} PUBLIC
    ${e2fsprogs_lib_headers}
    ${libext2_com_err_srcs_dir}
)