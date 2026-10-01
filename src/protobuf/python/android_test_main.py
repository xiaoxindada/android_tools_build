import importlib
import unittest
import warnings

TESTS = [
    "any_test",
    "decoder_test",
    "descriptor_database_test",
    "descriptor_pool_test",
    "descriptor_test",
    "duration_test",
    "field_mask_test",
    "generator_test",
    "import_test",
    "json_format_test",
    "keywords_test",
    "message_factory_test",
    "message_test",
    "proto_builder_test",
    "proto_json_test",
    "proto_test",
    "proto_text_test",
    "reflection_cpp_test",
    "reflection_test",
    "runtime_version_test",
    "service_reflection_test",
    "symbol_database_test",
    "text_encoding_test",
    "text_format_test",
    "thread_safe_test",
    "timestamp_test",
    "unknown_fields_test",
    # Requires zoneinfo, which is currently broken in host Python (missing tzdata)
    # "well_known_types_test",
    "wire_format_test",
]

def noop(*args, **kwargs):
    pass

if __name__ == "__main__":
    # Printing the warnings may cause the test to fail in TradeFed
    warnings.showwarning = noop

    test_suite = unittest.TestSuite()
    for test_name in TESTS:
        test_module = importlib.import_module(f"google.protobuf.internal.{test_name}")
        test_suite.addTest(unittest.defaultTestLoader.loadTestsFromModule(test_module))

    # warnings="default" is required to match the behavior of unittest.main()
    runner = unittest.TextTestRunner(verbosity=2, warnings="default")
    runner.run(test_suite)
