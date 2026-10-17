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
// PCHIP interpolation functions and related utilities.
use std::cmp::Ordering;

/// Computes the three-point finite difference formula for derivative approximation at the boundary
/// of a non-zero interval, as used in PCHIP interpolation.
fn three_point_finite_difference(h0: f32, h1: f32, s0: f32, s1: f32) -> f32 {
    let num = (2.0 * h0 + h1) * s0 - h0 * s1;
    let den = h0 + h1;
    let mut m = num / den;
    if m.is_sign_negative() != s0.is_sign_negative() {
        m = 0.0;
    } else if m.is_sign_negative() != s1.is_sign_negative() && m.abs() > 3.0 * s0.abs() {
        m = 3.0 * s0;
    }
    m
}

/// Result structure for FFI calls computing PCHIP slopes, containing the slopes or an error message.
#[repr(C)]
pub struct PchipSlopesResult {
    /// The computed PCHIP slope values for each control point.
    pub slopes: Vec<f32>,
    /// True if the slope computation was successful, false otherwise.
    pub success: bool,
    /// Detailed error message if the computation failed.
    pub error_message: String,
}

impl PchipSlopesResult {
    /// Returns the computed slopes if successful.
    pub fn get_slopes(&self) -> &[f32] {
        &self.slopes
    }
    /// Returns the error message if the computation failed.
    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }
}

/// FFI wrapper for `pchip_slopes`, returning a `PchipSlopesResult` for C/C++ interoperability.
pub fn pchip_slopes_ffi(x: &[f32], y: &[f32]) -> PchipSlopesResult {
    match pchip_slopes(x, y) {
        Ok(slopes) => PchipSlopesResult { slopes, success: true, error_message: String::new() },
        Err(e) => PchipSlopesResult { slopes: Vec::new(), success: false, error_message: e },
    }
}



/// Computes the slopes for a piecewise cubic interpolator with the given control points.
///
/// Returns an error if the input vectors have different sizes or if the x values are not
/// monotonically increasing.
pub fn pchip_slopes(x: &[f32], y: &[f32]) -> Result<Vec<f32>, String> {
    if x.len() != y.len() {
        return Err(format!("x and y must have the same size. x: {}, y: {}", x.len(), y.len()));
    }
    let n = x.len();
    if n <= 1 {
        return Ok(vec![0.0; n]);
    }

    for i in 0..n - 1 {
        if x[i + 1] < x[i] {
            return Err(format!(
                "x values must be equal or increasing. x[{}] = {}, x[{}] = {}",
                i,
                x[i],
                i + 1,
                x[i + 1]
            ));
        }
    }

    let mut h = vec![0.0; n - 1];
    let mut s = vec![0.0; n - 1];
    for i in 0..n - 1 {
        h[i] = x[i + 1] - x[i];
        if h[i] != 0.0 {
            s[i] = (y[i + 1] - y[i]) / h[i];
        } else {
            s[i] = 0.0;
        }
    }

    let mut d = vec![0.0; n];
    if n == 2 {
        d[0] = s[0];
        d[1] = s[0];
        return Ok(d);
    }

    for i in 0..n {
        if i > 0 && i < n - 1 && h[i - 1] > 0.0 && h[i] > 0.0 {
            // Interior point.
            if s[i - 1] == 0.0
                || s[i] == 0.0
                || s[i - 1].is_sign_negative() != s[i].is_sign_negative()
            {
                d[i] = 0.0;
            } else {
                let num = 3.0 * (h[i - 1] + h[i]) * s[i - 1] * s[i];
                let den = (2.0 * h[i - 1] + h[i]) * s[i - 1] + (h[i - 1] + 2.0 * h[i]) * s[i];
                d[i] = num / den;
            }
        } else if i < n - 1 && h[i] > 0.0 {
            // Left endpoint of a non-zero interval.
            if i + 1 < n - 1 && h[i + 1] > 0.0 {
                d[i] = three_point_finite_difference(h[i], h[i + 1], s[i], s[i + 1]);
            } else {
                d[i] = s[i];
            }
        } else if i > 0 && h[i - 1] > 0.0 {
            // Right endpoint of a non-zero interval.
            if i > 1 && h[i - 2] > 0.0 {
                d[i] = three_point_finite_difference(h[i - 1], h[i - 2], s[i - 1], s[i - 2]);
            } else {
                d[i] = s[i - 1];
            }
        } else {
            d[i] = 0.0;
        }
    }

    Ok(d)
}

