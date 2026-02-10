//! Potential Fields Module
//!
//! This module contains functions related to potential fields based on Blakely (1995).
//! Ported from Fortran to Rust.
//!
//! Author: Victor Carreira (Original Fortran)
//! Rust Port: 2026

use std::f64::consts::PI;

// Physical constants
const GAMMA: f64 = 6.67e-11; // Gravitational constant
const SI2MG: f64 = 1.0e5; // SI to milligal conversion
const KM2M: f64 = 1.0e3; // Kilometers to meters
const D2RAD: f64 = PI / 180.0; // Degrees to radians
const T2NT: f64 = 1.0e9; // Tesla to nanoTesla
const CM: f64 = 1.0e-7; // Constant for magnetic calculations
const TWOPI: f64 = 2.0 * PI;

/// Calculate the three components of gravitational attraction at a single point
/// due to a uniform sphere of homogeneous density.
///
/// # Arguments
///
/// * `xq, yq, zq` - Center of sphere coordinates (km)
/// * `ra` - Radius of sphere (km)
/// * `rho` - Density of sphere (kg/m³)
/// * `xp, yp, zp` - Observation point coordinates (km)
///
/// # Returns
///
/// Tuple of (gx, gy, gz) - Gravitational components (mGal)
pub fn sphere(
    xq: f64,
    yq: f64,
    zq: f64,
    ra: f64,
    rho: f64,
    xp: f64,
    yp: f64,
    zp: f64,
) -> Result<(f64, f64, f64), &'static str> {
    let rx = xp - xq;
    let ry = yp - yq;
    let rz = zp - zq;
    let r = (rx * rx + ry * ry + rz * rz).sqrt();

    if r == 0.0 {
        return Err("Sphere: Bad argument detected! r = 0");
    }

    let r3 = r * r * r;
    let tmass = 4.0 * PI * rho * ra.powi(3) / 3.0;

    let mut gx = -GAMMA * tmass * rx / r3;
    let mut gy = -GAMMA * tmass * ry / r3;
    let mut gz = -GAMMA * tmass * rz / r3;

    // Convert to mGal
    gx *= SI2MG * KM2M;
    gy *= SI2MG * KM2M;
    gz *= SI2MG * KM2M;

    Ok((gx, gy, gz))
}

/// Calculate x and z components of gravitational attraction due to an
/// infinitely extended cylinder lying parallel to y axis.
///
/// # Arguments
///
/// * `xq, zq` - Axis of cylinder coordinates in x-z plane (km)
/// * `ra` - Radius of cylinder (km)
/// * `rho` - Density (kg/m³)
/// * `xp, zp` - Observation point coordinates (km)
///
/// # Returns
///
/// Tuple of (gx, gz) - Components of gravitational attraction (mGal)
pub fn cylinder(
    xq: f64,
    zq: f64,
    ra: f64,
    rho: f64,
    xp: f64,
    zp: f64,
) -> Result<(f64, f64), &'static str> {
    let rx = xp - xq;
    let rz = zp - zq;
    let r2 = rx * rx + rz * rz;

    if r2 == 0.0 {
        return Err("Cylinder: Bad argument detected! r2 = 0");
    }

    let tmass = PI * ra * ra * rho;
    let mut gx = 2.0 * GAMMA * tmass * rx / r2;
    let mut gz = 2.0 * GAMMA * tmass * rz / r2;

    // Convert to mGal
    gx *= SI2MG * KM2M;
    gz *= SI2MG * KM2M;

    Ok((gx, gz))
}

/// Compute direction cosines from inclination and declination.
///
/// # Arguments
///
/// * `incl` - Inclination in degrees positive below horizontal
/// * `decl` - Declination in degrees positive east of true north
/// * `azim` - Azimuth of x axis in degrees positive east of north
///
/// # Returns
///
/// Tuple of (a, b, c) - The three direction cosines
pub fn dircos(incl: f64, decl: f64, azim: f64) -> (f64, f64, f64) {
    let xincl = incl * D2RAD;
    let xdecl = decl * D2RAD;
    let xazim = azim * D2RAD;

    let a = xincl.cos() * (xdecl - xazim).cos();
    let b = xincl.cos() * (xdecl - xazim).sin();
    let c = xincl.sin();

    (a, b, c)
}

/// Compute the three components of magnetic induction caused by a uniformly
/// magnetized sphere. X axis is north, Z axis is down.
///
/// # Arguments
///
/// * `xq, yq, zq` - Sphere center coordinates
/// * `ra` - Radius of sphere
/// * `mi` - Magnetization inclination (degrees)
/// * `md` - Magnetization declination (degrees)
/// * `m` - Intensity of magnetization (A/m)
/// * `xp, yp, zp` - Observation point coordinates
///
/// # Returns
///
/// Tuple of (bx, by, bz) - The three components of magnetic induction (nT)
pub fn dipole(
    xq: f64,
    yq: f64,
    zq: f64,
    ra: f64,
    mi: f64,
    md: f64,
    m: f64,
    xp: f64,
    yp: f64,
    zp: f64,
) -> Result<(f64, f64, f64), &'static str> {
    // Get direction cosines
    let (mx, my, mz) = dircos(mi, md, 0.0);

    let rx = xp - xq;
    let ry = yp - yq;
    let rz = zp - zq;
    let r2 = rx * rx + ry * ry + rz * rz;
    let r = r2.sqrt();

    if r == 0.0 {
        return Err("Dipole: Bad argument detected! r = 0");
    }

    let r5 = r.powi(5);
    let dot = rx * mx + ry * my + rz * mz;
    let moment = 4.0 * PI * ra.powi(3) * m / 3.0;

    let mut bx = CM * moment * (3.0 * dot * rx - r2 * mx) / r5;
    let mut by = CM * moment * (3.0 * dot * ry - r2 * my) / r5;
    let mut bz = CM * moment * (3.0 * dot * rz - r2 * mz) / r5;

    // Convert to nT
    bx *= T2NT;
    by *= T2NT;
    bz *= T2NT;

    Ok((bx, by, bz))
}

