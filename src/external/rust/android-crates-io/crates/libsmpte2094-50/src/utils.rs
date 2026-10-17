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
// Basic types, according to SMPTE ST 2094-50.
// Serialization and deserialization functions are also provided.
use std::f32::consts::PI;
use crate::pchip::pchip_slopes;

const NUM_MIX_PARAMS: usize = 6;
const MIX_PARAM_SCALE: f32 = 50000.0;
const APPLICATION_VERSION: i32 = 0;
const MINIMUM_APPLICATION_VERSION: i32 = 0;
const DEFAULT_HDR_REFERENCE_WHITE: f32 = 203.0;

const MAX_CURVE_SIZE: usize = 32;
const MAX_RULES_SIZE: usize = 4;

const GAIN_APPLICATION_SPACE_CHROMATICITIES: [[f32; 8]; 3] = [
    [0.64, 0.33, 0.3, 0.6, 0.15, 0.06, 0.3127, 0.329], // Rec. 709 (sRGB)
    [0.68, 0.32, 0.265, 0.69, 0.15, 0.06, 0.3127, 0.329], // Display P3
    [0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329], // Rec. 2020
];

/// Returns the value that would be encoded for a given float value and scale.
fn encoded_float(value: f32, scale: f32) -> u16 {
    // Assumes that for negative values, the sign is encoded separately, and we only care about the
    // absolute value here.
    (value.abs() * scale).round() as u16
}

/// Returns the value that would be decoded from a given encoded float value and scale.
fn encode_decode_float(value: f32, scale: f32) -> f32 {
    encoded_float(value, scale) as f32 / scale * value.signum()
}

/// A 2D control point with an optional slope `m`, used to define tone mapping curves.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ControlPoint {
    /// The x-coordinate of the control point, in the range [0.0, 64.0].
    pub x: f32,
    /// The y-coordinate of the control point, in the range [-6.0, 6.0].
    pub y: f32,
    /// Optional slope (derivative) at this control point, used if `use_pchip_slope` is false.
    pub m: f32,
}

impl ControlPoint {
    /// Returns true if the point satisfies the validity constraints in SMPTE ST 2094-50, section 6.5.2.
    pub fn is_valid(&self) -> bool {
        // SMPTE ST 2094-50, section 6.5.2.
        (0.0..=64.0).contains(&self.x) && (-6.0..=6.0).contains(&self.y)
    }
}

/// Component mixing parameters for tone mapping rules, defined according to SMPTE ST 2094-50.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ComponentMix {
    /// Mixing weights for the respective R, G, and B color channels, each in [0.0, 1.0].
    pub rgb: [f32; 3],
    /// Mixing weight for the maximum color channel value (max(R, G, B)), in [0.0, 1.0].
    pub max: f32,
    /// Mixing weight for the minimum color channel value (min(R, G, B)), in [0.0, 1.0].
    pub min: f32,
    /// Mixing weight for the individual component channel, in [0.0, 1.0].
    pub component: f32,
}

impl ComponentMix {
    /// Returns true if the component mix satisfies the validity constraints in SMPTE ST 2094-50, section 6.4.2.
    pub fn is_valid(&self) -> bool {
        // SMPTE ST 2094-50, section 6.4.2.
        let all_in_range = self.rgb.iter().all(|&c| (0.0..=1.0).contains(&c))
            && (0.0..=1.0).contains(&self.max)
            && (0.0..=1.0).contains(&self.min)
            && (0.0..=1.0).contains(&self.component);
        let sum = self.rgb[0] + self.rgb[1] + self.rgb[2] + self.max + self.min + self.component;
        all_in_range && sum > 0.0
    }

    // Returns the mix encoding type, in {0..3}.
    fn mix_type(&self) -> i32 {
        if self.rgb == [0.0, 0.0, 0.0]
            && self.max == 1.0
            && self.min == 0.0
            && self.component == 0.0
        {
            0
        } else if self.rgb == [0.0, 0.0, 0.0]
            && self.max == 0.0
            && self.min == 0.0
            && self.component == 1.0
        {
            1
        } else if self.rgb[0] == self.rgb[1]
            && self.rgb[1] == self.rgb[2]
            && self.max == 0.5
            && self.min == 0.0
            && self.component == 0.0
        {
            2
        } else {
            3
        }
    }
}

/// Alternative tone mapping rule defining a curve, headroom, and mixing parameters.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToneMappingRule {
    /// The log2 upper limit of the alternate display headroom, in the range [0.0, 6.0].
    pub alternate_hdr_headroom_log2: f32,
    /// The sequence of control points specifying the tone mapping curve (up to 32 points).
    pub curve: Vec<ControlPoint>,
    /// True if PCHIP slopes should be computed dynamically, false to use `m` from control points.
    pub use_pchip_slope: bool,
    /// The component mixing parameters for this tone mapping rule.
    pub mix: ComponentMix,
}

