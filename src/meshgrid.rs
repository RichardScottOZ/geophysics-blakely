//! Meshgrid Module
//!
//! This module provides meshgrid functionality for 2D and 3D grids.
//! Ported from Fortran to Rust.
//!
//! Author: Victor Carreira (Original Fortran)
//! Rust Port: 2026

/// Create a linearly spaced 1D array.
///
/// # Arguments
///
/// * `start` - Starting value
/// * `end` - Ending value
/// * `num_points` - Number of points
///
/// # Returns
///
/// Vector of linearly spaced values
pub fn linspace(start: f64, end: f64, num_points: usize) -> Vec<f64> {
    if num_points == 0 {
        return Vec::new();
    }
    
    if num_points == 1 {
        return vec![start];
    }

    let step = (end - start) / (num_points - 1) as f64;
    (0..num_points)
        .map(|i| start + step * i as f64)
        .collect()
}

/// Create a 2D meshgrid from 1D arrays.
///
/// # Arguments
///
/// * `x` - 1D array of x coordinates
/// * `y` - 1D array of y coordinates
///
/// # Returns
///
/// Tuple of (X, Y) - 2D arrays representing the grid coordinates
/// Each grid is represented as a vector of vectors (rows)
pub fn meshgrid2d(x: &[f64], y: &[f64]) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let nx = x.len();
    let ny = y.len();

    let mut grid_x = Vec::with_capacity(nx);
    let mut grid_y = Vec::with_capacity(nx);

    for &xi in x.iter() {
        grid_x.push(vec![xi; ny]);
    }

    for _ in 0..nx {
        grid_y.push(y.to_vec());
    }

    (grid_x, grid_y)
}

/// Create a 3D meshgrid from 1D arrays.
///
/// # Arguments
///
/// * `x` - 1D array of x coordinates
/// * `y` - 1D array of y coordinates
/// * `z` - 1D array of z coordinates
///
/// # Returns
///
/// Tuple of (X, Y, Z) - 3D arrays representing the grid coordinates
/// Each grid is represented as a vector of vector of vectors
pub fn meshgrid3d(
    x: &[f64],
    y: &[f64],
    z: &[f64],
) -> (Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>, Vec<Vec<Vec<f64>>>) {
    let nx = x.len();
    let ny = y.len();
    let nz = z.len();

    let mut grid_x = Vec::with_capacity(nx);
    let mut grid_y = Vec::with_capacity(nx);
    let mut grid_z = Vec::with_capacity(nx);

    for &xi in x.iter() {
        let mut slice_x = Vec::with_capacity(ny);
        let mut slice_y = Vec::with_capacity(ny);
        let mut slice_z = Vec::with_capacity(ny);

        for &yi in y.iter() {
            slice_x.push(vec![xi; nz]);
            slice_y.push(vec![yi; nz]);
            slice_z.push(z.to_vec());
        }

        grid_x.push(slice_x);
        grid_y.push(slice_y);
        grid_z.push(slice_z);
    }

    (grid_x, grid_y, grid_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace() {
        let result = linspace(0.0, 1.0, 5);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 0.0);
        assert_eq!(result[4], 1.0);
        assert!((result[2] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_meshgrid2d() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![4.0, 5.0];

        let (grid_x, grid_y) = meshgrid2d(&x, &y);

        assert_eq!(grid_x.len(), 3);
        assert_eq!(grid_x[0].len(), 2);
        assert_eq!(grid_x[0][0], 1.0);
        assert_eq!(grid_x[2][1], 3.0);

        assert_eq!(grid_y.len(), 3);
        assert_eq!(grid_y[0].len(), 2);
        assert_eq!(grid_y[0][0], 4.0);
        assert_eq!(grid_y[2][1], 5.0);
    }

    #[test]
    fn test_meshgrid3d() {
        let x = vec![1.0, 2.0];
        let y = vec![3.0, 4.0];
        let z = vec![5.0, 6.0];

        let (grid_x, grid_y, grid_z) = meshgrid3d(&x, &y, &z);

        assert_eq!(grid_x.len(), 2);
        assert_eq!(grid_x[0].len(), 2);
        assert_eq!(grid_x[0][0].len(), 2);

        assert_eq!(grid_x[0][0][0], 1.0);
        assert_eq!(grid_y[0][0][0], 3.0);
        assert_eq!(grid_z[0][0][0], 5.0);
    }
}
