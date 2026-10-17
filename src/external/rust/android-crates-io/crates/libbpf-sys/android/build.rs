//! libbpf bindgen configuration
// Modified from the upstream libbpf-sys build.rs file in the libbpf-sys root dir.
use std::collections::HashSet;
fn main() {
    #[derive(Debug)]
    struct IgnoreMacros(HashSet<&'static str>);
    impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
        fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
            if self.0.contains(name) {
                bindgen::callbacks::MacroParsingBehavior::Ignore
            } else {
                bindgen::callbacks::MacroParsingBehavior::Default
            }
        }
    }
    let ignored_macros = IgnoreMacros(
        vec![
            "BTF_KIND_FUNC",
            "BTF_KIND_FUNC_PROTO",
            "BTF_KIND_VAR",
            "BTF_KIND_DATASEC",
            "BTF_KIND_FLOAT",
            "BTF_KIND_DECL_TAG",
            "BTF_KIND_TYPE_TAG",
            "BTF_KIND_ENUM64",
        ]
        .into_iter()
        .collect(),
    );
    bindgen_cmd::build(|mut builder| {
        builder = builder
            .derive_default(true)
            .explicit_padding(true)
            .default_enum_style(bindgen::EnumVariation::Consts)
            .size_t_is_usize(false)
            .prepend_enum_name(false)
            .layout_tests(false)
            .generate_comments(false)
            .emit_builtins()
            .allowlist_function("bpf_.+")
            .allowlist_function("btf_.+")
            .allowlist_function("libbpf_.+")
            .allowlist_function("perf_.+")
            .allowlist_function("ring_buffer_.+")
            .allowlist_function("ring__.+")
            .allowlist_function("user_ring_buffer_.+")
            .allowlist_function("vdprintf")
            .allowlist_type("bpf_.+")
            .allowlist_type("btf_.+")
            .allowlist_type("xdp_.+")
            .allowlist_type("perf_.+")
            .allowlist_var("BPF_.+")
            .allowlist_var("BTF_.+")
            .allowlist_var("XDP_.+")
            .allowlist_var("PERF_.+")
            .parse_callbacks(Box::new(ignored_macros));
        builder
    })
}
