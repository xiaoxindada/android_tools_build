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
// Tone mapping functions according to SMPTE ST 2094-50 Section 6.

use crate::pchip::GainCurve;
use crate::utils::{ComponentMix, DynamicMetadata};

/// RGB pixel in gain application color space.
pub type Rgb = [f32; 3];

/// Evaluates component mixing according to SMPTE ST 2094-50 Section 6.4.
#[inline]
pub fn evaluate_component_mixing(mix: &ComponentMix, c: Rgb) -> Rgb {
    let ksum = mix.rgb[0] + mix.rgb[1] + mix.rgb[2] + mix.max + mix.min + mix.component;
    if ksum <= 0.0 {
        return [0.0, 0.0, 0.0];
    }
    let max_c = c[0].max(c[1]).max(c[2]);
    let min_c = c[0].min(c[1]).min(c[2]);
    let luma = mix.rgb[0] * c[0] + mix.rgb[1] * c[1] + mix.rgb[2] * c[2];
    let base = mix.max * max_c + mix.min * min_c + luma;
    let inv_ksum = 1.0 / ksum;
    [
        (base + mix.component * c[0]) * inv_ksum,
        (base + mix.component * c[1]) * inv_ksum,
        (base + mix.component * c[2]) * inv_ksum,
    ]
}

/// A contributing tone mapping rule representation.
#[derive(Clone, Debug)]
pub enum ActiveRule {
    /// Zero color gain function (applied at baseline HDR headroom).
    ZeroGain,
    /// Curve rule with associated component mixing and gain curve.
    Curve { mix: ComponentMix, curve: GainCurve },
}

impl ActiveRule {
    #[inline]
    pub fn evaluate(&self, c: Rgb) -> Rgb {
        match self {
            ActiveRule::ZeroGain => [0.0, 0.0, 0.0],
            ActiveRule::Curve { mix, curve } => {
                let m = evaluate_component_mixing(mix, c);
                if mix.component == 0.0 {
                    let g = curve.interpolate(m[0]);
                    [g, g, g]
                } else {
                    [
                        curve.interpolate(m[0]),
                        curve.interpolate(m[1]),
                        curve.interpolate(m[2]),
                    ]
                }
            }
        }
    }
}

/// Headroom-adaptive tone mapper context.
#[derive(Clone, Debug)]
pub struct ToneMapper {
    rule_0: ActiveRule,
    rule_1: ActiveRule,
    weight_0: f32,
    weight_1: f32,
    is_identity: bool,
}

impl ToneMapper {
    /// Creates a no-op identity tone mapper.
    pub fn identity() -> Self {
        Self {
            rule_0: ActiveRule::ZeroGain,
            rule_1: ActiveRule::ZeroGain,
            weight_0: 1.0,
            weight_1: 0.0,
            is_identity: true,
        }
    }