impl ToneMappingRule {
    /// Returns true if the tone mapping rule satisfies the validity constraints in SMPTE ST 2094-50.
    pub fn is_valid(&self) -> bool {
        // SMPTE ST 2094-50, section 6.2.2.
        if !(0.0..=6.0).contains(&self.alternate_hdr_headroom_log2) {
            return false;
        }
        // SMPTE ST 2094-50, section 6.5.2.
        if self.curve.is_empty() || self.curve.len() > 32 {
            return false;
        }
        for i in 0..self.curve.len() - 1 {
            if self.curve[i].x > self.curve[i + 1].x {
                return false;
            }
            let encoded_x = encoded_float(self.curve[i].x, 1000.0);
            let encoded_x_next = encoded_float(self.curve[i + 1].x, 1000.0);
            let encoded_y = encoded_float(self.curve[i].y, 10000.0);
            let encoded_y_next = encoded_float(self.curve[i + 1].y, 10000.0);
            if encoded_x == encoded_x_next && encoded_y != encoded_y_next {
                return false;
            }
        }
        self.curve.iter().all(|p| p.is_valid()) && self.mix.is_valid()
    }

    /// Returns a reference to the control points defining the curve.
    pub fn get_curve(&self) -> &[ControlPoint] {
        &self.curve
    }
    /// Adds a control point to the curve.
    pub fn add_point(&mut self, point: ControlPoint) {
        self.curve.push(point);
    }
}

/// SMPTE ST 2094-50 dynamic metadata container, holding global parameters and tone mapping rules.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DynamicMetadata {
    /// True if adaptive tone mapping metadata is present.
    pub has_adaptive_tone_map_flag: bool,
    /// True if Reference White Tone Mapping (RWTM) is used to populate the rules.
    pub use_reference_white_tone_mapping_flag: bool,
    /// The mastering display HDR reference white in nits, in the range (0.0, 10000.0].
    pub hdr_reference_white: f32,
    /// The log2 upper limit of the baseline mastering display headroom, in the range [0.0, 6.0].
    pub baseline_hdr_headroom_log2: f32,
    /// Primaries and white point chromaticities of the color space where gain is applied.
    pub gain_application_space_chromaticities: [f32; 8],
    /// The sequence of alternative tone mapping rules (at most 4 elements).
    pub rules: Vec<ToneMappingRule>,
}

/// Result of a SMPTE ST 2094-50 serialization for C++ wrapping, containing the serialized bytes or an error message.
pub struct ToSt209450Result {
    /// True if the serialization was successful, false otherwise.
    pub success: bool,
    /// The serialized SMPTE ST 2094-50 binary data buffer.
    pub data: Vec<u8>,
    /// Detailed error message if serialization failed.
    pub error_message: String,
}

impl ToSt209450Result {
    /// Returns the serialized data buffer if successful.
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    /// Returns the error message if serialization failed.
    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }
}

/// Foreign function interface wrapper for `DynamicMetadata::to_st2094_50`.
pub fn to_st209450_ffi(metadata: &DynamicMetadata) -> ToSt209450Result {
    match metadata.to_st2094_50() {
        Ok(data) => ToSt209450Result { success: true, data, error_message: String::new() },
        Err(e) => ToSt209450Result { success: false, data: Vec::new(), error_message: e },
    }
}

/// Foreign function interface wrapper for `DynamicMetadata::is_valid`.
pub fn is_valid_ffi(metadata: &DynamicMetadata) -> bool {
    metadata.is_valid()
}



/// Result of a SMPTE ST 2094-50 deserialization for C++ wrapping, containing the parsed metadata or an error message.
#[repr(C)]
pub struct FromSt209450Result {
    /// True if deserialization was successful, false otherwise.
    pub success: bool,
    /// The deserialized dynamic metadata container.
    pub metadata: DynamicMetadata,
    /// Detailed error message if deserialization failed.
    pub error_message: String,
}

impl FromSt209450Result {
    /// Returns the error message if deserialization failed.
    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }
}

/// Foreign function interface wrapper for `DynamicMetadata::from_st2094_50`.
pub fn from_st209450_ffi(data: &[u8]) -> FromSt209450Result {
    match DynamicMetadata::from_st2094_50(data) {
        Ok(metadata) => {
            FromSt209450Result { success: true, metadata, error_message: String::new() }
        }
        Err(e) => FromSt209450Result {
            success: false,
            metadata: DynamicMetadata::default(),
            error_message: e,
        },
    }
}

/// A generic result structure for C++ wrapping representing success or failure with an error message.
pub struct SimpleResult {
    /// True if the operation was successful, false otherwise.
    pub success: bool,
    /// Detailed error message if the operation failed.
    pub error_message: String,
}

