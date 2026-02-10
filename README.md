# geophysics-blakely

Ported Fortran code to Python and Rust implementations.

This repository contains geophysics-related functions and utilities, originally written in Fortran by Victor Ribeiro Carreira, and ported to both Python and Rust.

## Original Source

The code is ported from: https://github.com/VictorCarreira/Geophysics

Based on Blakely (1995) - Potential Theory in Gravity and Magnetic Applications

## Contents

### Python Implementation

The Python implementation includes the following modules:

- **potential.py** - Potential fields calculations
  - `sphere()` - Gravitational attraction from a uniform sphere
  - `cylinder()` - Gravitational attraction from a cylinder
  - `dircos()` - Direction cosines calculation
  - `dipole()` - Magnetic induction from a magnetized sphere
  - `schmit()` - Schmidt normalized Legendre polynomial
  - `fac()` - Factorial function
  - `gbox()` - Vertical attraction of a rectangular prism

- **file_reader.py** - File reading utilities and linear fitting
  - `skip_line()` - Skip a line in a file
  - `count_lines()` - Count lines in a file
  - `read_data_file()` - Read x,y data from a file
  - `fit_ab()` - Linear least squares fitting
  - `process_data_file()` - Complete workflow for data fitting

- **meshgrid.py** - Meshgrid functionality
  - `meshgrid2d()` - Create 2D meshgrid
  - `meshgrid3d()` - Create 3D meshgrid
  - `create_linear_grid()` - Create linearly spaced array

- **gpr.py** - Ground Penetrating Radar module (placeholder)
- **magnetotelluric.py** - Magnetotelluric module (placeholder)
- **seismic.py** - Seismic module (placeholder)
- **seismology.py** - Seismology module (placeholder)

### Rust Implementation

The Rust implementation includes the following modules:

- **src/potential.rs** - Potential fields calculations
  - `sphere()` - Gravitational attraction from a uniform sphere
  - `cylinder()` - Gravitational attraction from a cylinder
  - `dircos()` - Direction cosines calculation
  - `dipole()` - Magnetic induction from a magnetized sphere
  - `schmit()` - Schmidt normalized Legendre polynomial
  - `fac()` - Factorial function
  - `gbox()` - Vertical attraction of a rectangular prism

- **src/file_reader.rs** - File reading utilities and linear fitting
  - `count_lines()` - Count lines in a file
  - `read_data_file()` - Read x,y data from a file
  - `fit_ab()` - Linear least squares fitting
  - `process_data_file()` - Complete workflow for data fitting

- **src/meshgrid.rs** - Meshgrid functionality
  - `linspace()` - Create linearly spaced array
  - `meshgrid2d()` - Create 2D meshgrid
  - `meshgrid3d()` - Create 3D meshgrid

- **src/gpr.rs** - Ground Penetrating Radar module (placeholder)
- **src/magnetotelluric.rs** - Magnetotelluric module (placeholder)
- **src/seismic.rs** - Seismic module (placeholder)
- **src/seismology.rs** - Seismology module (placeholder)

## Usage

### Python

```python
# Import the potential module
from potential import sphere, cylinder, dipole, gbox

# Calculate gravitational attraction from a sphere
gx, gy, gz = sphere(
    xq=0.0, yq=0.0, zq=0.0,  # sphere center (km)
    ra=1.0,                    # radius (km)
    rho=2670.0,                # density (kg/m³)
    xp=5.0, yp=0.0, zp=0.0    # observation point (km)
)
print(f"Gravitational components: gx={gx}, gy={gy}, gz={gz} mGal")

# Linear fitting example
from file_reader import fit_ab
import numpy as np

x = np.array([0.0, 1.0, 2.0, 3.0, 4.0])
y = np.array([1.0, 3.0, 5.0, 7.0, 9.0])
a, b, sa, sb = fit_ab(x, y)
print(f"Linear fit: y = {a}*x + {b}")

# Meshgrid example
from meshgrid import meshgrid2d, create_linear_grid

x = create_linear_grid(0.0, 10.0, 50)
y = create_linear_grid(0.0, 10.0, 50)
X, Y = meshgrid2d(x, y)
```

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
geophysics_blakely = { path = "." }
```

Example usage:

```rust
use geophysics_blakely::potential;
use geophysics_blakely::file_reader;
use geophysics_blakely::meshgrid;

fn main() {
    // Calculate gravitational attraction from a sphere
    let result = potential::sphere(
        0.0, 0.0, 0.0,  // sphere center (km)
        1.0,            // radius (km)
        2670.0,         // density (kg/m³)
        5.0, 0.0, 0.0   // observation point (km)
    );
    
    match result {
        Ok((gx, gy, gz)) => {
            println!("Gravitational components: gx={}, gy={}, gz={} mGal", gx, gy, gz);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Linear fitting example
    let x = vec![0.0, 1.0, 2.0, 3.0, 4.0];
    let y = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    
    match file_reader::fit_ab(&x, &y) {
        Ok((a, b, sa, sb)) => {
            println!("Linear fit: y = {}*x + {}", a, b);
            println!("Uncertainties: sa={}, sb={}", sa, sb);
        },
        Err(e) => eprintln!("Error: {}", e),
    }
    
    // Meshgrid example
    let x = meshgrid::linspace(0.0, 10.0, 50);
    let y = meshgrid::linspace(0.0, 10.0, 50);
    let (grid_x, grid_y) = meshgrid::meshgrid2d(&x, &y);
}
```

## Building and Testing

### Python

No build step required. Simply import the modules as shown above.

To run tests:
```bash
python -m pytest  # if you add tests
```

### Rust

Build the library:
```bash
cargo build
```

Run tests:
```bash
cargo test
```

Build with optimizations:
```bash
cargo build --release
```

## License

Copyright (C) 2019 by Victor Ribeiro Carreira / victorcarreira@on.br

Python and Rust ports: 2026

## References

- Blakely, R. J. (1995). Potential Theory in Gravity and Magnetic Applications. Cambridge University Press.
- Original Fortran repository: https://github.com/VictorCarreira/Geophysics

