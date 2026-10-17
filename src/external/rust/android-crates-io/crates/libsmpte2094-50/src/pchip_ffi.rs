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
// CXX bridge for pchip_rs.
use crate::pchip::{
    create_subsampled_pchip, GainCurve, GainCurveResult, PchipInterpolator,
    PchipInterpolatorResult, PchipSlopesResult, ReverseInterpolateResult,
    pchip_slopes_ffi,
};

#[cxx::bridge(namespace = "pchip_ffi")]
pub mod ffi {
    extern "Rust" {
        type PchipInterpolator;
        fn clone(self: &PchipInterpolator) -> Box<PchipInterpolator>;
        fn interpolate(self: &PchipInterpolator, xi: f32) -> f32;
        #[cxx_name = "reverse_interpolate"]
        fn pchip_interpolator_reverse_interpolate(
            self: &PchipInterpolator,
            yi: f32,
        ) -> Box<ReverseInterpolateResult>;
        #[cxx_name = "x"]
        fn pchip_interpolator_x(self: &PchipInterpolator) -> &[f32];
        #[cxx_name = "y"]
        fn pchip_interpolator_y(self: &PchipInterpolator) -> &[f32];
        #[cxx_name = "slopes"]
        fn pchip_interpolator_slopes(self: &PchipInterpolator) -> &[f32];

        type GainCurve;
        fn clone(self: &GainCurve) -> Box<GainCurve>;
        #[cxx_name = "interpolate"]
        fn gain_curve_interpolate(self: &GainCurve, xi: f32) -> f32;
        #[cxx_name = "reverse_interpolate"]
        fn gain_curve_reverse_interpolate(
            self: &GainCurve,
            yi: f32,
        ) -> Box<ReverseInterpolateResult>;

        type ReverseInterpolateResult;
        fn clone(self: &ReverseInterpolateResult) -> Box<ReverseInterpolateResult>;
        fn xi(self: &ReverseInterpolateResult) -> f32;
        fn success(self: &ReverseInterpolateResult) -> bool;

        type PchipSlopesResult;
        fn clone(self: &PchipSlopesResult) -> Box<PchipSlopesResult>;
        fn success(self: &PchipSlopesResult) -> bool;
        #[cxx_name = "get_slopes"]
        fn pchip_slopes_result_slopes(self: &PchipSlopesResult) -> &[f32];
        #[cxx_name = "get_error_message"]
        fn pchip_slopes_result_error(self: &PchipSlopesResult) -> &str;

        type PchipInterpolatorResult;
        fn clone(self: &PchipInterpolatorResult) -> Box<PchipInterpolatorResult>;
        fn success(self: &PchipInterpolatorResult) -> bool;
        #[cxx_name = "interp"]
        fn pchip_interpolator_result_interp(self: &PchipInterpolatorResult) -> Box<PchipInterpolator>;
        #[cxx_name = "get_error_message"]
        fn pchip_interpolator_result_error(self: &PchipInterpolatorResult) -> &str;

        type GainCurveResult;
        fn clone(self: &GainCurveResult) -> Box<GainCurveResult>;
        fn success(self: &GainCurveResult) -> bool;
        #[cxx_name = "curve"]
        fn gain_curve_result_curve(self: &GainCurveResult) -> Box<GainCurve>;
        #[cxx_name = "get_error_message"]
        fn gain_curve_result_error(self: &GainCurveResult) -> &str;

        #[cxx_name = "pchip_slopes_ffi"]
        fn pchip_slopes_ffi_bridge(x: &[f32], y: &[f32]) -> Box<PchipSlopesResult>;
        #[cxx_name = "create_subsampled_pchip_ffi"]
        fn create_subsampled_pchip_ffi_bridge(
            x: &[f32],
            y: &[f32],
            num_control_points: usize,
        ) -> Box<PchipInterpolatorResult>;
        #[cxx_name = "pchip_interpolator_create_ffi"]
        fn pchip_interpolator_create_ffi_bridge(x: &[f32], y: &[f32]) -> Box<PchipInterpolatorResult>;
        #[cxx_name = "pchip_interpolator_create_with_slopes_ffi"]
        fn pchip_interpolator_create_with_slopes_ffi_bridge(x: &[f32], y: &[f32], slopes: &[f32]) -> Box<PchipInterpolatorResult>;
        #[cxx_name = "gain_curve_create_ffi"]
        fn gain_curve_create_ffi_bridge(x: &[f32], y: &[f32]) -> Box<GainCurveResult>;
        #[cxx_name = "gain_curve_create_with_slopes_ffi"]
        fn gain_curve_create_with_slopes_ffi_bridge(x: &[f32], y: &[f32], slopes: &[f32]) -> Box<GainCurveResult>;
    }
}