impl SimpleResult {
    /// Returns the error message if the operation failed.
    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }
}

/// Converts a Rust `Result<(), String>` into a `SimpleResult` for FFI returns.
pub fn to_simple_result(result: Result<(), String>) -> SimpleResult {
    match result {
        Ok(_) => SimpleResult { success: true, error_message: String::new() },
        Err(e) => SimpleResult { success: false, error_message: e },
    }
}

/// Foreign function interface wrapper for `DynamicMetadata::populate_implicit_parameters`.
pub fn populate_implicit_parameters_ffi(metadata: &mut DynamicMetadata) -> SimpleResult {
    to_simple_result(metadata.populate_implicit_parameters())
}

/// Foreign function interface wrapper for `DynamicMetadata::populate_pchip_slopes`.
pub fn dynamic_metadata_populate_pchip_slopes_ffi(metadata: &mut DynamicMetadata) -> SimpleResult {
    to_simple_result(metadata.populate_pchip_slopes())
}

impl DynamicMetadata {
    /// Returns true if the metadata satisfies all mandatory parameter constraints in SMPTE ST 2094-50,
    /// including constraints required for serialization.
    pub fn is_valid(&self) -> bool {
        // SMPTE ST 2094-50, section 6.1.2.
        if self.hdr_reference_white <= 0.0 || self.hdr_reference_white > 10000.0 {
            return false;
        }
        let encoded_hdr_reference_white = encoded_float(self.hdr_reference_white, 5.0);
        if encoded_hdr_reference_white == 0 {
            return false;
        }
        // SMPTE ST 2094-50, section 6.2.2.
        if !(0.0..=6.0).contains(&self.baseline_hdr_headroom_log2) {
            return false;
        }
        let encoded_baseline = encoded_float(self.baseline_hdr_headroom_log2, 10000.0);
        if self.rules.len() > 4 {
            return false;
        }
        for i in 0..self.rules.len() {
            let encoded_alt = encoded_float(self.rules[i].alternate_hdr_headroom_log2, 10000.0);
            if encoded_alt == encoded_baseline {
                return false;
            }
            if i > 0 {
                let alt_prev = self.rules[i - 1].alternate_hdr_headroom_log2;
                let encoded_alt_prev = encoded_float(alt_prev, 10000.0);
                if encoded_alt_prev >= encoded_alt {
                    return false;
                }
            }
        }
        if self.gain_application_space_chromaticities.iter().any(|&c| !(0.0..=1.0).contains(&c)) {
            return false;
        }

        // SMPTE ST 2094-50, section 6.2.2.
        // Check triangle area and interior point.
        let c = self.gain_application_space_chromaticities.map(|c| encode_decode_float(c, 50000.0));
        let (xr, yr) = (c[0], c[1]);
        let (xg, yg) = (c[2], c[3]);
        let (xb, yb) = (c[4], c[5]);
        let (xw, yw) = (c[6], c[7]);

        let area = 0.5 * (xr * (yg - yb) + xg * (yb - yr) + xb * (yr - yg)).abs();
        if area == 0.0 {
            return false;
        }
        // Barycentric coordinates for white point
        let det = (yg - yb) * (xr - xb) + (xb - xg) * (yr - yb);
        let l1 = ((yg - yb) * (xw - xb) + (xb - xg) * (yw - yb)) / det;
        let l2 = ((yb - yr) * (xw - xb) + (xr - xb) * (yw - yb)) / det;
        let l3 = 1.0 - l1 - l2;
        // Check that the white point is inside or very close to the boundary of
        // the RGB triangle.
        if l1 <= 0.0 || l2 <= 0.0 || l3 <= 0.0 {
            return false;
        }

        for rule in &self.rules {
            if !rule.is_valid() {
                return false;
            }

            // SMPTE ST 2094-50, section 6.2.3.
            // Check the sign of y values.
            // This constraint is a "should" in the spec but is mandatory for serialization.
            let h_alt = rule.alternate_hdr_headroom_log2;
            let h_baseline = self.baseline_hdr_headroom_log2;
            for pt in &rule.curve {
                if h_alt > h_baseline && pt.y < 0.0 {
                    return false;
                }
                if h_alt < h_baseline && pt.y > 0.0 {
                    return false;
                }
            }
        }
        true
    }

    /// Returns the index mode for the gain application space chromaticities, or 3 if custom.
    fn gain_application_space_chromaticities_mode(&self) -> i32 {
        GAIN_APPLICATION_SPACE_CHROMATICITIES
            .iter()
            .position(|&chromaticities| {
                self.gain_application_space_chromaticities == chromaticities
            })
            .unwrap_or(3) as i32
    }

    /// Returns a reference to the tone mapping rules.
    pub fn get_rules(&self) -> &[ToneMappingRule] {
        &self.rules
    }

