/*
 * Copyright (C) 2019 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <jsonpb/verify.h>

#include <iostream>
#include <memory>
#include <sstream>
#include <string>

#include <absl/strings/str_cat.h>
#include <android-base/result.h>
#include <android-base/strings.h>
#include <google/protobuf/descriptor.h>
#include <google/protobuf/message.h>
#include <google/protobuf/reflection.h>
#include <google/protobuf/util/json_util.h>
#include <json/reader.h>
#include <json/writer.h>

namespace android {
namespace jsonpb {

using google::protobuf::FieldDescriptor;
using google::protobuf::Message;
using google::protobuf::util::JsonPrintOptions;

// Return json_name of the field. If it is not set, return the name of the
// field.
std::string GetJsonName(const FieldDescriptor& field_descriptor) {
  if (field_descriptor.has_json_name()) {
    return std::string(field_descriptor.json_name());
  }
  return std::string(field_descriptor.name());
}

// Params:
//   path: path to navigate inside JSON tree. For example, {"foo", "bar"}
//         for the value "string" in {"foo": {"bar" : "string"}}
android::base::Result<void> AllFieldsAreKnown(const Message& message, const Json::Value& json,
                                              std::vector<std::string>* path) {
  if (!json.isObject()) {
    return base::Error() << base::Join(*path, ".") << ": Not a JSON object";
  }
  auto&& descriptor = message.GetDescriptor();

  auto json_members = json.getMemberNames();
  std::set<std::string> json_keys{json_members.begin(), json_members.end()};

  std::set<std::string> known_keys;
  for (int i = 0; i < descriptor->field_count(); ++i) {
    known_keys.insert(GetJsonName(*descriptor->field(i)));
  }

  std::set<std::string> unknown_keys;
  std::set_difference(json_keys.begin(), json_keys.end(), known_keys.begin(), known_keys.end(),
                      std::inserter(unknown_keys, unknown_keys.begin()));

  if (!unknown_keys.empty()) {
    return base::Error()
           << base::Join(*path, ".") << ": contains unknown keys: ["
           << base::Join(unknown_keys, ", ") << "]. Keys must be a known field name of "
           << descriptor->full_name() << "(or its json_name option if set): ["
           << base::Join(known_keys, ", ") << "]";
  }

  std::stringstream errorss;

  // Check message fields.
  auto&& reflection = message.GetReflection();
  std::vector<const FieldDescriptor*> set_field_descriptors;
  reflection->ListFields(message, &set_field_descriptors);
  for (auto&& field_descriptor : set_field_descriptors) {
    if (field_descriptor->cpp_type() != FieldDescriptor::CppType::CPPTYPE_MESSAGE) {
      continue;
    }
    if (field_descriptor->is_map()) {
      continue;
    }

    std::string json_name = GetJsonName(*field_descriptor);
    const Json::Value& json_value = json[json_name];

    if (field_descriptor->is_repeated()) {
      auto&& fields = reflection->GetRepeatedFieldRef<Message>(message, field_descriptor);

      if (json_value.type() != Json::ValueType::arrayValue) {
        errorss << base::Join(*path, ".") << ": not a JSON list. This should not happen.\n";
        continue;
      }

      if (json_value.size() != static_cast<size_t>(fields.size())) {
        errorss << base::Join(*path, ".") << ": JSON list has size " << json_value.size()
                << " but message has size " << fields.size() << ". This should not happen.\n";
        continue;
      }

      std::unique_ptr<Message> scratch_space(fields.NewMessage());
      for (int i = 0; i < fields.size(); ++i) {
        path->push_back(absl::StrCat(json_name, "[", i, "]"));
        auto res = AllFieldsAreKnown(fields.Get(i, scratch_space.get()), json_value[i], path);
        path->pop_back();
        if (!res.ok()) {
          errorss << res.error().message();
        }
      }
    } else {
      auto&& field = reflection->GetMessage(message, field_descriptor);
      path->push_back(json_name);
      auto res = AllFieldsAreKnown(field, json_value, path);
      path->pop_back();
      if (!res.ok()) {
        errorss << res.error().message();
      }
    }
  }

  if (errorss.tellp() > 0) {
    return base::Error() << errorss.rdbuf();
  }
  return {};
}

android::base::Result<void> AllFieldsAreKnown(const google::protobuf::Message& message,
                                              const std::string& json) {
  Json::CharReaderBuilder builder;
  std::unique_ptr<Json::CharReader> reader(builder.newCharReader());
  Json::Value value;
  std::string error;
  if (!reader->parse(&*json.begin(), &*json.end(), &value, &error)) {
    return base::Error() << error;
  }

  std::vector<std::string> json_tree_path{"<root>"};
  return AllFieldsAreKnown(message, value, &json_tree_path);
}

android::base::Result<void> EqReformattedJson(const std::string& json,
                                              google::protobuf::Message* scratch_space) {
  Json::CharReaderBuilder builder;
  std::unique_ptr<Json::CharReader> reader(builder.newCharReader());
  Json::Value old_json;
  std::string error;
  if (!reader->parse(&*json.begin(), &*json.end(), &old_json, &error)) {
    return base::Error() << error;
  }

  auto new_json_string = internal::FormatJson(json, scratch_space);
  if (!new_json_string.ok()) {
    return new_json_string.error();
  }
  Json::Value new_json;
  if (!reader->parse(&*new_json_string->begin(), &*new_json_string->end(), &new_json, &error)) {
    return base::Error() << error;
  }

  if (old_json != new_json) {
    std::stringstream ss;
    ss << "Formatted JSON tree does not match source. Possible reasons "
          "include: \n"
          "- JSON Integers (without quotes) are matched against 64-bit "
          "integers in Prototype\n"
          "  (Reformatted integers will now have quotes.) Quote these integers "
          "in source\n"
          "  JSON or use 32-bit integers instead.\n"
          "- Enum values are stored as integers in source JSON file. Use enum "
          "value name \n"
          "  string instead, or change schema field to string / integers.\n"
          "- JSON keys are re-formatted to be lowerCamelCase. To fix, define "
          "json_name "
          "option\n"
          "  for appropriate fields.\n"
          "\n"
          "Reformatted JSON is printed below.\n";
    Json::StreamWriterBuilder factory;
    std::unique_ptr<Json::StreamWriter> const writer(factory.newStreamWriter());
    writer->write(new_json, &ss);
    return base::Error() << ss.str();
  }
  return {};
}

namespace internal {
android::base::Result<std::string> FormatJson(const std::string& json,
                                              google::protobuf::Message* scratch_space) {
  auto res = google::protobuf::util::JsonStringToMessage(json, scratch_space);
  if (!res.ok()) {
    return android::base::Error() << res;
  }
  std::string ret;
  res = google::protobuf::util::MessageToJsonString(*scratch_space, &ret,
                                                    JsonPrintOptions{
                                                        .add_whitespace = true,
                                                    });
  if (!res.ok()) {
    return android::base::Error() << res;
  }
  return ret;
}
}  // namespace internal

}  // namespace jsonpb
}  // namespace android