/// Calculate the factorial of n (n!).
///
/// # Arguments
///
/// * `n` - Input number
///
/// # Returns
///
/// Factorial of n
pub fn fac(n: i32) -> Result<i32, &'static str> {
    if n < 0 {
        return Err("fac: Bad argument detected! n < 0");
    }

    if n == 0 || n == 1 {
        return Ok(1);
    }

    let mut result = 1;
    for i in 2..=n {
        result *= i;
    }

    Ok(result)
}

/// Return Schmidt normalized Legendre polynomial.
/// Based on Press et al. (1986).
///
/// # Arguments
///
/// * `n` - Degree of polynomial (must be > 0)
/// * `m` - Order of polynomial (must be 0 <= m <= n)
/// * `theta` - Argument of polynomial (degrees)
///
/// # Returns
///
/// Schmidt normalized Legendre polynomial value
pub fn schmit(n: i32, m: i32, theta: f64) -> Result<f64, &'static str> {
    if m < 0 || m > n {
        return Err("Schmit: Bad argument detected! m < 0 or m > n");
    }

    let x = (theta * D2RAD).cos();
    let mut pmm = 1.0;

    if m > 0 {
        let somx2 = ((1.0 - x) * (1.0 + x)).sqrt();
        let mut fact = 1.0;
        for _ in 1..=m {
            pmm = -pmm * fact * somx2;
            fact += 2.0;
        }
    }

    let result = if n == m {
        pmm
    } else {
        let pmmp1 = x * (2 * m + 1) as f64 * pmm;
        if n == m + 1 {
            pmmp1
        } else {
            let mut pmm_val = pmm;
            let mut pmmp1_val = pmmp1;
            let mut pnn = 0.0;
            for nn in (m + 2)..=n {
                pnn = (x * (2 * nn - 1) as f64 * pmmp1_val - (nn + m - 1) as f64 * pmm_val)
                    / (nn - m) as f64;
                pmm_val = pmmp1_val;
                pmmp1_val = pnn;
            }
            pnn
        }
    };

    let result = if m != 0 {
        let xnorm = (2.0 * fac(n - m)? as f64 / fac(n + m)? as f64).sqrt();
        xnorm * result
    } else {
        result
    };

    Ok(result)
}

/// Compute the vertical attraction of a rectangular prism.
/// Sides of prism are parallel to x, y, z axes, and z axis is vertical down.
///
/// # Arguments
///
/// * `x0, y0, z0` - Observation point coordinates (km)
/// * `x1, y1, z1` - Minimum corner of prism (km)
/// * `x2, y2, z2` - Maximum corner of prism (km)
/// * `rho` - Density of prism (kg/m³)
///
/// # Returns
///
/// Vertical attraction of gravity (mGal)
pub fn gbox(
    x0: f64,
    y0: f64,
    z0: f64,
    x1: f64,
    y1: f64,
    z1: f64,
    x2: f64,
    y2: f64,
    z2: f64,
    rho: f64,
) -> Result<f64, &'static str> {
    let isign = [-1.0, 1.0];
    let x = [x0 - x1, x0 - x2];
    let y = [y0 - y1, y0 - y2];
    let z = [z0 - z1, z0 - z2];

    let mut total_sum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let rijk = (x[i] * x[i] + y[j] * y[j] + z[k] * z[k]).sqrt();
                let ijk = isign[i] * isign[j] * isign[k];

                let mut arg1 = (x[i] * y[j]).atan2(z[k] * rijk);
                if arg1 < 0.0 {
                    arg1 += TWOPI;
                }

                let arg2 = rijk + y[j];
                let arg3 = rijk + x[i];

                if arg2 <= 0.0 {
                    return Err("Gbox: Bad field point! arg2 <= 0");
                }
                if arg3 <= 0.0 {
                    return Err("Gbox: Bad field point! arg3 <= 0");
                }

                let arg2 = arg2.ln();
                let arg3 = arg3.ln();

                total_sum += ijk * (z[k] * arg1 - x[i] * arg2 - y[j] * arg3);
            }
        }
    }

    let g = rho * GAMMA * total_sum * SI2MG * KM2M;

    Ok(g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dircos() {
        let (a, b, c) = dircos(45.0, 30.0, 0.0);
        // Basic sanity check
        assert!(a.abs() < 1.0);
        assert!(b.abs() < 1.0);
        assert!(c.abs() < 1.0);
    }

    #[test]
    fn test_fac() {
        assert_eq!(fac(0).unwrap(), 1);
        assert_eq!(fac(1).unwrap(), 1);
        assert_eq!(fac(5).unwrap(), 120);
        assert!(fac(-1).is_err());
    }

    #[test]
    fn test_sphere() {
        let result = sphere(0.0, 0.0, 0.0, 1.0, 1000.0, 10.0, 0.0, 0.0);
        assert!(result.is_ok());
        let (gx, _gy, _gz) = result.unwrap();
        // Gravity should be in the x direction
        assert!(gx.abs() > 0.0);
    }

    #[test]
    fn test_cylinder() {
        let result = cylinder(0.0, 0.0, 1.0, 1000.0, 10.0, 0.0);
        assert!(result.is_ok());
        let (gx, _gz) = result.unwrap();
        assert!(gx.abs() > 0.0);
    }
}