    /// Adds a tone mapping rule to the metadata.
    pub fn add_rule(&mut self, rule: ToneMappingRule) {
        self.rules.push(rule);
    }

    /// Implements section C.3.8 of SMPTE ST 2094-50, populating rules using Reference White Tone Mapping (RWTM).
    fn populate_using_rwtm(&mut self) {
        self.has_adaptive_tone_map_flag = true;
        self.use_reference_white_tone_mapping_flag = true;
        self.gain_application_space_chromaticities = GAIN_APPLICATION_SPACE_CHROMATICITIES[2];

        if self.baseline_hdr_headroom_log2 == 0.0 {
            self.rules.clear();
            return;
        }

        // Set the two alternate image headrooms using Formula (C.1).
        self.rules.resize(2, ToneMappingRule::default());
        self.rules[0].alternate_hdr_headroom_log2 = 0.0;
        self.rules[1].alternate_hdr_headroom_log2 = (8.0f32 / 3.0f32).log2()
            * (self.baseline_hdr_headroom_log2 / (1000.0f32 / 203.0f32).log2()).min(1.0);

        for (a, rule) in self.rules.iter_mut().enumerate() {
            // Use maxRGB for applying the curve.
            rule.mix.max = 1.0;
            rule.mix.rgb = [0.0; 3];
            rule.mix.min = 0.0;
            rule.mix.component = 0.0;

            // Compute y_white from Formula (C.2).
            let y_white = if a == 1 {
                1.0
            } else {
                1.0 - 0.5
                    * (self.baseline_hdr_headroom_log2 / (1000.0f32 / 203.0f32).log2()).min(1.0)
            };

            // Compute the Bezier control points using Formula (C.3).
            let kappa = 0.65;
            let x_knee = 1.0;
            let y_knee = y_white;
            let x_max = self.baseline_hdr_headroom_log2.exp2();
            let y_max = rule.alternate_hdr_headroom_log2.exp2();
            let x_mid = (1.0 - kappa) * x_knee + kappa * (x_knee * y_max / y_knee);
            let y_mid = (1.0 - kappa) * y_knee + kappa * y_max;

            // Compute the quadratic coefficients using Formula (C.5).
            let a_x = x_knee - 2.0 * x_mid + x_max;
            let a_y = y_knee - 2.0 * y_mid + y_max;
            let b_x = 2.0 * x_mid - 2.0 * x_knee;
            let b_y = 2.0 * y_mid - 2.0 * y_knee;
            let c_x = x_knee;
            let c_y = y_knee;

            rule.curve.resize(8, ControlPoint::default());
            rule.use_pchip_slope = false;
            for c in 0..8 {
                // Compute the linear domain curve values using Formula (C.4).
                let t = c as f32 / 7.0;
                let x = a_x * t * t + b_x * t + c_x;
                let y = a_y * t * t + b_y * t + c_y;
                let m = (2.0 * a_y * t + b_y) / (2.0 * a_x * t + b_x);

                // Compute the log domain curve values using Formula (C.6).
                rule.curve[c].x = x;
                rule.curve[c].y = (y / x).log2();
                rule.curve[c].m = (x * m - y) / (x * y * 2.0f32.ln());
            }
        }
    }

    /// Populates the PCHIP slopes for each rule in the metadata.
    ///
    /// For those rules, the existing slope values, if any, are overwritten
    /// and the `use_pchip_slope` flag is set to `false`.
    pub fn populate_pchip_slopes(&mut self) -> Result<(), String> {
        for rule in &mut self.rules {
            if rule.curve.is_empty() || !rule.use_pchip_slope {
                continue;
            }
            let x: Vec<f32> = rule.curve.iter().map(|pt| pt.x).collect();
            let y: Vec<f32> = rule.curve.iter().map(|pt| pt.y).collect();
            let slopes = pchip_slopes(&x, &y)?;
            if slopes.len() != rule.curve.len() {
                return Err(format!(
                    "Number of PCHIP slopes {} does not match number of control points {}",
                    slopes.len(),
                    rule.curve.len()
                ));
            }
            for (i, slope) in slopes.iter().enumerate() {
                rule.curve[i].m = *slope;
            }
            rule.use_pchip_slope = false;
        }
        Ok(())
    }

    /// Populates implicit parameters in the metadata, such as RWTM rules and PCHIP slopes.
    pub fn populate_implicit_parameters(&mut self) -> Result<(), String> {
        if self.use_reference_white_tone_mapping_flag {
            self.populate_using_rwtm();
        }
        self.populate_pchip_slopes()
    }