/// A Piecewise Cubic Hermite Interpolating Polynomial (PCHIP) interpolator.
/// Maintains control points `x`, `y`, and precomputed `slopes` to evaluate the interpolating curve.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PchipInterpolator {
    /// Monotonically increasing x-coordinates of the control points.
    pub x: Vec<f32>,
    /// Corresponding y-coordinates of the control points.
    pub y: Vec<f32>,
    /// Precomputed Hermite slopes (derivatives) at each control point.
    pub slopes: Vec<f32>,
}

/// Result structure for reverse interpolation evaluations, indicating the estimated `xi` and whether the search succeeded.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct ReverseInterpolateResult {
    /// The estimated x-coordinate that maps to the target y-value.
    pub xi: f32,
    /// True if a valid matching x-coordinate was successfully found.
    pub success: bool,
}

/// Result structure for FFI calls creating a `PchipInterpolator`, containing the interpolator instance or an error message.
pub struct PchipInterpolatorResult {
    /// The resulting PCHIP interpolator instance if creation succeeded.
    pub interp: PchipInterpolator,
    /// True if the interpolator was successfully created, false otherwise.
    pub success: bool,
    /// Detailed error message if the creation failed.
    pub error_message: String,
}

impl PchipInterpolatorResult {
    /// Returns the error message if the interpolator creation failed.
    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }
}

impl PchipInterpolator {
    /// FFI wrapper for `PchipInterpolator::create`, returning a `PchipInterpolatorResult`.
    pub fn create_ffi(x: &[f32], y: &[f32]) -> PchipInterpolatorResult {
        match Self::create(x.to_vec(), y.to_vec()) {
            Ok(interp) => {
                PchipInterpolatorResult { interp, success: true, error_message: String::new() }
            }
            Err(e) => PchipInterpolatorResult {
                interp: PchipInterpolator::default(),
                success: false,
                error_message: e,
            },
        }
    }

    /// FFI wrapper for `PchipInterpolator::create_with_slopes`, returning a `PchipInterpolatorResult`.
    pub fn create_with_slopes_ffi(x: &[f32], y: &[f32], slopes: &[f32]) -> PchipInterpolatorResult {
        let interp = Self::create_with_slopes(x.to_vec(), y.to_vec(), slopes.to_vec());
        PchipInterpolatorResult { interp, success: true, error_message: String::new() }
    }

    /// Creates a new `PchipInterpolator` with provided control points `x`, `y`, and precomputed `slopes`.
    pub fn create_with_slopes(x: Vec<f32>, y: Vec<f32>, slopes: Vec<f32>) -> Self {
        Self { x, y, slopes }
    }

    /// Creates a `PchipInterpolator` from control points `x` and `y`.
    /// Computes the necessary slopes internally. Returns an error if slope computation fails.
    pub fn create(x: Vec<f32>, y: Vec<f32>) -> Result<Self, String> {
        let slopes = pchip_slopes(&x, &y)?;
        Ok(Self::create_with_slopes(x, y, slopes))
    }

