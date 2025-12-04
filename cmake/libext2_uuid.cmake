set(target_name "ext2_uuid")

set(libext2_uuid_dir
    "${CMAKE_SOURCE_DIR}/src/e2fsprogs/lib/uuid"
)

set(e2fsprogs_cflags
        "-Wall"
        # Some warnings that Android's build system enables by default are not
        # supported by upstream e2fsprogs.  When such a warning shows up,
        # disable it below.  Please don't disable warnings that upstream
        # e2fsprogs is supposed to support; for those, fix the code instead.
        "-Wno-pointer-arith"
        "-Wno-sign-compare"
        "-Wno-type-limits"
        "-Wno-typedef-redefinition"
        "-Wno-unused-parameter"
        "-Wno-unused-but-set-variable"
        "-Wno-macro-redefined"
        "-Wno-sign-compare" #Better keep compare
)

set(libext2_uuid_srcs
        "${libext2_uuid_dir}/clear.c"
        "${libext2_uuid_dir}/compare.c"
        "${libext2_uuid_dir}/copy.c"
        "${libext2_uuid_dir}/gen_uuid.c"
        "${libext2_uuid_dir}/isnull.c"
        "${libext2_uuid_dir}/pack.c"
        "${libext2_uuid_dir}/parse.c"
        "${libext2_uuid_dir}/unpack.c"
        "${libext2_uuid_dir}/unparse.c"
        "${libext2_uuid_dir}/uuid_time.c"
)

list(APPEND libext2_headers "${libext2_uuid_dir}")

add_library(${target_name} STATIC ${libext2_uuid_srcs})
target_compile_options(${target_name} PRIVATE ${e2fsprogs_cflags})

target_include_directories(${target_name} PUBLIC
    ${e2fsprogs_lib_headers}
    ${libext2_uuid_dir}
)