    /// Serializes the dynamic metadata to the SMPTE ST 2094-50 binary format.
    fn to_st2094_50(&self) -> Result<Vec<u8>, String> {
        let mut writer = BitWriter::new();

        // smpte_st_2094_50_application_info()
        writer.write(APPLICATION_VERSION as u16, 3);
        writer.write(MINIMUM_APPLICATION_VERSION as u16, 3);
        writer.write(0, 2); // reserved

        // smpte_st_2094_50_color_volume_transform()
        writer.write_bit(self.hdr_reference_white != DEFAULT_HDR_REFERENCE_WHITE);
        writer.write_bit(self.has_adaptive_tone_map_flag);
        writer.write(0, 6); // reserved

        if self.hdr_reference_white != DEFAULT_HDR_REFERENCE_WHITE {
            if !(0.0..=10000.0).contains(&self.hdr_reference_white) {
                return Err(format!(
                    "hdr_reference_white: {} is out of range (0, 10000]",
                    self.hdr_reference_white
                ));
            }
            writer.write_float(self.hdr_reference_white * 5.0);
        }

        if !self.has_adaptive_tone_map_flag {
            return Ok(writer.get_data());
        }

        // smpte_st_2094_50_adaptive_tone_map()

        // baseline_hdr_headroom
        if !(0.0..=6.0).contains(&self.baseline_hdr_headroom_log2) {
            return Err(format!(
                "baseline_hdr_headroom_log2: {} is out of range [0, 6]",
                self.baseline_hdr_headroom_log2
            ));
        }
        writer.write_float(self.baseline_hdr_headroom_log2 * 10000.0);
        // use_reference_white_tone_mapping_flag
        writer.write_bit(self.use_reference_white_tone_mapping_flag);
        if self.use_reference_white_tone_mapping_flag {
            writer.write(0, 7); // reserved
            return Ok(writer.get_data());
        }

        // num_alternate_images
        if self.rules.len() > MAX_RULES_SIZE {
            return Err(format!(
                "num_alternate_images: {} is out of range [0, 4]",
                self.rules.len()
            ));
        }
        writer.write(self.rules.len() as u16, 3);
        // gain_application_space_chromaticities_flag
        let mode = self.gain_application_space_chromaticities_mode();
        writer.write(mode as u16, 2);

        // has_common_mix_params_flag
        let has_common_mix_params_flag =
            self.rules.iter().skip(1).all(|rule| rule.mix == self.rules[0].mix);
        writer.write_bit(has_common_mix_params_flag);

        // has_common_curve_params_flag
        let mut has_common_curve_params_flag = true;
        for rule in self.rules.iter().skip(1) {
            has_common_curve_params_flag &= rule.use_pchip_slope == self.rules[0].use_pchip_slope
                && rule.curve.len() == self.rules[0].curve.len();
            for i in 0..rule.curve.len() {
                if has_common_curve_params_flag {
                    has_common_curve_params_flag &= rule.curve[i].x == self.rules[0].curve[i].x;
                }
            }
        }
        writer.write_bit(has_common_curve_params_flag);

        if mode == 3 {
            for r in self.gain_application_space_chromaticities {
                if !(0.0..=1.0).contains(&r) {
                    return Err(format!(
                        "gain_application_space_chromaticities: {} is out of range [0, 1]",
                        r
                    ));
                }
                writer.write_float(r * MIX_PARAM_SCALE);
            }
        }

        for (a, rule) in self.rules.iter().enumerate() {
            // alternate_hdr_headrooms[a]
            if rule.alternate_hdr_headroom_log2 < 0.0 || rule.alternate_hdr_headroom_log2 > 6.0 {
                return Err(format!(
                    "alternate_hdr_headroom_log2: {} is out of range [0, 6] for alternate image {}",
                    rule.alternate_hdr_headroom_log2, a
                ));
            }
            writer.write_float(rule.alternate_hdr_headroom_log2 * 10000.0);

            // smpte_st_2094_50_component_mixing(a)
            if a == 0 || !has_common_mix_params_flag {
                let mix = &rule.mix;
                let component_mixing_type = mix.mix_type();
                writer.write(component_mixing_type as u16, 2);
                if component_mixing_type != 3 {
                    writer.write(0, 6);
                } else {
                    let mix_params =
                        [mix.rgb[0], mix.rgb[1], mix.rgb[2], mix.max, mix.min, mix.component];
                    let mut max_c = 0.0f32;
                    for &c in &mix_params {
                        if !(0.0..=1.0).contains(&c) {
                            return Err(format!("mix component {} is out of range [0, 1]", c));
                        }
                        max_c = max_c.max(c);
                    }

                    // normalize and write has_component_mixing_coefficient_flag.
                    for &c in &mix_params {
                        writer.write_bit(c != 0.0);
                    }
                    for &c in &mix_params {
                        if c != 0.0 {
                            writer.write_float(c / max_c * MIX_PARAM_SCALE);
                        }
                    }
                }
            }

            // smpte_st_2094_50_control_point_curve(a)
            if a == 0 || !has_common_curve_params_flag {
                if rule.curve.is_empty() || rule.curve.len() > MAX_CURVE_SIZE {
                    return Err(format!(
                        "curve size {} is > {} or == 0 for alternate image {}",
                        rule.curve.len(),
                        MAX_CURVE_SIZE,
                        a
                    ));
                }
                writer.write((rule.curve.len() - 1) as u16, 5);
                writer.write_bit(rule.use_pchip_slope);
                writer.write(0, 2);
                for point in rule.curve.iter() {
                    // gain_curve_control_points_x[a][c]
                    writer.write_float(point.x * 1000.0);
                }
            }

            let expected_y_sign =
                if self.baseline_hdr_headroom_log2 < rule.alternate_hdr_headroom_log2 {
                    1.0
                } else {
                    -1.0
                };

            for point in rule.curve.iter() {
                let y_sign = if point.y >= 0.0 { 1.0 } else { -1.0 };
                if y_sign != expected_y_sign && point.y.abs() > 1e-6 {
                    return Err(format!(
                        "Sign of y does not match expected sign {} based on altr headroom {} and baseline headroom {} for altr index {}, point (x={}, y={})",
                        expected_y_sign, rule.alternate_hdr_headroom_log2, self.baseline_hdr_headroom_log2, a, point.x, point.y
                    ));
                }
                // gain_curve_control_points_y[a][c]
                writer.write_float(point.y * y_sign * 10000.0);
            }

            if !rule.use_pchip_slope {
                for point in rule.curve.iter() {
                    // gain_curve_control_points_theta[a][c]
                    writer.write_float((point.m.atan() + PI / 2.0) * 36000.0 / PI);
                }
            }
        }

        Ok(writer.get_data())
    }