    /// Interpolates the value at a given `xi` using the PCHIP method.
    pub fn interpolate(&self, xi: f32) -> f32 {
        if self.x.is_empty() {
            return 0.0;
        }
        let pos =
            self.x.binary_search_by(
                |val| {
                    if *val <= xi {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                },
            );

        let idx = match pos {
            Ok(i) => i,
            Err(i) => i,
        };

        if idx == 0 {
            return self.y[0];
        }
        if idx == self.x.len() {
            return *self.y.last().unwrap();
        }

        let i = idx - 1;
        let x0 = self.x[i];
        let x1 = self.x[i + 1];
        let y0 = self.y[i];
        let y1 = self.y[i + 1];
        let m0 = self.slopes[i];
        let m1 = self.slopes[i + 1];
        let h = x1 - x0;
        if h == 0.0 {
            return y0;
        }
        let t = (xi - x0) / h;

        let h00 = (1.0f32 + 2.0f32 * t) * (1.0f32 - t) * (1.0f32 - t);
        let h10 = t * (1.0f32 - t) * (1.0f32 - t);
        let h01 = t * t * (3.0f32 - 2.0f32 * t);
        let h11 = t * t * (t - 1.0f32);

        h00 * y0 + h10 * h * m0 + h01 * y1 + h11 * h * m1
    }

    /// Finds an `xi` such that `interpolate(xi)` is approximately equal to `yi`.
    /// This uses a binary search approach within each segment and may return unexpected results
    /// if the gain curve is not monotonic.
    pub fn reverse_interpolate(&self, yi: f32) -> ReverseInterpolateResult {
        if self.x.len() < 2 {
            return ReverseInterpolateResult {
                xi: self.x.first().copied().unwrap_or(0.0),
                success: false,
            };
        }

        // Check if yi is below the minimum or above maximum y-value.
        // If so, return x corresponding to the first occurrence of min_y or max_y.
        let mut min_y = self.y[0];
        let mut max_y = self.y[0];
        let mut min_idx = 0;
        let mut max_idx = 0;
        for (i, &y) in self.y.iter().enumerate().skip(1) {
            if y < min_y {
                min_y = y;
                min_idx = i;
            }
            if y > max_y {
                max_y = y;
                max_idx = i;
            }
        }

        if yi <= min_y {
            return ReverseInterpolateResult { xi: self.x[min_idx], success: yi == min_y };
        }
        if yi >= max_y {
            return ReverseInterpolateResult { xi: self.x[max_idx], success: yi == max_y };
        }

        // Find the first interval [x_i, x_{i+1}] that contains yi in its y-range.
        for i in 0..self.x.len() - 1 {
            let y0 = self.y[i];
            let y1 = self.y[i + 1];
            if (y0 <= yi && yi <= y1) || (y1 <= yi && yi <= y0) {
                let mut low = self.x[i];
                let mut high = self.x[i + 1];
                // Optimization for endpoints
                if (yi - y0).abs() < 1e-9f32 {
                    return ReverseInterpolateResult { xi: low, success: true };
                }
                if (yi - y1).abs() < 1e-9f32 {
                    return ReverseInterpolateResult { xi: high, success: true };
                }

                // Bisection search
                let increasing = y0 <= y1;
                // 10 iterations are enough for accuracy.
                for _ in 0..10 {
                    let mid_x = low + (high - low) / 2.0f32;
                    if self.interpolate(mid_x) < yi {
                        if increasing {
                            low = mid_x;
                        } else {
                            high = mid_x;
                        }
                    } else {
                        if increasing {
                            high = mid_x;
                        } else {
                            low = mid_x;
                        }
                    }
                }
                return ReverseInterpolateResult { xi: low + (high - low) / 2.0f32, success: true };
            }
        }
        // Should not happen given check against max_y
        ReverseInterpolateResult { xi: *self.x.last().unwrap(), success: false }
    }

    /// Returns the `x` control points.
    pub fn x(&self) -> &[f32] {
        &self.x
    }
    /// Returns the `y` control points.
    pub fn y(&self) -> &[f32] {
        &self.y
    }
    /// Returns the precomputed slopes.
    pub fn slopes(&self) -> &[f32] {
        &self.slopes
    }
}

/// A gain curve representation based on a PCHIP interpolator, supporting logarithmic extrapolation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GainCurve {
    /// The underlying PCHIP interpolator defining the gain curve.
    pub interp: PchipInterpolator,
    /// The minimum y-value reached by the interpolator, used for extrapolation.
    pub y_min: f32,
}

/// Result structure for FFI calls creating a `GainCurve`, containing the gain curve instance or an error message.
pub struct GainCurveResult {
    /// The resulting gain curve instance if creation succeeded.
    pub curve: GainCurve,
    /// True if the gain curve was successfully created, false otherwise.
    pub success: bool,
    /// Detailed error message if the creation failed.
    pub error_message: String,
}

impl GainCurveResult {
    /// Returns the error message if the gain curve creation failed.
    pub fn get_error_message(&self) -> &str {
        &self.error_message
    }
}

