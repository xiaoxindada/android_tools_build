/*
 * Copyright 2026 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// CXX bridge for utils_rs.
use crate::utils::{
    ComponentMix, ControlPoint, DynamicMetadata, FromSt209450Result, SimpleResult,
    ToSt209450Result, ToneMappingRule,
    to_st209450_ffi, is_valid_ffi, from_st209450_ffi, populate_implicit_parameters_ffi,
    dynamic_metadata_populate_pchip_slopes_ffi, dynamic_metadata_populate_using_rwtm,
};

#[cxx::bridge(namespace = "utils_ffi")]
pub mod ffi {
    extern "Rust" {
        type ToneMappingRule;
        fn clone(self: &ToneMappingRule) -> Box<ToneMappingRule>;
        #[cxx_name = "alternate_hdr_headroom_log2"]
        fn get_rule_headroom(self: &ToneMappingRule) -> f32;
        #[cxx_name = "set_alternate_hdr_headroom_log2"]
        fn set_rule_headroom(self: &mut ToneMappingRule, val: f32);
        #[cxx_name = "use_pchip_slope"]
        fn get_rule_use_pchip(self: &ToneMappingRule) -> bool;
        #[cxx_name = "set_use_pchip_slope"]
        fn set_rule_use_pchip(self: &mut ToneMappingRule, val: bool);
        #[cxx_name = "get_curve"]
        fn get_rule_curve(self: &ToneMappingRule) -> &[ControlPoint];
        #[cxx_name = "add_point"]
        fn add_rule_point(self: &mut ToneMappingRule, point: Box<ControlPoint>);
        #[cxx_name = "mix"]
        fn get_rule_mix(self: &ToneMappingRule) -> &ComponentMix;
        #[cxx_name = "set_mix"]
        fn set_rule_mix(self: &mut ToneMappingRule, mix: Box<ComponentMix>);
        #[cxx_name = "tone_mapping_rule_create_ffi"]
        fn tone_mapping_rule_create_ffi_bridge() -> Box<ToneMappingRule>;

        type DynamicMetadata;
        fn clone(self: &DynamicMetadata) -> Box<DynamicMetadata>;
        #[cxx_name = "has_adaptive_tone_map_flag"]
        fn get_meta_adaptive(self: &DynamicMetadata) -> bool;
        #[cxx_name = "set_has_adaptive_tone_map_flag"]
        fn set_meta_adaptive(self: &mut DynamicMetadata, val: bool);
        #[cxx_name = "use_reference_white_tone_mapping_flag"]
        fn get_meta_rwtm(self: &DynamicMetadata) -> bool;
        #[cxx_name = "set_use_reference_white_tone_mapping_flag"]
        fn set_meta_rwtm(self: &mut DynamicMetadata, val: bool);
        #[cxx_name = "hdr_reference_white"]
        fn get_meta_hdr_ref(self: &DynamicMetadata) -> f32;
        #[cxx_name = "set_hdr_reference_white"]
        fn set_meta_hdr_ref(self: &mut DynamicMetadata, val: f32);
        #[cxx_name = "baseline_hdr_headroom_log2"]
        fn get_meta_baseline(self: &DynamicMetadata) -> f32;
        #[cxx_name = "set_baseline_hdr_headroom_log2"]
        fn set_meta_baseline(self: &mut DynamicMetadata, val: f32);
        #[cxx_name = "gain_application_space_chromaticities"]
        fn get_meta_chrom(self: &DynamicMetadata) -> &[f32; 8];
        #[cxx_name = "set_gain_application_space_chromaticities"]
        fn set_meta_chrom(self: &mut DynamicMetadata, val: &[f32; 8]);
        #[cxx_name = "get_rules"]
        fn get_meta_rules(self: &DynamicMetadata) -> &[ToneMappingRule];
        #[cxx_name = "add_rule"]
        fn add_meta_rule(self: &mut DynamicMetadata, rule: Box<ToneMappingRule>);
        #[cxx_name = "dynamic_metadata_create_ffi"]
        fn dynamic_metadata_create_ffi_bridge() -> Box<DynamicMetadata>;

        type ControlPoint;
        fn clone(self: &ControlPoint) -> Box<ControlPoint>;
        fn x(self: &ControlPoint) -> f32;
        fn y(self: &ControlPoint) -> f32;
        fn m(self: &ControlPoint) -> f32;
        #[cxx_name = "control_point_create_ffi"]
        fn control_point_create_ffi_bridge(x: f32, y: f32, m: f32) -> Box<ControlPoint>;
        fn control_point_ptr(pt: &ControlPoint) -> Box<ControlPoint>;

        type ComponentMix;
        fn clone(self: &ComponentMix) -> Box<ComponentMix>;
        fn rgb(self: &ComponentMix) -> &[f32; 3];
        fn max(self: &ComponentMix) -> f32;
        fn min(self: &ComponentMix) -> f32;
        fn component(self: &ComponentMix) -> f32;
        #[cxx_name = "component_mix_create_ffi"]
        fn component_mix_create_ffi_bridge(
            rgb: [f32; 3],
            max: f32,
            min: f32,
            component: f32,
        ) -> Box<ComponentMix>;
        fn component_mix_ptr(mix: &ComponentMix) -> Box<ComponentMix>;

        type ToSt209450Result;
        fn clone(self: &ToSt209450Result) -> Box<ToSt209450Result>;
        fn success(self: &ToSt209450Result) -> bool;
        #[cxx_name = "get_data"]
        fn to_result_data(self: &ToSt209450Result) -> &[u8];
        #[cxx_name = "get_error_message"]
        fn to_result_error(self: &ToSt209450Result) -> &str;

        type FromSt209450Result;
        fn clone(self: &FromSt209450Result) -> Box<FromSt209450Result>;
        fn success(self: &FromSt209450Result) -> bool;
        #[cxx_name = "metadata"]
        fn from_result_metadata(self: &FromSt209450Result) -> &DynamicMetadata;
        #[cxx_name = "get_error_message"]
        fn from_result_error(self: &FromSt209450Result) -> &str;

        type SimpleResult;
        fn clone(self: &SimpleResult) -> Box<SimpleResult>;
        fn success(self: &SimpleResult) -> bool;
        #[cxx_name = "get_error_message"]
        fn simple_result_error(self: &SimpleResult) -> &str;

        #[cxx_name = "to_st209450_ffi"]
        fn to_st209450_ffi_bridge(metadata: &DynamicMetadata) -> Box<ToSt209450Result>;
        #[cxx_name = "is_valid_ffi"]
        fn is_valid_ffi_bridge(metadata: &DynamicMetadata) -> bool;
        #[cxx_name = "from_st209450_ffi"]
        fn from_st209450_ffi_bridge(data: &[u8]) -> Box<FromSt209450Result>;
        #[cxx_name = "populate_implicit_parameters_ffi"]
        fn populate_implicit_parameters_ffi_bridge(metadata: &mut DynamicMetadata) -> Box<SimpleResult>;
        #[cxx_name = "dynamic_metadata_populate_pchip_slopes_ffi"]
        fn dynamic_metadata_populate_pchip_slopes_ffi_bridge(
            metadata: &mut DynamicMetadata,
        ) -> Box<SimpleResult>;
        #[cxx_name = "dynamic_metadata_populate_using_rwtm_ffi"]
        fn dynamic_metadata_populate_using_rwtm_ffi_bridge(metadata: &mut DynamicMetadata);

        fn tone_mapping_rule_ptr(rule: &ToneMappingRule) -> Box<ToneMappingRule>;
        fn dynamic_metadata_ptr(meta: &DynamicMetadata) -> Box<DynamicMetadata>;
    }
}

pub fn tone_mapping_rule_create_ffi_bridge() -> Box<ToneMappingRule> {
    Box::new(ToneMappingRule::default())
}

pub fn dynamic_metadata_create_ffi_bridge() -> Box<DynamicMetadata> {
    Box::new(DynamicMetadata::default())
}

pub fn control_point_create_ffi_bridge(x: f32, y: f32, m: f32) -> Box<ControlPoint> {
    Box::new(ControlPoint { x, y, m })
}

pub fn component_mix_create_ffi_bridge(
    rgb: [f32; 3],
    max: f32,
    min: f32,
    component: f32,
) -> Box<ComponentMix> {
    Box::new(ComponentMix { rgb, max, min, component })
}

pub fn control_point_ptr(pt: &ControlPoint) -> Box<ControlPoint> {
    pt.clone()
}

pub fn component_mix_ptr(mix: &ComponentMix) -> Box<ComponentMix> {
    mix.clone()
}

pub fn tone_mapping_rule_ptr(rule: &ToneMappingRule) -> Box<ToneMappingRule> {
    rule.clone()
}

pub fn dynamic_metadata_ptr(meta: &DynamicMetadata) -> Box<DynamicMetadata> {
    meta.clone()
}

pub fn to_st209450_ffi_bridge(metadata: &DynamicMetadata) -> Box<ToSt209450Result> {
    Box::new(to_st209450_ffi(metadata))
}

pub fn is_valid_ffi_bridge(metadata: &DynamicMetadata) -> bool {
    is_valid_ffi(metadata)
}

pub fn from_st209450_ffi_bridge(data: &[u8]) -> Box<FromSt209450Result> {
    Box::new(from_st209450_ffi(data))
}

pub fn populate_implicit_parameters_ffi_bridge(metadata: &mut DynamicMetadata) -> Box<SimpleResult> {
    Box::new(populate_implicit_parameters_ffi(metadata))
}

pub fn dynamic_metadata_populate_pchip_slopes_ffi_bridge(
    metadata: &mut DynamicMetadata,
) -> Box<SimpleResult> {
    Box::new(dynamic_metadata_populate_pchip_slopes_ffi(metadata))
}

pub fn dynamic_metadata_populate_using_rwtm_ffi_bridge(metadata: &mut DynamicMetadata) {
    dynamic_metadata_populate_using_rwtm(metadata);
}

impl ToneMappingRule {
    fn clone(&self) -> Box<ToneMappingRule> {
        Box::new(Clone::clone(self))
    }
    fn get_rule_headroom(&self) -> f32 {
        self.alternate_hdr_headroom_log2
    }
    fn set_rule_headroom(&mut self, val: f32) {
        self.alternate_hdr_headroom_log2 = val;
    }
    fn get_rule_use_pchip(&self) -> bool {
        self.use_pchip_slope
    }
    fn set_rule_use_pchip(&mut self, val: bool) {
        self.use_pchip_slope = val;
    }
    fn get_rule_curve(&self) -> &[ControlPoint] {
        &self.curve
    }
    fn add_rule_point(&mut self, point: Box<ControlPoint>) {
        self.curve.push(*point);
    }
    fn get_rule_mix(&self) -> &ComponentMix {
        &self.mix
    }
    fn set_rule_mix(&mut self, mix: Box<ComponentMix>) {
        self.mix = *mix;
    }
}

impl DynamicMetadata {
    fn clone(&self) -> Box<DynamicMetadata> {
        Box::new(Clone::clone(self))
    }
    fn get_meta_adaptive(&self) -> bool {
        self.has_adaptive_tone_map_flag
    }
    fn set_meta_adaptive(&mut self, val: bool) {
        self.has_adaptive_tone_map_flag = val;
    }
    fn get_meta_rwtm(&self) -> bool {
        self.use_reference_white_tone_mapping_flag
    }
    fn set_meta_rwtm(&mut self, val: bool) {
        self.use_reference_white_tone_mapping_flag = val;
    }
    fn get_meta_hdr_ref(&self) -> f32 {
        self.hdr_reference_white
    }
    fn set_meta_hdr_ref(&mut self, val: f32) {
        self.hdr_reference_white = val;
    }
    fn get_meta_baseline(&self) -> f32 {
        self.baseline_hdr_headroom_log2
    }
    fn set_meta_baseline(&mut self, val: f32) {
        self.baseline_hdr_headroom_log2 = val;
    }
    fn get_meta_chrom(&self) -> &[f32; 8] {
        &self.gain_application_space_chromaticities
    }
    fn set_meta_chrom(&mut self, val: &[f32; 8]) {
        self.gain_application_space_chromaticities = *val;
    }
    fn get_meta_rules(&self) -> &[ToneMappingRule] {
        &self.rules
    }
    fn add_meta_rule(&mut self, rule: Box<ToneMappingRule>) {
        self.rules.push(*rule);
    }
}

impl ControlPoint {
    fn clone(&self) -> Box<ControlPoint> {
        Box::new(Clone::clone(self))
    }
    fn x(&self) -> f32 {
        self.x
    }
    fn y(&self) -> f32 {
        self.y
    }
    fn m(&self) -> f32 {
        self.m
    }
}

impl ComponentMix {
    fn clone(&self) -> Box<ComponentMix> {
        Box::new(Clone::clone(self))
    }
    fn rgb(&self) -> &[f32; 3] {
        &self.rgb
    }
    fn max(&self) -> f32 {
        self.max
    }
    fn min(&self) -> f32 {
        self.min
    }
    fn component(&self) -> f32 {
        self.component
    }
}

impl Clone for ToSt209450Result {
    fn clone(&self) -> Self {
        Self {
            success: self.success,
            data: self.data.clone(),
            error_message: self.error_message.clone(),
        }
    }
}

impl ToSt209450Result {
    fn clone(&self) -> Box<ToSt209450Result> {
        Box::new(Clone::clone(self))
    }
    fn success(&self) -> bool {
        self.success
    }
    fn to_result_data(&self) -> &[u8] {
        &self.data
    }
    fn to_result_error(&self) -> &str {
        &self.error_message
    }
}

impl Clone for FromSt209450Result {
    fn clone(&self) -> Self {
        Self {
            success: self.success,
            metadata: Clone::clone(&self.metadata),
            error_message: self.error_message.clone(),
        }
    }
}

impl FromSt209450Result {
    fn clone(&self) -> Box<FromSt209450Result> {
        Box::new(Clone::clone(self))
    }
    fn success(&self) -> bool {
        self.success
    }
    fn from_result_metadata(&self) -> &DynamicMetadata {
        &self.metadata
    }
    fn from_result_error(&self) -> &str {
        &self.error_message
    }
}

impl Clone for SimpleResult {
    fn clone(&self) -> Self {
        Self {
            success: self.success,
            error_message: self.error_message.clone(),
        }
    }
}

impl SimpleResult {
    fn clone(&self) -> Box<SimpleResult> {
        Box::new(Clone::clone(self))
    }
    fn success(&self) -> bool {
        self.success
    }
    fn simple_result_error(&self) -> &str {
        &self.error_message
    }
}