    /// Creates a new `ToneMapper` from metadata and target HDR headroom (in log2).
    /// The target HDR headroom is the ratio between the display's maximum light
    /// level over SDR white light level.
    pub fn new(metadata: &DynamicMetadata, target_headroom_log2: f32) -> Result<Self, String> {
        if !metadata.is_valid() {
            return Err("Invalid metadata".to_string());
        }

        if !metadata.has_adaptive_tone_map_flag {
            // In this case, "the headroom-adaptive tone mapping can be decided by the output system."
            // Just return identity.
            return Ok(Self::identity());
        }

        let mut meta = metadata.clone();
        meta.populate_implicit_parameters()?;

        #[derive(Clone, Copy)]
        struct RuleCandidate {
            headroom: f32,
            rule_index: Option<usize>, // None indicates implicit zero-gain baseline rule
        }

        let mut candidates = Vec::with_capacity(meta.rules.len() + 1);
        for (i, rule) in meta.rules.iter().enumerate() {
            candidates.push(RuleCandidate {
                headroom: rule.alternate_hdr_headroom_log2,
                rule_index: Some(i),
            });
        }
        candidates.push(RuleCandidate {
            headroom: meta.baseline_hdr_headroom_log2,
            rule_index: None,
        });

        candidates.sort_by(|a, b| {
            a.headroom
                .partial_cmp(&b.headroom)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        if candidates.len() == 1 {
            return Ok(Self::identity());
        }

        // Find bracketing rules.
        let idx_1 = candidates
            .iter()
            .position(|c| c.headroom >= target_headroom_log2)
            .unwrap_or(candidates.len() - 1)
            .max(1);
        let idx_0 = idx_1 - 1;

        let h_0 = candidates[idx_0].headroom;
        let h_1 = candidates[idx_1].headroom;

        let create_active_rule = |cand: RuleCandidate| -> Result<ActiveRule, String> {
            match cand.rule_index {
                None => Ok(ActiveRule::ZeroGain),
                Some(idx) => {
                    let rule = &meta.rules[idx];
                    let x: Vec<f32> = rule.curve.iter().map(|p| p.x).collect();
                    let y: Vec<f32> = rule.curve.iter().map(|p| p.y).collect();
                    let slopes: Vec<f32> = rule.curve.iter().map(|p| p.m).collect();
                    let curve = GainCurve::create_with_slopes(x, y, slopes);
                    Ok(ActiveRule::Curve {
                        mix: rule.mix,
                        curve,
                    })
                }
            }
        };

        let rule_0 = create_active_rule(candidates[idx_0])?;
        let rule_1 = create_active_rule(candidates[idx_1])?;

        let (weight_0, weight_1) = if h_1 > h_0 {
            let w_1 = ((target_headroom_log2 - h_0) / (h_1 - h_0)).clamp(0.0, 1.0);
            (1.0 - w_1, w_1)
        } else {
            (1.0, 0.0)
        };

        let is_identity = match (&rule_0, &rule_1) {
            (ActiveRule::ZeroGain, ActiveRule::ZeroGain) => true,
            (ActiveRule::ZeroGain, _) if weight_1 == 0.0 => true,
            (_, ActiveRule::ZeroGain) if weight_0 == 0.0 => true,
            _ => false,
        };

        Ok(Self {
            rule_0,
            rule_1,
            weight_0,
            weight_1,
            is_identity,
        })
    }

    /// Returns whether this tone mapper is a no-op identity transform.
    pub fn is_identity(&self) -> bool {
        self.is_identity
    }

    /// Tone maps a single RGB pixel. The RGB values must be in gain application
    /// color space, i.e. linear SDR-relative in the gain application
    /// chromaticities, see SMPTE ST 2094-50 Section A.2 titled
    /// "Conversion to gain application color space". Essentially:
    /// encoded values -> apply EOTF -> divide by hdr_reference_white
    /// -> convert to gain_application_space_chromaticities.
    #[inline]
    pub fn tone_map_pixel(&self, color: Rgb) -> Rgb {
        if self.is_identity {
            return color;
        }

        let mut log_gains = [0.0f32; 3];
        if self.weight_0 > 0.0 {
            let g_0 = self.rule_0.evaluate(color);
            for c in 0..3 {
                log_gains[c] += self.weight_0 * g_0[c];
            }
        }
        if self.weight_1 > 0.0 {
            let g_1 = self.rule_1.evaluate(color);
            for c in 0..3 {
                log_gains[c] += self.weight_1 * g_1[c];
            }
        }

        [
            color[0] * log_gains[0].exp2(),
            color[1] * log_gains[1].exp2(),
            color[2] * log_gains[2].exp2(),
        ]
    }

    /// In-place tone maps an interleaved float RGB buffer.
    pub fn tone_map_buffer(&self, buffer: &mut [f32]) -> Result<(), String> {
        if buffer.len() % 3 != 0 {
            return Err(format!(
                "Buffer length must be a multiple of 3 (got {})",
                buffer.len()
            ));
        }

        if self.is_identity {
            return Ok(());
        }

        for chunk in buffer.chunks_exact_mut(3) {
            let rgb = [chunk[0], chunk[1], chunk[2]];
            let mapped = self.tone_map_pixel(rgb);
            chunk[0] = mapped[0];
            chunk[1] = mapped[1];
            chunk[2] = mapped[2];
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{ControlPoint, ToneMappingRule};
    use googletest::prelude::*;

    const EPSILON: f32 = 1e-4;

    #[gtest]
    fn test_component_mixing_preserves_neutral_gray() {
        let mix = ComponentMix {
            rgb: [0.2, 0.7, 0.1],
            max: 0.5,
            min: 0.1,
            component: 0.1,
        };
        // By definition in Section 6.4.1, component mixing on neutral gray values is identity.
        let gray_sub = [0.42, 0.42, 0.42];
        let mixed_sub = evaluate_component_mixing(&mix, gray_sub);
        assert!((mixed_sub[0] - 0.42).abs() < EPSILON);
        assert!((mixed_sub[1] - 0.42).abs() < EPSILON);
        assert!((mixed_sub[2] - 0.42).abs() < EPSILON);

        let gray_hdr = [2.5, 2.5, 2.5];
        let mixed_hdr = evaluate_component_mixing(&mix, gray_hdr);
        assert!((mixed_hdr[0] - 2.5).abs() < EPSILON);
        assert!((mixed_hdr[1] - 2.5).abs() < EPSILON);
        assert!((mixed_hdr[2] - 2.5).abs() < EPSILON);
    }

    #[gtest]
    fn test_component_mixing_with_component_weight() {
        let mix = ComponentMix {
            rgb: [0.3, 0.6, 0.1],
            max: 0.0,
            min: 0.0,
            component: 0.5,
        };
        // For c = [1.0, 0.0, 0.0]:
        // luma = 0.3 * 1.0 = 0.3
        // base = 0.3
        // ksum = 0.3 + 0.6 + 0.1 + 0 + 0 + 0.5 = 1.5
        // m[0] = (0.3 + 0.5 * 1.0) / 1.5 = 0.8 / 1.5 ≈ 0.53333336
        // m[1] = (0.3 + 0.5 * 0.0) / 1.5 = 0.3 / 1.5 = 0.2
        // m[2] = (0.3 + 0.5 * 0.0) / 1.5 = 0.3 / 1.5 = 0.2
        let c = [1.0, 0.0, 0.0];
        let m = evaluate_component_mixing(&mix, c);
        assert!((m[0] - 0.8 / 1.5).abs() < EPSILON);
        assert!((m[1] - 0.2).abs() < EPSILON);
        assert!((m[2] - 0.2).abs() < EPSILON);
    }

    #[gtest]
    fn test_component_mixing_max_and_min() {
        let c = [0.2, 0.8, 0.5];

        // max(R, G, B) mixing (used in RWTM).
        let mix_max = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 1.0,
            min: 0.0,
            component: 0.0,
        };
        let m_max = evaluate_component_mixing(&mix_max, c);
        assert!((m_max[0] - 0.8).abs() < EPSILON);
        assert!((m_max[1] - 0.8).abs() < EPSILON);
        assert!((m_max[2] - 0.8).abs() < EPSILON);

        // min(R, G, B) mixing.
        let mix_min = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 1.0,
            component: 0.0,
        };
        let m_min = evaluate_component_mixing(&mix_min, c);
        assert!((m_min[0] - 0.2).abs() < EPSILON);
        assert!((m_min[1] - 0.2).abs() < EPSILON);
        assert!((m_min[2] - 0.2).abs() < EPSILON);

        // Invalid mix with zero sum returns [0, 0, 0].
        let mix_zero = ComponentMix::default();
        let m_zero = evaluate_component_mixing(&mix_zero, c);
        assert_eq!(m_zero, [0.0, 0.0, 0.0]);
    }

    #[gtest]
    fn test_disabled_adaptive_tone_mapping_is_identity() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: false, // disabled
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        // Even with a rule present, disabled flag forces identity.
        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 2.0;
        rule.use_pchip_slope = true;
        rule.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 1.0,
                m: 0.0,
            },
            ControlPoint {
                x: 64.0,
                y: 1.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule);

        let mapper = ToneMapper::new(&agtm, 2.0).unwrap();
        assert!(mapper.is_identity());
        let out = mapper.tone_map_pixel([0.5, 0.5, 0.5]);
        assert_eq!(out, [0.5, 0.5, 0.5]);
    }