impl GainCurve {
    /// Creates a new `GainCurve` from control points `x`, `y`, and precomputed `slopes`.
    pub fn create_with_slopes(x: Vec<f32>, y: Vec<f32>, slopes: Vec<f32>) -> Self {
        let interp = PchipInterpolator::create_with_slopes(x, y, slopes);
        let mut y_min = 0.0;
        if !interp.y.is_empty() {
            y_min = interp.y.iter().cloned().fold(f32::INFINITY, f32::min);
        }
        Self { interp, y_min }
    }

    /// Creates a `GainCurve` from an existing `PchipInterpolator`.
    pub fn from_interpolator(interp: PchipInterpolator) -> Self {
        let mut y_min = 0.0;
        if !interp.y.is_empty() {
            y_min = interp.y.iter().cloned().fold(f32::INFINITY, f32::min);
        }
        Self { interp, y_min }
    }

    /// FFI wrapper for `GainCurve::create`, returning a `GainCurveResult`.
    pub fn create_ffi(x: &[f32], y: &[f32]) -> GainCurveResult {
        match Self::create(x.to_vec(), y.to_vec()) {
            Ok(curve) => GainCurveResult { curve, success: true, error_message: String::new() },
            Err(e) => {
                GainCurveResult { curve: GainCurve::default(), success: false, error_message: e }
            }
        }
    }

    /// FFI wrapper for `GainCurve::create_with_slopes`, returning a `GainCurveResult`.
    pub fn create_with_slopes_ffi(x: &[f32], y: &[f32], slopes: &[f32]) -> GainCurveResult {
        let curve = Self::create_with_slopes(x.to_vec(), y.to_vec(), slopes.to_vec());
        GainCurveResult { curve, success: true, error_message: String::new() }
    }

    /// Creates a `GainCurve` from control points `x` and `y`.
    /// Computes the necessary slopes internally.
    pub fn create(x: Vec<f32>, y: Vec<f32>) -> Result<Self, String> {
        let interp = PchipInterpolator::create(x, y)?;
        let mut y_min = 0.0;
        if !interp.y.is_empty() {
            y_min = interp.y.iter().cloned().fold(f32::INFINITY, f32::min);
        }
        Ok(Self { interp, y_min })
    }

    /// Interpolates the value at a given `xi`.
    pub fn interpolate(&self, xi: f32) -> f32 {
        let x_back = *self.interp.x.last().unwrap_or(&0.0);
        let y_back = *self.interp.y.last().unwrap_or(&0.0);
        if xi > x_back && x_back > 0.0 {
            return y_back + (x_back / xi).log2();
        }
        self.interp.interpolate(xi)
    }

    /// Finds an `xi` such that `interpolate(xi)` is approximately equal to `yi`.
    /// May return unexpected results if the gain curve is not monotonic.
    // TODO(vrabaud): Create a proper tone curve class that will handle the
    // reverse interpolation as the tone curve should be monotonically increasing
    // (and there is not guarantee for the gain curve).
    pub fn reverse_interpolate(&self, yi: f32) -> ReverseInterpolateResult {
        let y_back = *self.interp.y.last().unwrap_or(&0.0);
        let x_back = *self.interp.x.last().unwrap_or(&0.0);
        // If yi is smaller than the minimum value reached by the PCHIP and it is
        // smaller than y.back(), we solve the logarithmic extrapolation.
        if yi < self.y_min && yi < y_back {
            return ReverseInterpolateResult { xi: x_back * (y_back - yi).exp2(), success: true };
        }
        self.interp.reverse_interpolate(yi)
    }

    /// Returns a reference to the underlying `PchipInterpolator`.
    pub fn interpolator(&self) -> &PchipInterpolator {
        &self.interp
    }
}

