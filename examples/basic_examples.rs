//! Example usage of the geophysics-blakely Rust library.
//!
//! This program demonstrates the main functions from the potential fields,
//! file reader, and meshgrid modules.

use geophysics_blakely::{file_reader, meshgrid, potential};

fn main() {
    println!("{}", "=".repeat(60));
    println!("Geophysics Blakely - Rust Examples");
    println!("{}", "=".repeat(60));

    // Example 1: Sphere gravitational attraction
    println!("\n1. Sphere Gravitational Attraction");
    println!("{}", "-".repeat(40));
    println!("Calculating gravity from a sphere:");
    println!("  Sphere center: (0, 0, 0) km");
    println!("  Sphere radius: 1 km");
    println!("  Density: 2670 kg/m³");
    println!("  Observation point: (5, 0, 0) km");

    match potential::sphere(0.0, 0.0, 0.0, 1.0, 2670.0, 5.0, 0.0, 0.0) {
        Ok((gx, gy, gz)) => {
            println!("\nResults:");
            println!("  gx = {:.6} mGal", gx);
            println!("  gy = {:.6} mGal", gy);
            println!("  gz = {:.6} mGal", gz);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 2: Cylinder gravitational attraction
    println!("\n2. Cylinder Gravitational Attraction");
    println!("{}", "-".repeat(40));
    println!("Calculating gravity from an infinite cylinder:");
    println!("  Cylinder axis: (0, 0) in x-z plane");
    println!("  Cylinder radius: 1 km");
    println!("  Density: 2670 kg/m³");
    println!("  Observation point: (5, 0) in x-z plane");

    match potential::cylinder(0.0, 0.0, 1.0, 2670.0, 5.0, 0.0) {
        Ok((gx, gz)) => {
            println!("\nResults:");
            println!("  gx = {:.6} mGal", gx);
            println!("  gz = {:.6} mGal", gz);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 3: Magnetic dipole
    println!("\n3. Magnetic Dipole");
    println!("{}", "-".repeat(40));
    println!("Calculating magnetic field from a magnetized sphere:");
    println!("  Sphere center: (0, 0, 0)");
    println!("  Sphere radius: 1 unit");
    println!("  Magnetization: 1000 A/m, incl=45°, decl=30°");
    println!("  Observation point: (5, 0, 0)");

    match potential::dipole(0.0, 0.0, 0.0, 1.0, 45.0, 30.0, 1000.0, 5.0, 0.0, 0.0) {
        Ok((bx, by, bz)) => {
            println!("\nResults:");
            println!("  bx = {:.6} nT", bx);
            println!("  by = {:.6} nT", by);
            println!("  bz = {:.6} nT", bz);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 4: Direction cosines
    println!("\n4. Direction Cosines");
    println!("{}", "-".repeat(40));
    println!("Computing direction cosines:");
    println!("  Inclination: 45°");
    println!("  Declination: 30°");
    println!("  Azimuth: 0°");

    let (a, b, c) = potential::dircos(45.0, 30.0, 0.0);
    println!("\nResults:");
    println!("  a = {:.6}", a);
    println!("  b = {:.6}", b);
    println!("  c = {:.6}", c);

    // Example 5: Rectangular prism (gbox)
    println!("\n5. Rectangular Prism Gravity");
    println!("{}", "-".repeat(40));
    println!("Calculating vertical gravity from a rectangular prism:");
    println!("  Observation: (0, 0, 0) km");
    println!("  Prism: x=[-1,1], y=[-1,1], z=[1,2] km");
    println!("  Density: 2670 kg/m³");

    match potential::gbox(0.0, 0.0, 0.0, -1.0, -1.0, 1.0, 1.0, 1.0, 2.0, 2670.0) {
        Ok(g) => {
            println!("\nResults:");
            println!("  g = {:.6} mGal", g);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 6: Linear fitting
    println!("\n6. Linear Least Squares Fitting");
    println!("{}", "-".repeat(40));
    println!("Fitting a line to data:");

    let x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    let y = vec![1.0, 3.0, 5.0, 7.0, 9.0];

    println!("  Data points: {}", x.len());
    println!("  x = {:?}", x);
    println!("  y = {:?}", y);

    match file_reader::fit_ab(&x, &y) {
        Ok((a, b, sa, sb)) => {
            println!("\nResults:");
            println!("  y = {:.6}*x + {:.6}", a, b);
            println!("  Uncertainties: sa={:.6}, sb={:.6}", sa, sb);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Example 7: Meshgrid
    println!("\n7. Meshgrid Creation");
    println!("{}", "-".repeat(40));
    println!("Creating a 2D grid:");

    let x = meshgrid::linspace(0.0, 10.0, 5);
    let y = meshgrid::linspace(0.0, 5.0, 3);

    println!("  x grid: {:?}", x);
    println!("  y grid: {:?}", y);

    let (grid_x, grid_y) = meshgrid::meshgrid2d(&x, &y);
    println!("\nResults:");
    println!("  Grid shape: ({}, {})", grid_x.len(), grid_x[0].len());
    println!("  X[0,:] = {:?}", grid_x[0]);
    println!("  Y[0,:] = {:?}", grid_y[0]);

    println!("\n{}", "=".repeat(60));
    println!("All examples completed successfully!");
    println!("{}", "=".repeat(60));
}
