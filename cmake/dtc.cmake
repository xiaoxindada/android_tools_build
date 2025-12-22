
if(CMAKE_SYSTEM_NAME STREQUAL "Android")
  message("This is Android")
endif()

set(dtc_srcs_dir "${CMAKE_SOURCE_DIR}/src/dtc")
set(libfdt_srcs_dir "${dtc_srcs_dir}/libfdt")
set(libufdt_srcs_dir "${CMAKE_SOURCE_DIR}/src/libufdt")
set(mkdtimg_srcs_dir "${libufdt_srcs_dir}/utils/src")

set(dtc_enc_srcs
  "${dtc_srcs_dir}/dtc-lexer.l"
  "${dtc_srcs_dir}/dtc-parser.y"
)

set(dtc_dec_srcs
  "${dtc_srcs_dir}/dtc-lexer.c"
  "${dtc_srcs_dir}/dtc-parser.c"
  "${dtc_srcs_dir}/dtc-parser.h"
)

add_custom_command(
    OUTPUT
      ${dtc_dec_srcs}
    COMMAND 
      flex -o dtc-lexer.c dtc-lexer.l &&
      bison -d -o dtc-parser.c dtc-parser.y
    DEPENDS 
      ${dtc_enc_srcs}
    WORKING_DIRECTORY 
      ${dtc_srcs_dir}
)
add_custom_target(gen_dtc_dec_srcs ALL DEPENDS ${dtc_dec_srcs})

# get dtc version from Android.bp
execute_process(
  COMMAND
    bash -c
    "echo $(sed -f ${dtc_srcs_dir}/METADATA_version.sed -n ${dtc_srcs_dir}/METADATA)"
  OUTPUT_VARIABLE dtc_version
  OUTPUT_STRIP_TRAILING_WHITESPACE
)
message("dtc version: ${dtc_version}")
set(VCS_TAG "${dtc_version}-Android-build")
configure_file("${dtc_srcs_dir}/version_gen.h.in" "${dtc_srcs_dir}/version_gen.h")

set(common_includes
    ${dtc_srcs_dir} 
    ${libfdt_srcs_dir}
)

set(dt_cflags 
  "-Wall"
  "-Wno-macro-redefined"
  "-Wno-missing-field-initializers"
  "-Wno-sign-compare" "-Wno-unused-parameter"
  "-DNO_YAML"
)

set(libfdt_srcs
    "${libfdt_srcs_dir}/fdt.c"
    "${libfdt_srcs_dir}/fdt_check.c"
    "${libfdt_srcs_dir}/fdt_ro.c"
    "${libfdt_srcs_dir}/fdt_wip.c"
    "${libfdt_srcs_dir}/fdt_sw.c"
    "${libfdt_srcs_dir}/fdt_rw.c"
    "${libfdt_srcs_dir}/fdt_strerror.c"
    "${libfdt_srcs_dir}/fdt_empty_tree.c"
    "${libfdt_srcs_dir}/fdt_addresses.c"
    "${libfdt_srcs_dir}/fdt_overlay.c"
    "${libfdt_srcs_dir}/acpi.c"
)

set(libufdt_sysdeps_srcs 
  "${libufdt_srcs_dir}/sysdeps/libufdt_sysdeps_posix.c"
)

set(dtc_srcs
    ${dtc_dec_srcs}
    "${dtc_srcs_dir}/checks.c"
    "${dtc_srcs_dir}/data.c"
    "${dtc_srcs_dir}/dtc.c"
    "${dtc_srcs_dir}/flattree.c"
    "${dtc_srcs_dir}/fstree.c"
    "${dtc_srcs_dir}/livetree.c"
    "${dtc_srcs_dir}/srcpos.c"
    "${dtc_srcs_dir}/treesource.c"
    "${dtc_srcs_dir}/util.c"
)

set(mkdtimg_srcs
    "${mkdtimg_srcs_dir}/mkdtimg.c" 
    "${mkdtimg_srcs_dir}/mkdtimg_cfg_create.c"
    "${mkdtimg_srcs_dir}/mkdtimg_core.c" 
    "${mkdtimg_srcs_dir}/mkdtimg_create.c"
    "${mkdtimg_srcs_dir}/mkdtimg_dump.c" 
    "${mkdtimg_srcs_dir}/dt_table.c"
)

set(fdtget_srcs 
  "${dtc_srcs_dir}/fdtget.c" 
  "${dtc_srcs_dir}/util.c"
)

set(fdtput_srcs 
  "${dtc_srcs_dir}/fdtput.c"
  "${dtc_srcs_dir}/util.c"
)

set(fdtdump_srcs 
  "${dtc_srcs_dir}/fdtdump.c" 
  "${dtc_srcs_dir}/util.c"
)

set(fdtoverlay_srcs 
  "${dtc_srcs_dir}/fdtoverlay.c" 
  "${dtc_srcs_dir}/util.c"
)

add_library(fdt STATIC ${libfdt_srcs})
target_compile_options(fdt PUBLIC ${dt_cflags})
target_include_directories(fdt PUBLIC ${common_includes})

add_library(ufdt_sysdeps STATIC ${libufdt_sysdeps_srcs})
target_compile_options(ufdt_sysdeps PUBLIC ${dt_cflags})
target_include_directories(ufdt_sysdeps PUBLIC
  "${libufdt_srcs_dir}/include"
  "${libufdt_srcs_dir}/sysdeps/include"
)

# bins
add_executable(dtc ${dtc_srcs})
add_executable(mkdtimg ${mkdtimg_srcs})
add_executable(fdtget ${fdtget_srcs})
add_executable(fdtput ${fdtput_srcs})
add_executable(fdtdump ${fdtdump_srcs})
add_executable(fdtoverlay ${fdtoverlay_srcs})

target_include_directories(mkdtimg PUBLIC ${mkdtimg_srcs_dir})

target_link_libraries(dtc PRIVATE fdt)
target_link_libraries(fdtget PRIVATE fdt)
target_link_libraries(fdtput PRIVATE fdt)
target_link_libraries(fdtdump PRIVATE fdt)
target_link_libraries(fdtoverlay PRIVATE fdt)
target_link_libraries(mkdtimg PRIVATE fdt ufdt_sysdeps)