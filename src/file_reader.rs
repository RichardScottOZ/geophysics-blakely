//! File Reader Utilities
//!
//! This module contains utilities for reading files and performing linear fits.
//! Ported from Fortran to Rust.
//!
//! Author: Victor Carreira (Original Fortran)
//! Rust Port: 2026

use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Count the number of lines in a file.
///
/// # Arguments
///
/// * `filename` - Path to the file
/// * `skip_header` - Whether to skip the first line
///
/// # Returns
///
/// Number of lines in the file
pub fn count_lines(filename: &str, skip_header: bool) -> Result<usize, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    if skip_header {
        lines.next();
    }

    let count = lines.count();
    Ok(count)
}

/// Read x and y data from a file.
///
/// # Arguments
///
/// * `filename` - Path to the data file
/// * `skip_header` - Whether to skip the first line
///
/// # Returns
///
/// Tuple of (x_values, y_values) as vectors
pub fn read_data_file(
    filename: &str,
    skip_header: bool,
) -> Result<(Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    if skip_header {
        lines.next();
    }

    let mut x_values = Vec::new();
    let mut y_values = Vec::new();

    for line_result in lines {
        let line = line_result?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            let x: f64 = parts[0].parse()?;
            let y: f64 = parts[1].parse()?;
            x_values.push(x);
            y_values.push(y);
        }
    }

    Ok((x_values, y_values))
}

/// Perform linear fit to calculate coefficients a and b for y = a*x + b.
/// Also calculate uncertainties sa and sb.
///
/// # Arguments
///
/// * `x` - X data values
/// * `y` - Y data values
///
/// # Returns
///
/// Tuple of (a, b, sa, sb) - Linear (a) and angular (b) coefficients and their uncertainties
pub fn fit_ab(x: &[f64], y: &[f64]) -> Result<(f64, f64, f64, f64), &'static str> {
    let n = x.len();

    if n != y.len() {
        return Err("x and y arrays must have the same length");
    }

    if n < 2 {
        return Err("Need at least 2 data points for fitting");
    }

    let n_f64 = n as f64;

    // Calculate sums
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xx: f64 = x.iter().map(|&xi| xi * xi).sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(&xi, &yi)| xi * yi).sum();

    let delta = n_f64 * sum_xx - sum_x * sum_x;

    if delta == 0.0 {
        return Err("Delta is zero - cannot perform fit");
    }

    let mi = sum_x / n_f64;
    let sig2: f64 = x.iter().map(|&xi| (xi - mi).powi(2)).sum::<f64>() / n_f64;

    // Calculate uncertainties
    let sa = (sig2 * n_f64 / delta).sqrt();
    let sb = (sig2 * sum_xx / delta).sqrt();

    // Calculate coefficients using least squares
    let a = (n_f64 * sum_xy - sum_x * sum_y) / delta;
    let b = (sum_xx * sum_y - sum_xy * sum_x) / delta;

    Ok((a, b, sa, sb))
}

/// Read data from a file, perform linear fit, and write results to output file.
///
/// # Arguments
///
/// * `input_file` - Path to input data file
/// * `output_file` - Path to output file for parameters
/// * `skip_header` - Whether to skip the first line of input file
///
/// # Returns
///
/// Tuple of (a, b, sa, sb) - Linear fit coefficients and uncertainties
pub fn process_data_file(
    input_file: &str,
    output_file: &str,
    skip_header: bool,
) -> Result<(f64, f64, f64, f64), Box<dyn std::error::Error>> {
    // Read data
    let (x, y) = read_data_file(input_file, skip_header)?;

    println!("Number of data points: {}", x.len());

    // Perform fit
    let (a, b, sa, sb) = fit_ab(&x, &y)?;

    println!("Coefficients and uncertainties:");
    println!("a = {:.6}, sa = {:.6}", a, sa);
    println!("b = {:.6}, sb = {:.6}", b, sb);

    // Write results to output file
    let mut file = File::create(output_file)?;
    writeln!(file, "       a       b      sa     sb")?;
    writeln!(file, "{:10.6} {:10.6} {:10.6} {:10.6}", a, b, sa, sb)?;

    Ok((a, b, sa, sb))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_ab() {
        // Test with simple linear data: y = 2x + 1
        let x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let y = vec![1.0, 3.0, 5.0, 7.0, 9.0];

        let result = fit_ab(&x, &y);
        assert!(result.is_ok());

        let (a, b, _sa, _sb) = result.unwrap();
        // a should be approximately 2.0
        assert!((a - 2.0).abs() < 1e-10);
        // b should be approximately 1.0
        assert!((b - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_fit_ab_error_cases() {
        let x = vec![1.0];
        let y = vec![1.0];

        // Should fail with too few points
        assert!(fit_ab(&x, &y).is_err());

        let x = vec![1.0, 2.0];
        let y = vec![1.0];

        // Should fail with mismatched lengths
        assert!(fit_ab(&x, &y).is_err());
    }
}