    #[gtest]
    fn test_linear_gain_rule_applies_expected_gain() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 2.0;
        rule.use_pchip_slope = true;
        rule.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 1.0,
                m: 0.0,
            },
            ControlPoint {
                x: 64.0,
                y: 1.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule);

        // Target headroom 2.0: full interpolation to the alternate rule (multiplier = 2^1 = 2.0).
        let mapper_2 = ToneMapper::new(&agtm, 2.0).unwrap();
        let out_2 = mapper_2.tone_map_pixel([0.5, 0.5, 0.5]);
        assert!((out_2[0] - 1.0).abs() < EPSILON);
        assert!((out_2[1] - 1.0).abs() < EPSILON);
        assert!((out_2[2] - 1.0).abs() < EPSILON);

        // Target headroom 1.0: halfway between baseline 0.0 and alternate 2.0 (weight = 0.5).
        // Expected gain = 2^(0.5 * 1.0) = sqrt(2) ≈ 1.41421356.
        // Expected output = 0.5 * sqrt(2) ≈ 0.70710678.
        let mapper_1 = ToneMapper::new(&agtm, 1.0).unwrap();
        let out_1 = mapper_1.tone_map_pixel([0.5, 0.5, 0.5]);
        assert!((out_1[0] - 0.707107).abs() < EPSILON);
        assert!((out_1[1] - 0.707107).abs() < EPSILON);
        assert!((out_1[2] - 0.707107).abs() < EPSILON);

        // In-place buffer mapping.
        let mut buffer = vec![0.5, 0.5, 0.5];
        assert!(mapper_1.tone_map_buffer(&mut buffer).is_ok());
        assert!((buffer[0] - 0.707107).abs() < EPSILON);
        assert!((buffer[1] - 0.707107).abs() < EPSILON);
        assert!((buffer[2] - 0.707107).abs() < EPSILON);
    }

    #[gtest]
    fn test_invalid_metadata_returns_error() {
        let mut agtm = DynamicMetadata::default();
        agtm.baseline_hdr_headroom_log2 = 99.0; // Out of range [0, 6]
        assert!(ToneMapper::new(&agtm, 1.0).is_err());
    }

    #[gtest]
    fn test_buffer_length_must_be_multiple_of_three() {
        let agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };
        let mapper = ToneMapper::new(&agtm, 0.0).unwrap();
        let mut invalid_buffer = vec![0.5, 0.5, 0.5, 0.5];
        assert!(mapper.tone_map_buffer(&mut invalid_buffer).is_err());
    }

    #[gtest]
    fn test_empty_buffer_succeeds() {
        let agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };
        let mapper = ToneMapper::new(&agtm, 0.0).unwrap();
        let mut empty_vec: Vec<f32> = Vec::new();
        assert!(mapper.tone_map_buffer(&mut empty_vec).is_ok());
    }

    #[gtest]
    fn test_zero_component_weight_shares_gain() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 1.0;
        rule.use_pchip_slope = true;
        rule.mix = ComponentMix {
            rgb: [0.2627, 0.6780, 0.0593],
            max: 0.0,
            min: 0.0,
            component: 0.0,
        };
        rule.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 0.0,
                m: 0.0,
            },
            ControlPoint {
                x: 1.0,
                y: 1.0,
                m: 1.0,
            },
        ];
        agtm.rules.push(rule);

        let mapper = ToneMapper::new(&agtm, 1.0).unwrap();

        // Non-gray colored input:
        let c = [0.8, 0.4, 0.2];
        let out = mapper.tone_map_pixel(c);

        // Mixed luma = 0.2627 * 0.8 + 0.6780 * 0.4 + 0.0593 * 0.2 = 0.49322.
        // Gain = 2^0.49322 ≈ 1.4075986 applied equally to all channels.
        let expected_multiplier = 0.49322f32.exp2();
        assert!((out[0] - c[0] * expected_multiplier).abs() < EPSILON);
        assert!((out[1] - c[1] * expected_multiplier).abs() < EPSILON);
        assert!((out[2] - c[2] * expected_multiplier).abs() < EPSILON);

        // Since the same gain is shared across channels, color ratios are preserved:
        assert!((out[0] / out[1] - c[0] / c[1]).abs() < EPSILON);
        assert!((out[0] / out[2] - c[0] / c[2]).abs() < EPSILON);
    }

    #[gtest]
    fn test_hdr_highlight_inputs_larger_than_one() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 0.0;
        rule.use_pchip_slope = true;
        rule.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        // Compressive tone mapping curve: (0, 0), (1.0, -0.5), (4.0, -1.0)
        rule.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 0.0,
                m: 0.0,
            },
            ControlPoint {
                x: 1.0,
                y: -0.5,
                m: 0.0,
            },
            ControlPoint {
                x: 4.0,
                y: -1.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule);

        let mapper = ToneMapper::new(&agtm, 0.0).unwrap();

        // Input highlight pixel with values > 1.0 (e.g. 4.0).
        // At x = 4.0, y = -1.0 in log2 space, gain is 2^(-1) = 0.5.
        // Output should be 4.0 * 0.5 = 2.0.
        let out = mapper.tone_map_pixel([4.0, 4.0, 4.0]);
        assert!((out[0] - 2.0).abs() < EPSILON);
        assert!((out[1] - 2.0).abs() < EPSILON);
        assert!((out[2] - 2.0).abs() < EPSILON);

        // Buffer mapping with SDR-relative HDR values > 1.0
        let mut hdr_buffer = vec![1.0, 2.0, 4.0];
        assert!(mapper.tone_map_buffer(&mut hdr_buffer).is_ok());
        // x = 1.0 -> y = -0.5 -> gain = 2^(-0.5) -> 1.0 / sqrt(2) ≈ 0.70710678
        assert!((hdr_buffer[0] - 0.707107).abs() < EPSILON);
        // x = 4.0 -> y = -1.0 -> gain = 0.5 -> 4.0 * 0.5 = 2.0
        assert!((hdr_buffer[2] - 2.0).abs() < EPSILON);
    }

    #[gtest]
    fn test_logarithmic_extrapolation_rolls_off_highlights() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 2.0;
        rule.use_pchip_slope = true;
        rule.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        // Control points end at x = 1.0, y = 0.0
        rule.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 0.0,
                m: 0.0,
            },
            ControlPoint {
                x: 1.0,
                y: 0.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule);

        let mapper = ToneMapper::new(&agtm, 2.0).unwrap();

        // According to Section 6.5.3, for x > x_{N-1}:
        // gain(x) = y_{N-1} + log2(x_{N-1} / x)
        // Mapped value = x * 2^gain(x) = x_{N-1} * 2^y_{N-1} = 1.0 * 2^0 = 1.0.
        // The output should be clamped/rolled-off to exactly 1.0 for all x >= 1.0.
        for highlight_val in [1.0, 1.5, 2.0, 4.0, 8.0] {
            let out = mapper.tone_map_pixel([highlight_val, highlight_val, highlight_val]);
            assert!((out[0] - 1.0).abs() < EPSILON);
            assert!((out[1] - 1.0).abs() < EPSILON);
            assert!((out[2] - 1.0).abs() < EPSILON);
        }
    }

    #[gtest]
    fn test_flat_extrapolation_below_leftmost_control_point() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 2.0;
        rule.use_pchip_slope = true;
        rule.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        // Leftmost control point starts at x = 0.5 with gain y = 1.0 (multiplier 2^1 = 2.0).
        rule.curve = vec![
            ControlPoint {
                x: 0.5,
                y: 1.0,
                m: 0.0,
            },
            ControlPoint {
                x: 2.0,
                y: 0.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule);

        let mapper = ToneMapper::new(&agtm, 2.0).unwrap();

        // For any input x <= x_0 (i.e. x <= 0.5), Section 6.5.2 specifies that gain(x) = y_0 = 1.0.
        // The mapped value is x * 2^1.0 = 2 * x.
        for low_val in [0.0, 0.05, 0.1, 0.25, 0.5] {
            let out = mapper.tone_map_pixel([low_val, low_val, low_val]);
            let expected = low_val * 2.0;
            assert!((out[0] - expected).abs() < EPSILON);
            assert!((out[1] - expected).abs() < EPSILON);
            assert!((out[2] - expected).abs() < EPSILON);
        }
    }

    #[gtest]
    fn test_custom_slopes_when_use_pchip_slope_is_false() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        // Rule with use_pchip_slope = false and custom slopes.
        // Points (0, 0) and (1, 0).
        // With use_pchip_slope = true, PCHIP slopes would be 0, yielding flat y=0 everywhere.
        // With custom slopes m_0 = 2.0, m_1 = -2.0, the Hermite cubic polynomial evaluates at x = 0.5:
        // y(0.5) = 0.125 * 1.0 * 2.0 + (-0.125) * 1.0 * (-2.0) = 0.5.
        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 1.0;
        rule.use_pchip_slope = false;
        rule.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 0.0,
                m: 2.0,
            },
            ControlPoint {
                x: 1.0,
                y: 0.0,
                m: -2.0,
            },
        ];
        agtm.rules.push(rule);

        let mapper = ToneMapper::new(&agtm, 1.0).unwrap();

        // At x = 0.5, gain is 0.5 log2, so output = 0.5 * 2^0.5 ≈ 0.707107.
        let out = mapper.tone_map_pixel([0.5, 0.5, 0.5]);
        assert!((out[0] - 0.707107).abs() < EPSILON);

        // For comparison, verify that with use_pchip_slope = true on the same curve,
        // the slopes are overwritten to 0.0 and gain is 0.0 (output = 0.5).
        let mut pchip_agtm = agtm.clone();
        pchip_agtm.rules[0].use_pchip_slope = true;
        let pchip_mapper = ToneMapper::new(&pchip_agtm, 1.0).unwrap();
        let pchip_out = pchip_mapper.tone_map_pixel([0.5, 0.5, 0.5]);
        assert!((pchip_out[0] - 0.5).abs() < EPSILON);
    }

    #[gtest]
    fn test_interpolation_between_two_alternate_rules() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 0.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        // Rule 1: alternate headroom = 1.0, constant gain = +1.0 in log2 (multiplier = 2.0).
        let mut rule1 = ToneMappingRule::default();
        rule1.alternate_hdr_headroom_log2 = 1.0;
        rule1.use_pchip_slope = true;
        rule1.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule1.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 1.0,
                m: 0.0,
            },
            ControlPoint {
                x: 64.0,
                y: 1.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule1);

        // Rule 2: alternate headroom = 3.0, constant gain = +3.0 in log2 (multiplier = 8.0).
        let mut rule2 = ToneMappingRule::default();
        rule2.alternate_hdr_headroom_log2 = 3.0;
        rule2.use_pchip_slope = true;
        rule2.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule2.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 3.0,
                m: 0.0,
            },
            ControlPoint {
                x: 64.0,
                y: 3.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule2);

        // Target headroom 2.0 is midway between Rule 1 (1.0) and Rule 2 (3.0).
        // It must interpolate between Rule 1 and Rule 2 (NOT baseline 0.0).
        // Expected blended log2 gain = 0.5 * 1.0 + 0.5 * 3.0 = 2.0.
        // Output multiplier = 2^2.0 = 4.0.
        let mapper = ToneMapper::new(&agtm, 2.0).unwrap();
        let in_pixel = [0.2, 0.2, 0.2];
        let out_pixel = mapper.tone_map_pixel(in_pixel);
        assert!((out_pixel[0] - 0.8).abs() < EPSILON);
        assert!((out_pixel[1] - 0.8).abs() < EPSILON);
        assert!((out_pixel[2] - 0.8).abs() < EPSILON);
    }

    #[gtest]
    fn test_baseline_headroom_between_rules() {
        // Baseline headroom = 2.0 (e.g. 800 nits master).
        // Candidate headroom order will be: Rule A (0.0), Baseline (2.0), Rule B (4.0).
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        // Rule A (downward tone mapping to SDR): alternate headroom = 0.0, constant gain = -1.0.
        let mut rule_a = ToneMappingRule::default();
        rule_a.alternate_hdr_headroom_log2 = 0.0;
        rule_a.use_pchip_slope = true;
        rule_a.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule_a.curve = vec![
            ControlPoint {
                x: 0.0,
                y: -1.0,
                m: 0.0,
            },
            ControlPoint {
                x: 64.0,
                y: -1.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule_a);

        // Rule B (upward expansion): alternate headroom = 4.0, constant gain = +1.0.
        let mut rule_b = ToneMappingRule::default();
        rule_b.alternate_hdr_headroom_log2 = 4.0;
        rule_b.use_pchip_slope = true;
        rule_b.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule_b.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 1.0,
                m: 0.0,
            },
            ControlPoint {
                x: 64.0,
                y: 1.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule_b);

        // Target headroom 1.0 (between Rule A at 0.0 and Baseline at 2.0):
        // 50% Rule A (-1.0) + 50% Baseline (0.0) => log2 gain = -0.5 => multiplier = 2^-0.5 ≈ 0.707107.
        let mapper_down = ToneMapper::new(&agtm, 1.0).unwrap();
        let out_down = mapper_down.tone_map_pixel([1.0, 1.0, 1.0]);
        assert!((out_down[0] - 0.707107).abs() < EPSILON);

        // Target headroom 3.0 (between Baseline at 2.0 and Rule B at 4.0):
        // 50% Baseline (0.0) + 50% Rule B (+1.0) => log2 gain = +0.5 => multiplier = 2^0.5 ≈ 1.414214.
        let mapper_up = ToneMapper::new(&agtm, 3.0).unwrap();
        let out_up = mapper_up.tone_map_pixel([1.0, 1.0, 1.0]);
        assert!((out_up[0] - 1.414214).abs() < EPSILON);
    }

    #[gtest]
    fn test_target_headroom_beyond_extremes_clamping() {
        let mut agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            baseline_hdr_headroom_log2: 1.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            ..Default::default()
        };

        let mut rule = ToneMappingRule::default();
        rule.alternate_hdr_headroom_log2 = 2.0;
        rule.use_pchip_slope = true;
        rule.mix = ComponentMix {
            rgb: [0.0, 0.0, 0.0],
            max: 0.0,
            min: 0.0,
            component: 1.0,
        };
        rule.curve = vec![
            ControlPoint {
                x: 0.0,
                y: 1.0,
                m: 0.0,
            },
            ControlPoint {
                x: 64.0,
                y: 1.0,
                m: 0.0,
            },
        ];
        agtm.rules.push(rule);

        // Target headroom 5.0 > max alternate headroom (2.0).
        // Clamps to Rule 1 weight = 1.0, multiplier = 2^1 = 2.0.
        let mapper_high = ToneMapper::new(&agtm, 5.0).unwrap();
        let out_high = mapper_high.tone_map_pixel([0.5, 0.5, 0.5]);
        assert!((out_high[0] - 1.0).abs() < EPSILON);

        // Target headroom 0.0 < min baseline headroom (1.0).
        // Clamps to baseline weight = 1.0, identity multiplier = 1.0.
        let mapper_low = ToneMapper::new(&agtm, 1.0).unwrap();
        let out_low = mapper_low.tone_map_pixel([0.5, 0.5, 0.5]);
        assert!((out_low[0] - 0.5).abs() < EPSILON);
    }

    #[gtest]
    fn test_reference_white_tone_mapping_rwtm() {
        let agtm = DynamicMetadata {
            hdr_reference_white: 203.0,
            has_adaptive_tone_map_flag: true,
            use_reference_white_tone_mapping_flag: true,
            baseline_hdr_headroom_log2: 2.0,
            gain_application_space_chromaticities: [
                0.708, 0.292, 0.17, 0.797, 0.131, 0.046, 0.3127, 0.329,
            ],
            rules: vec![],
        };

        // Target headroom 2.0 equals baseline -> identity mapper.
        let mapper_base = ToneMapper::new(&agtm, 2.0).unwrap();
        assert!(mapper_base.is_identity());

        // Target headroom 0.0 (tone mapping from 4x SDR down to SDR).
        // populate_using_rwtm generates rules and ToneMapper::new initializes successfully.
        let mapper_sdr = ToneMapper::new(&agtm, 0.0).unwrap();
        assert!(!mapper_sdr.is_identity());

        // Reference white at x = 1.0 should tone map without panics or NaNs.
        let out = mapper_sdr.tone_map_pixel([1.0, 1.0, 1.0]);
        assert!(out[0].is_finite() && out[0] > 0.0);
        assert!((out[0] - out[1]).abs() < EPSILON);
        assert!((out[1] - out[2]).abs() < EPSILON);
    }
}