    /// Deserializes dynamic metadata from the SMPTE ST 2094-50 binary format.
    fn from_st2094_50(data: &[u8]) -> Result<DynamicMetadata, String> {
        let mut metadata = DynamicMetadata::default();
        let mut reader = BitReader::new(data);

        reader.skip(3)?; // application_version
        let minimum_application_version = reader.read_literal(3)?;
        reader.skip(2)?; // reserved
        if minimum_application_version > 0 {
            return Err("Unsupported minimum_application_version".to_string());
        }

        let has_custom_hdr_ref_white_flag = reader.read_bit()?;
        metadata.has_adaptive_tone_map_flag = reader.read_bit()?;
        reader.skip(6)?; // reserved

        if has_custom_hdr_ref_white_flag {
            let hdr_ref_white_scaled = reader.read_literal(16)?;
            metadata.hdr_reference_white = (hdr_ref_white_scaled as f32).clamp(1.0, 50000.0) / 5.0;
        } else {
            metadata.hdr_reference_white = 203.0;
        }

        if !metadata.has_adaptive_tone_map_flag {
            return Ok(metadata);
        }

        let baseline_hdr_headroom_log2_scaled = reader.read_literal(16)?;
        metadata.baseline_hdr_headroom_log2 =
            (baseline_hdr_headroom_log2_scaled as f32).min(60000.0) / 10000.0;
        metadata.use_reference_white_tone_mapping_flag = reader.read_bit()?;
        if metadata.use_reference_white_tone_mapping_flag {
            reader.skip(7)?;
            metadata.populate_using_rwtm();
            return Ok(metadata);
        }

        let num_alternate_images = std::cmp::min(reader.read_literal(3)? as usize, MAX_RULES_SIZE);
        metadata.rules.resize(num_alternate_images, ToneMappingRule::default());

        let mode = reader.read_literal(2)? as usize;
        let has_common_mix_params_flag = reader.read_bit()?;
        let has_common_curve_params_flag = reader.read_bit()?;

        if mode == 3 {
            for r in 0..8 {
                let scaled = reader.read_literal(16)?;
                metadata.gain_application_space_chromaticities[r] =
                    (scaled as f32).min(50000.0) / 50000.0;
            }
        } else {
            metadata.gain_application_space_chromaticities =
                GAIN_APPLICATION_SPACE_CHROMATICITIES[mode];
        }

        let mut mix_params_0 = [0.0; NUM_MIX_PARAMS];
        let mut num_pts_m1_0 = 0;
        let mut use_pchip_slope_0 = false;
        let mut curve_x_0 = Vec::<f32>::new();

        for a in 0..num_alternate_images {
            let alternate_hdr_headroom_log2_scaled = reader.read_literal(16)?;
            metadata.rules[a].alternate_hdr_headroom_log2 =
                (alternate_hdr_headroom_log2_scaled as f32).min(60000.0) / 10000.0;

            let mix_params;
            if a == 0 || !has_common_mix_params_flag {
                let component_mixing_type = reader.read_literal(2)?;
                if component_mixing_type != 3 {
                    reader.skip(6)?;
                }

                mix_params = match component_mixing_type {
                    0 => [0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
                    1 => [0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
                    2 => {
                        let kw = (0.5 / 3.0) as f32;
                        [kw, kw, kw, 0.5, 0.0, 0.0]
                    }
                    _ => {
                        let mut flags = [false; NUM_MIX_PARAMS];
                        for flag in flags.iter_mut() {
                            *flag = reader.read_bit()?;
                        }
                        let mut params_i = [0; NUM_MIX_PARAMS];
                        let mut sum = 0;
                        for i in 0..NUM_MIX_PARAMS {
                            if flags[i] {
                                params_i[i] = reader.read_literal(16)?.min(MIX_PARAM_SCALE as i32);
                                sum += params_i[i];
                            }
                        }
                        if sum == 0 {
                            return Err("sum_mix_params is 0".to_string());
                        }
                        let mut params = [0.0; NUM_MIX_PARAMS];
                        for i in 0..NUM_MIX_PARAMS {
                            params[i] = (params_i[i] as f64 / sum as f64) as f32;
                        }
                        params
                    }
                };
                if a == 0 {
                    mix_params_0 = mix_params;
                }
            } else {
                mix_params = mix_params_0;
            }
            metadata.rules[a].mix = ComponentMix {
                rgb: [mix_params[0], mix_params[1], mix_params[2]],
                max: mix_params[3],
                min: mix_params[4],
                component: mix_params[5],
            };

            let num_pts_m1;
            if a == 0 || !has_common_curve_params_flag {
                num_pts_m1 = reader.read_literal(5)? as usize;
                metadata.rules[a].use_pchip_slope = reader.read_bit()?;
                reader.skip(2)?;
                metadata.rules[a].curve.resize(num_pts_m1 + 1, ControlPoint::default());
                for pt in metadata.rules[a].curve.iter_mut() {
                    let x_scaled = reader.read_literal(16)?;
                    pt.x = (x_scaled as f32).min(64000.0) / 1000.0;
                }
                if a == 0 {
                    num_pts_m1_0 = num_pts_m1;
                    use_pchip_slope_0 = metadata.rules[a].use_pchip_slope;
                    for point in metadata.rules[a].curve.iter() {
                        curve_x_0.push(point.x);
                    }
                }
            } else {
                num_pts_m1 = num_pts_m1_0;
                metadata.rules[a].use_pchip_slope = use_pchip_slope_0;
                metadata.rules[a].curve.resize(num_pts_m1 + 1, ControlPoint::default());
                for (i, x) in curve_x_0.iter().enumerate() {
                    metadata.rules[a].curve[i].x = *x;
                }
            }

            let sign = if metadata.baseline_hdr_headroom_log2
                < metadata.rules[a].alternate_hdr_headroom_log2
            {
                1.0
            } else {
                -1.0
            };
            for i in 0..=num_pts_m1 {
                let y_scaled = reader.read_literal(16)?;
                metadata.rules[a].curve[i].y = (y_scaled as f32).min(60000.0) / 10000.0 * sign;
            }
            if !metadata.rules[a].use_pchip_slope {
                for i in 0..=num_pts_m1 {
                    let theta_scaled = reader.read_literal(16)?;
                    metadata.rules[a].curve[i].m =
                        ((theta_scaled.clamp(1, 35999) as f32) * PI / 36000.0 - PI / 2.0).tan();
                }
            }
        }

        Ok(metadata)
    }
}

/// Foreign function interface wrapper for `DynamicMetadata::populate_using_rwtm`.
pub fn dynamic_metadata_populate_using_rwtm(metadata: &mut DynamicMetadata) {
    metadata.populate_using_rwtm();
}

/// Bit manipulation utility for writing packed bitstreams.
struct BitWriter {
    /// The output byte buffer accumulating the packed bitstream.
    buffer: Vec<u8>,
    /// Temporary bit buffer holding unaligned bits before writing to the byte buffer.
    bit_buffer: u64,
    /// The current number of valid bits stored in `bit_buffer`.
    bit_count: i8,
}

impl BitWriter {
    fn new() -> Self {
        Self { buffer: Vec::new(), bit_buffer: 0, bit_count: 0 }
    }

    fn write(&mut self, value: u16, num_bits: i8) {
        if num_bits <= 0 {
            return;
        }
        self.bit_count += num_bits;
        self.bit_buffer = (self.bit_buffer << num_bits) | (value as u64 & ((1u64 << num_bits) - 1));
        while self.bit_count >= 8 {
            let byte = ((self.bit_buffer >> (self.bit_count - 8)) & 0xFF) as u8;
            self.buffer.push(byte);
            self.bit_count -= 8;
        }
    }

    /// Writes a float that fits in the u16 range.
    fn write_float(&mut self, value: f32) {
        assert!(value >= 0.0);
        self.write(value.round() as u16, 16);
    }

    fn write_bit(&mut self, value: bool) {
        self.write(value as u16, 1);
    }

    fn get_data(mut self) -> Vec<u8> {
        if self.bit_count > 0 {
            // Pad with zeros.
            let byte = ((self.bit_buffer << (8 - self.bit_count)) & 0xFF) as u8;
            self.buffer.push(byte);
            self.bit_buffer = 0;
            self.bit_count = 0;
        }
        self.buffer
    }
}

/// Basic bit reader utility used for deserialization of packed bitstreams.
struct BitReader<'a> {
    /// The input packed bitstream slice being read.
    data: &'a [u8],
    /// Current byte offset within the input data.
    byte_pos: usize,
    /// Current bit offset within the current byte.
    bit_pos: i8,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, byte_pos: 0, bit_pos: 0 }
    }

    fn read_literal(&mut self, num_bits: i8) -> Result<i32, String> {
        let mut value: i32 = 0;
        for _ in 0..num_bits {
            if self.byte_pos >= self.data.len() {
                return Err("Reading past end of data".to_string());
            }
            let byte = self.data[self.byte_pos];
            value = (value << 1) | (((byte >> (7 - self.bit_pos)) & 1) as i32);
            self.bit_pos += 1;
            if self.bit_pos == 8 {
                self.byte_pos += 1;
                self.bit_pos = 0;
            }
        }
        Ok(value)
    }

    fn read_bit(&mut self) -> Result<bool, String> {
        Ok(self.read_literal(1)? != 0)
    }

    fn skip(&mut self, num_bits: i8) -> Result<(), String> {
        for _ in 0..num_bits {
            if self.byte_pos >= self.data.len() {
                return Err("Reading past end of data".to_string());
            }
            self.bit_pos += 1;
            if self.bit_pos == 8 {
                self.byte_pos += 1;
                self.bit_pos = 0;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_valid_metadata() {
        let metadata = DynamicMetadata {
            hdr_reference_white: 1000.0,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.3, 0.3],
            ..Default::default()
        };
        expect_true!(metadata.is_valid());
    }

    #[gtest]
    fn test_invalid_reference_white() {
        let mut metadata = DynamicMetadata {
            hdr_reference_white: 0.0,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.3, 0.3],
            ..Default::default()
        };
        expect_false!(metadata.is_valid());

        metadata.hdr_reference_white = 10001.0;
        expect_false!(metadata.is_valid());
    }

    #[gtest]
    fn test_white_point_outside() {
        let metadata = DynamicMetadata {
            hdr_reference_white: 1000.0,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.6, 0.6],
            ..Default::default()
        };
        expect_false!(metadata.is_valid());
    }

    #[gtest]
    fn test_tone_mapping_rule_invalid_curve_x_order() {
        let rule = ToneMappingRule {
            alternate_hdr_headroom_log2: 1.0,
            curve: vec![
                ControlPoint { x: 2.0, y: 1.0, m: 0.0 },
                ControlPoint { x: 1.0, y: 2.0, m: 0.0 }, // x decreases
            ],
            ..Default::default()
        };
        expect_false!(rule.is_valid());
    }

    #[gtest]
    fn test_tone_mapping_rule_invalid_curve_same_x_different_y() {
        let rule = ToneMappingRule {
            alternate_hdr_headroom_log2: 1.0,
            curve: vec![
                ControlPoint { x: 1.0, y: 1.0, m: 0.0 },
                ControlPoint { x: 1.0, y: 2.0, m: 0.0 }, // Same x, different y
            ],
            ..Default::default()
        };
        expect_false!(rule.is_valid());
    }

    #[gtest]
    fn test_gain_curve_sign_should_constraint() {
        let mut metadata = DynamicMetadata {
            hdr_reference_white: 1000.0,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.3, 0.3],
            ..Default::default()
        };

        let rule = ToneMappingRule {
            alternate_hdr_headroom_log2: 3.0,
            mix: ComponentMix { max: 1.0, ..Default::default() },
            curve: vec![ControlPoint { x: 1.0, y: -0.1, m: 0.0 }],
            ..Default::default()
        };
        metadata.rules.push(rule);

        expect_false!(metadata.is_valid());
    }

    #[gtest]
    fn test_headroom_precision_rounding() {
        // baseline = 2.0 -> encoded = 20000
        // alt = 2.000001 -> encoded = 20000
        // They should be considered equal and thus invalid.
        let mut metadata = DynamicMetadata {
            hdr_reference_white: 1000.0,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.3, 0.3],
            ..Default::default()
        };
        metadata
            .rules
            .push(ToneMappingRule { alternate_hdr_headroom_log2: 2.000001, ..Default::default() });
        expect_false!(metadata.is_valid());
    }
}
