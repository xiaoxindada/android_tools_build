set(target_name "zucchini")
set(zucchini_dir "${CMAKE_SOURCE_DIR}/src/zucchini")

set(cflags
        "-DUSE_BRILLO=1"
        "-D_FILE_OFFSET_BITS=64"
        "-Wall"
        "-Wextra"
        "-Wimplicit-fallthrough"
        "-Wno-unused-parameter"
)

set(libzucchini_srcs
        "${zucchini_dir}/abs32_utils.cc"
        "${zucchini_dir}/address_translator.cc"
        "${zucchini_dir}/arm_utils.cc"
        "${zucchini_dir}/binary_data_histogram.cc"
        "${zucchini_dir}/buffer_sink.cc"
        "${zucchini_dir}/buffer_source.cc"
        "${zucchini_dir}/crc32.cc"
        "${zucchini_dir}/disassembler.cc"
        "${zucchini_dir}/disassembler_dex.cc"
        "${zucchini_dir}/disassembler_elf.cc"
        "${zucchini_dir}/disassembler_no_op.cc"
        "${zucchini_dir}/disassembler_win32.cc"
        "${zucchini_dir}/disassembler_ztf.cc"
        "${zucchini_dir}/element_detection.cc"
        "${zucchini_dir}/encoded_view.cc"
        "${zucchini_dir}/ensemble_matcher.cc"
        "${zucchini_dir}/equivalence_map.cc"
        "${zucchini_dir}/heuristic_ensemble_matcher.cc"
        "${zucchini_dir}/image_index.cc"
        "${zucchini_dir}/imposed_ensemble_matcher.cc"
        "${zucchini_dir}/io_utils.cc"
        "${zucchini_dir}/mapped_file.cc"
        "${zucchini_dir}/patch_reader.cc"
        "${zucchini_dir}/patch_writer.cc"
        "${zucchini_dir}/reference_bytes_mixer.cc"
        "${zucchini_dir}/reference_set.cc"
        "${zucchini_dir}/rel32_finder.cc"
        "${zucchini_dir}/rel32_utils.cc"
        "${zucchini_dir}/reloc_elf.cc"
        "${zucchini_dir}/reloc_win32.cc"
        "${zucchini_dir}/target_pool.cc"
        "${zucchini_dir}/targets_affinity.cc"
        "${zucchini_dir}/zucchini_apply.cc"
        "${zucchini_dir}/zucchini_gen.cc"
        "${zucchini_dir}/zucchini_tools.cc"
)

add_library(${target_name} STATIC ${libzucchini_srcs})
target_compile_options(${target_name} PRIVATE ${cflags})
target_include_directories(${target_name} PUBLIC
    ${libchrome_dir}
    ${libzucchini_headers}
)
target_link_libraries(${target_name} PUBLIC
    chrome
    cutils
    log
    base
)

