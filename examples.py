#!/usr/bin/env python3
"""
Example usage of the geophysics-blakely Python library.

This script demonstrates the main functions from the potential fields,
file reader, and meshgrid modules.
"""

import numpy as np
from potential import sphere, cylinder, dipole, dircos, fac, gbox
from file_reader import fit_ab
from meshgrid import meshgrid2d, create_linear_grid


def main():
    print("=" * 60)
    print("Geophysics Blakely - Python Examples")
    print("=" * 60)
    
    # Example 1: Sphere gravitational attraction
    print("\n1. Sphere Gravitational Attraction")
    print("-" * 40)
    print("Calculating gravity from a sphere:")
    print("  Sphere center: (0, 0, 0) km")
    print("  Sphere radius: 1 km")
    print("  Density: 2670 kg/m³")
    print("  Observation point: (5, 0, 0) km")
    
    gx, gy, gz = sphere(0.0, 0.0, 0.0, 1.0, 2670.0, 5.0, 0.0, 0.0)
    print(f"\nResults:")
    print(f"  gx = {gx:.6f} mGal")
    print(f"  gy = {gy:.6f} mGal")
    print(f"  gz = {gz:.6f} mGal")
    
    # Example 2: Cylinder gravitational attraction
    print("\n2. Cylinder Gravitational Attraction")
    print("-" * 40)
    print("Calculating gravity from an infinite cylinder:")
    print("  Cylinder axis: (0, 0) in x-z plane")
    print("  Cylinder radius: 1 km")
    print("  Density: 2670 kg/m³")
    print("  Observation point: (5, 0) in x-z plane")
    
    gx, gz = cylinder(0.0, 0.0, 1.0, 2670.0, 5.0, 0.0)
    print(f"\nResults:")
    print(f"  gx = {gx:.6f} mGal")
    print(f"  gz = {gz:.6f} mGal")
    
    # Example 3: Magnetic dipole
    print("\n3. Magnetic Dipole")
    print("-" * 40)
    print("Calculating magnetic field from a magnetized sphere:")
    print("  Sphere center: (0, 0, 0)")
    print("  Sphere radius: 1 unit")
    print("  Magnetization: 1000 A/m, incl=45°, decl=30°")
    print("  Observation point: (5, 0, 0)")
    
    bx, by, bz = dipole(0.0, 0.0, 0.0, 1.0, 45.0, 30.0, 1000.0, 5.0, 0.0, 0.0)
    print(f"\nResults:")
    print(f"  bx = {bx:.6f} nT")
    print(f"  by = {by:.6f} nT")
    print(f"  bz = {bz:.6f} nT")
    
    # Example 4: Direction cosines
    print("\n4. Direction Cosines")
    print("-" * 40)
    print("Computing direction cosines:")
    print("  Inclination: 45°")
    print("  Declination: 30°")
    print("  Azimuth: 0°")
    
    a, b, c = dircos(45.0, 30.0, 0.0)
    print(f"\nResults:")
    print(f"  a = {a:.6f}")
    print(f"  b = {b:.6f}")
    print(f"  c = {c:.6f}")
    
    # Example 5: Rectangular prism (gbox)
    print("\n5. Rectangular Prism Gravity")
    print("-" * 40)
    print("Calculating vertical gravity from a rectangular prism:")
    print("  Observation: (0, 0, 0) km")
    print("  Prism: x=[-1,1], y=[-1,1], z=[1,2] km")
    print("  Density: 2670 kg/m³")
    
    g = gbox(0.0, 0.0, 0.0, -1.0, -1.0, 1.0, 1.0, 1.0, 2.0, 2670.0)
    print(f"\nResults:")
    print(f"  g = {g:.6f} mGal")
    
    # Example 6: Linear fitting
    print("\n6. Linear Least Squares Fitting")
    print("-" * 40)
    print("Fitting a line to data:")
    
    x = np.array([0.0, 1.0, 2.0, 3.0, 4.0])
    y = np.array([1.0, 3.0, 5.0, 7.0, 9.0])
    
    print(f"  Data points: {len(x)}")
    print(f"  x = {x}")
    print(f"  y = {y}")
    
    a, b, sa, sb = fit_ab(x, y)
    print(f"\nResults:")
    print(f"  y = {a:.6f}*x + {b:.6f}")
    print(f"  Uncertainties: sa={sa:.6f}, sb={sb:.6f}")
    
    # Example 7: Meshgrid
    print("\n7. Meshgrid Creation")
    print("-" * 40)
    print("Creating a 2D grid:")
    
    x = create_linear_grid(0.0, 10.0, 5)
    y = create_linear_grid(0.0, 5.0, 3)
    
    print(f"  x grid: {x}")
    print(f"  y grid: {y}")
    
    X, Y = meshgrid2d(x, y)
    print(f"\nResults:")
    print(f"  Grid shape: {X.shape}")
    print(f"  X[0,:] = {X[0,:]}")
    print(f"  Y[:,0] = {Y[:,0]}")
    
    print("\n" + "=" * 60)
    print("All examples completed successfully!")
    print("=" * 60)


if __name__ == '__main__':
    main()