/// Subsamples the given `x` and `y` points to select at most `n_break` points.
/// The selection is done by iteratively adding the point with the maximum interpolation error
/// using a PCHIP interpolator created from the currently selected points.
/// The first and last points are always included.
pub fn sub_sample_dist(x: &[f32], y: &[f32], n_break: usize) -> Result<Vec<usize>, String> {
    let mut selected_mask = vec![false; x.len()];
    selected_mask[0] = true;
    if x.len() > 1 {
        selected_mask[x.len() - 1] = true;
    }

    for _ in 0..(n_break as i32 - 2).max(0) {
        let mut current_x = Vec::new();
        let mut current_y = Vec::new();
        for j in 0..x.len() {
            if selected_mask[j] {
                current_x.push(x[j]);
                current_y.push(y[j]);
            }
        }
        let interp = PchipInterpolator::create(current_x, current_y)?;
        let mut max_error = -1.0;
        let mut max_idx = None;
        for j in 0..x.len() {
            if !selected_mask[j] {
                let error = (interp.interpolate(x[j]) - y[j]).abs();
                if error > max_error {
                    max_error = error;
                    max_idx = Some(j);
                }
            }
        }
        if let Some(idx) = max_idx {
            selected_mask[idx] = true;
        } else {
            break;
        }
    }

    let mut selected_indices = Vec::new();
    for (i, &selected) in selected_mask.iter().enumerate() {
        if selected {
            selected_indices.push(i);
        }
    }
    selected_indices.sort();
    Ok(selected_indices)
}

/// FFI wrapper for `create_subsampled_pchip`, returning a `PchipInterpolatorResult`.
pub fn create_subsampled_pchip_ffi(
    x: &[f32],
    y: &[f32],
    num_control_points: usize,
) -> PchipInterpolatorResult {
    match create_subsampled_pchip(x, y, num_control_points) {
        Ok(interp) => {
            PchipInterpolatorResult { interp, success: true, error_message: String::new() }
        }
        Err(e) => PchipInterpolatorResult {
            interp: PchipInterpolator::default(),
            success: false,
            error_message: e,
        },
    }
}

/// Creates a `PchipInterpolator` by first unique-ifying the `x` values
/// and then subsampling the points to a maximum of `num_control_points`.
/// Duplicate `x` values are handled by taking the last `y` value for that `x`.
pub fn create_subsampled_pchip(
    x: &[f32],
    y: &[f32],
    num_control_points: usize,
) -> Result<PchipInterpolator, String> {
    let mut unique_x = Vec::new();
    let mut unique_y = Vec::new();
    if !x.is_empty() {
        unique_x.push(x[0]);
        unique_y.push(y[0]);
        for i in 1..x.len() {
            if x[i] > *unique_x.last().unwrap() {
                unique_x.push(x[i]);
                unique_y.push(y[i]);
            } else if x[i] == *unique_x.last().unwrap() {
                *unique_y.last_mut().unwrap() = y[i]; // keep last y for same x
            }
        }
    }

    let selected_indices = sub_sample_dist(&unique_x, &unique_y, num_control_points)?;

    let mut x_sel = Vec::with_capacity(selected_indices.len());
    let mut y_sel = Vec::with_capacity(selected_indices.len());
    for &idx in &selected_indices {
        x_sel.push(unique_x[idx]);
        y_sel.push(unique_y[idx]);
    }

    PchipInterpolator::create(x_sel, y_sel)
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_pchip_slopes_linear() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 1.0, 2.0];
        let slopes = pchip_slopes(&x, &y).unwrap();
        assert_eq!(slopes, vec![1.0, 1.0, 1.0]);
    }

    #[gtest]
    fn test_interpolate_linear() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 1.0, 2.0];
        let interp = PchipInterpolator::create(x, y).unwrap();
        assert_eq!(interp.interpolate(0.5), 0.5);
        assert_eq!(interp.interpolate(1.5), 1.5);
    }

    #[gtest]
    fn test_reverse_interpolate() {
        let x = vec![0.0, 1.0, 2.0];
        let y = vec![0.0, 1.0, 2.0];
        let interp = PchipInterpolator::create(x, y).unwrap();
        let res = interp.reverse_interpolate(0.5);
        assert!(res.success);
        assert!((res.xi - 0.5f32).abs() < 1e-3f32);
    }

    #[gtest]
    fn test_gain_curve_extrapolation() {
        let x = vec![1.0, 2.0];
        let y = vec![1.0, 2.0];
        let curve = GainCurve::create(x, y).unwrap();
        // xi = 4.0, x_back = 2.0, y_back = 2.0. Interpolated = 2.0 + log2(2.0/4.0) = 2.0 - 1.0 = 1.0.
        assert_eq!(curve.interpolate(4.0), 1.0);
    }
}