pub fn pchip_slopes_ffi_bridge(x: &[f32], y: &[f32]) -> Box<PchipSlopesResult> {
    Box::new(pchip_slopes_ffi(x, y))
}

pub fn create_subsampled_pchip_ffi_bridge(
    x: &[f32],
    y: &[f32],
    num_control_points: usize,
) -> Box<PchipInterpolatorResult> {
    match create_subsampled_pchip(x, y, num_control_points) {
        Ok(interp) => Box::new(PchipInterpolatorResult {
            interp,
            success: true,
            error_message: String::new(),
        }),
        Err(e) => Box::new(PchipInterpolatorResult {
            interp: PchipInterpolator::default(),
            success: false,
            error_message: e,
        }),
    }
}

pub fn pchip_interpolator_create_ffi_bridge(x: &[f32], y: &[f32]) -> Box<PchipInterpolatorResult> {
    Box::new(PchipInterpolator::create_ffi(x, y))
}

pub fn pchip_interpolator_create_with_slopes_ffi_bridge(x: &[f32], y: &[f32], slopes: &[f32]) -> Box<PchipInterpolatorResult> {
    Box::new(PchipInterpolator::create_with_slopes_ffi(x, y, slopes))
}

pub fn gain_curve_create_ffi_bridge(x: &[f32], y: &[f32]) -> Box<GainCurveResult> {
    Box::new(GainCurve::create_ffi(x, y))
}

pub fn gain_curve_create_with_slopes_ffi_bridge(x: &[f32], y: &[f32], slopes: &[f32]) -> Box<GainCurveResult> {
    Box::new(GainCurve::create_with_slopes_ffi(x, y, slopes))
}

impl PchipInterpolator {
    fn clone(&self) -> Box<PchipInterpolator> {
        Box::new(Clone::clone(self))
    }
    fn pchip_interpolator_reverse_interpolate(&self, yi: f32) -> Box<ReverseInterpolateResult> {
        Box::new(self.reverse_interpolate(yi))
    }
    fn pchip_interpolator_x(&self) -> &[f32] {
        &self.x
    }
    fn pchip_interpolator_y(&self) -> &[f32] {
        &self.y
    }
    fn pchip_interpolator_slopes(&self) -> &[f32] {
        &self.slopes
    }
}

impl GainCurve {
    fn clone(&self) -> Box<GainCurve> {
        Box::new(Clone::clone(self))
    }
    fn gain_curve_interpolate(&self, xi: f32) -> f32 {
        self.interpolate(xi)
    }
    fn gain_curve_reverse_interpolate(&self, yi: f32) -> Box<ReverseInterpolateResult> {
        Box::new(self.reverse_interpolate(yi))
    }
}

impl ReverseInterpolateResult {
    fn clone(&self) -> Box<ReverseInterpolateResult> {
        Box::new(Clone::clone(self))
    }
    fn xi(&self) -> f32 {
        self.xi
    }
    fn success(&self) -> bool {
        self.success
    }
}

impl Clone for PchipSlopesResult {
    fn clone(&self) -> Self {
        Self {
            slopes: self.slopes.clone(),
            success: self.success,
            error_message: self.error_message.clone(),
        }
    }
}

impl PchipSlopesResult {
    fn clone(&self) -> Box<PchipSlopesResult> {
        Box::new(Clone::clone(self))
    }
    fn success(&self) -> bool {
        self.success
    }
    fn pchip_slopes_result_slopes(&self) -> &[f32] {
        &self.slopes
    }
    fn pchip_slopes_result_error(&self) -> &str {
        &self.error_message
    }
}

impl Clone for PchipInterpolatorResult {
    fn clone(&self) -> Self {
        Self {
            interp: Clone::clone(&self.interp),
            success: self.success,
            error_message: self.error_message.clone(),
        }
    }
}

impl PchipInterpolatorResult {
    fn clone(&self) -> Box<PchipInterpolatorResult> {
        Box::new(Clone::clone(self))
    }
    fn success(&self) -> bool {
        self.success
    }
    fn pchip_interpolator_result_interp(&self) -> Box<PchipInterpolator> {
        Box::new(Clone::clone(&self.interp))
    }
    fn pchip_interpolator_result_error(&self) -> &str {
        &self.error_message
    }
}

impl Clone for GainCurveResult {
    fn clone(&self) -> Self {
        Self {
            curve: Clone::clone(&self.curve),
            success: self.success,
            error_message: self.error_message.clone(),
        }
    }
}

impl GainCurveResult {
    fn clone(&self) -> Box<GainCurveResult> {
        Box::new(Clone::clone(self))
    }
    fn success(&self) -> bool {
        self.success
    }
    fn gain_curve_result_curve(&self) -> Box<GainCurve> {
        Box::new(Clone::clone(&self.curve))
    }
    fn gain_curve_result_error(&self) -> &str {
        &self.error_message
    }
}
