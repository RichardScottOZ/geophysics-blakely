"""
Potential Fields Module

This module contains subroutines related to potential fields based on Blakely (1995).
Ported from Fortran to Python.

Author: Victor Carreira (Original Fortran)
Python Port: 2026
"""

import numpy as np
from typing import Tuple


# Physical constants
GAMMA = 6.67e-11  # Gravitational constant
SI2MG = 1.0e5     # SI to milligal conversion
PI = np.pi
KM2M = 1.0e3      # Kilometers to meters
D2RAD = np.pi / 180.0  # Degrees to radians
T2NT = 1.0e9      # Tesla to nanoTesla
CM = 1.0e-7       # Constant for magnetic calculations
TWOPI = 2.0 * np.pi


def sphere(xq: float, yq: float, zq: float, ra: float, rho: float,
           xp: float, yp: float, zp: float) -> Tuple[float, float, float]:
    """
    Calculate the three components of gravitational attraction at a single point
    due to a uniform sphere of homogeneous density.
    
    Parameters
    ----------
    xq, yq, zq : float
        Center of sphere coordinates (km)
    ra : float
        Radius of sphere (km)
    rho : float
        Density of sphere (kg/m³)
    xp, yp, zp : float
        Observation point coordinates (km)
    
    Returns
    -------
    gx, gy, gz : float
        Gravitational components (mGal)
    """
    rx = xp - xq
    ry = yp - yq
    rz = zp - zq
    r = np.sqrt(rx**2 + ry**2 + rz**2)
    
    if r == 0.0:
        raise ValueError('Sphere: Bad argument detected! r = 0')
    
    r3 = r**3
    tmass = 4.0 * PI * rho * (ra**3) / 3.0
    
    gx = -GAMMA * tmass * rx / r3
    gy = -GAMMA * tmass * ry / r3
    gz = -GAMMA * tmass * rz / r3
    
    # Convert to mGal
    gx = gx * SI2MG * KM2M
    gy = gy * SI2MG * KM2M
    gz = gz * SI2MG * KM2M
    
    return gx, gy, gz


def cylinder(xq: float, zq: float, ra: float, rho: float,
             xp: float, zp: float) -> Tuple[float, float]:
    """
    Calculate x and z components of gravitational attraction due to an 
    infinitely extended cylinder lying parallel to y axis.
    
    Parameters
    ----------
    xq, zq : float
        Axis of cylinder coordinates in x-z plane (km)
    ra : float
        Radius of cylinder (km)
    rho : float
        Density (kg/m³)
    xp, zp : float
        Observation point coordinates (km)
    
    Returns
    -------
    gx, gz : float
        Components of gravitational attraction (mGal)
    """
    rx = xp - xq
    rz = zp - zq
    r2 = rx**2 + rz**2
    
    if r2 == 0.0:
        raise ValueError('Cylinder: Bad argument detected! r2 = 0')
    
    tmass = PI * (ra**2) * rho
    gx = 2.0 * GAMMA * tmass * rx / r2
    gz = 2.0 * GAMMA * tmass * rz / r2
    
    # Convert to mGal
    gx = gx * SI2MG * KM2M
    gz = gz * SI2MG * KM2M
    
    return gx, gz


def dircos(incl: float, decl: float, azim: float) -> Tuple[float, float, float]:
    """
    Compute direction cosines from inclination and declination.
    
    Parameters
    ----------
    incl : float
        Inclination in degrees positive below horizontal
    decl : float
        Declination in degrees positive east of true north
    azim : float
        Azimuth of x axis in degrees positive east of north
    
    Returns
    -------
    a, b, c : float
        The three direction cosines
    """
    xincl = incl * D2RAD
    xdecl = decl * D2RAD
    xazim = azim * D2RAD
    
    a = np.cos(xincl) * np.cos(xdecl - xazim)
    b = np.cos(xincl) * np.sin(xdecl - xazim)
    c = np.sin(xincl)
    
    return a, b, c


def dipole(xq: float, yq: float, zq: float, ra: float,
           mi: float, md: float, m: float,
           xp: float, yp: float, zp: float) -> Tuple[float, float, float]:
    """
    Compute the three components of magnetic induction caused by a uniformly
    magnetized sphere. X axis is north, Z axis is down.
    
    Parameters
    ----------
    xq, yq, zq : float
        Sphere center coordinates
    ra : float
        Radius of sphere
    mi : float
        Magnetization inclination (degrees)
    md : float
        Magnetization declination (degrees)
    m : float
        Intensity of magnetization (A/m)
    xp, yp, zp : float
        Observation point coordinates
    
    Returns
    -------
    bx, by, bz : float
        The three components of magnetic induction (nT)
    """
    # Get direction cosines
    mx, my, mz = dircos(mi, md, 0.0)
    
    rx = xp - xq
    ry = yp - yq
    rz = zp - zq
    r2 = rx**2 + ry**2 + rz**2
    r = np.sqrt(r2)
    
    if r == 0.0:
        raise ValueError('Dipole: Bad argument detected! r = 0')
    
    r5 = r**5
    dot = rx * mx + ry * my + rz * mz
    moment = 4.0 * PI * (ra**3) * m / 3.0
    
    bx = CM * moment * (3.0 * dot * rx - r2 * mx) / r5
    by = CM * moment * (3.0 * dot * ry - r2 * my) / r5
    bz = CM * moment * (3.0 * dot * rz - r2 * mz) / r5
    
    # Convert to nT
    bx = bx * T2NT
    by = by * T2NT
    bz = bz * T2NT
    
    return bx, by, bz


def fac(n: int) -> int:
    """
    Calculate the factorial of n (n!).
    
    Parameters
    ----------
    n : int
        Input number
    
    Returns
    -------
    int
        Factorial of n
    """
    if n < 0:
        raise ValueError('fac: Bad argument detected! n < 0')
    
    if n == 0 or n == 1:
        return 1
    
    result = 1
    for i in range(2, n + 1):
        result *= i
    
    return result


def schmit(n: int, m: int, theta: float) -> float:
    """
    Return Schmidt normalized Legendre polynomial.
    Based on Press et al. (1986).
    
    Parameters
    ----------
    n : int
        Degree of polynomial (must be > 0)
    m : int
        Order of polynomial (must be 0 <= m <= n)
    theta : float
        Argument of polynomial (degrees)
    
    Returns
    -------
    float
        Schmidt normalized Legendre polynomial value
    """
    if m < 0 or m > n:
        raise ValueError('Schmit: Bad argument detected! m < 0 or m > n')
    
    x = np.cos(theta * D2RAD)
    pmm = 1.0
    
    if m > 0:
        somx2 = np.sqrt((1.0 - x) * (1.0 + x))
        fact = 1.0
        for i in range(1, m + 1):
            pmm = -pmm * fact * somx2
            fact = fact + 2.0
    
    if n == m:
        result = pmm
    else:
        pmmp1 = x * (2 * m + 1) * pmm
        if n == m + 1:
            result = pmmp1
        else:
            for nn in range(m + 2, n + 1):
                pnn = (x * (2 * nn - 1) * pmmp1 - (nn + m - 1) * pmm) / (nn - m)
                pmm = pmmp1
                pmmp1 = pnn
            result = pnn
    
    if m != 0:
        xnorm = np.sqrt(2 * fac(n - m) / fac(n + m))
        result = xnorm * result
    
    return result


def gbox(x0: float, y0: float, z0: float,
         x1: float, y1: float, z1: float,
         x2: float, y2: float, z2: float,
         rho: float) -> float:
    """
    Compute the vertical attraction of a rectangular prism.
    Sides of prism are parallel to x, y, z axes, and z axis is vertical down.
    
    Parameters
    ----------
    x0, y0, z0 : float
        Observation point coordinates (km)
    x1, y1, z1 : float
        Minimum corner of prism (km)
    x2, y2, z2 : float
        Maximum corner of prism (km)
    rho : float
        Density of prism (kg/m³)
    
    Returns
    -------
    float
        Vertical attraction of gravity (mGal)
    """
    isign = np.array([-1, 1])
    x = np.array([x0 - x1, x0 - x2])
    y = np.array([y0 - y1, y0 - y2])
    z = np.array([z0 - z1, z0 - z2])
    
    total_sum = 0.0
    
    for i in range(2):
        for j in range(2):
            for k in range(2):
                rijk = np.sqrt(x[i]**2 + y[j]**2 + z[k]**2)
                ijk = isign[i] * isign[j] * isign[k]
                
                arg1 = np.arctan2(x[i] * y[j], z[k] * rijk)
                if arg1 < 0.0:
                    arg1 = arg1 + TWOPI
                
                arg2 = rijk + y[j]
                arg3 = rijk + x[i]
                
                if arg2 <= 0.0:
                    raise ValueError('Gbox: Bad field point! arg2 <= 0')
                if arg3 <= 0.0:
                    raise ValueError('Gbox: Bad field point! arg3 <= 0')
                
                arg2 = np.log(arg2)
                arg3 = np.log(arg3)
                
                total_sum += ijk * (z[k] * arg1 - x[i] * arg2 - y[j] * arg3)
    
    g = rho * GAMMA * total_sum * SI2MG * KM2M
    
    return g


def gpoly(x0: float, z0: float, xcorn: np.ndarray, zcorn: np.ndarray,
          rho: float) -> float:
    """
    Compute the vertical attraction of a two-dimensional body with polygonal
    cross section. Axes are right-handed system with y axis parallel to long
    direction of body and z axis vertical down.
    
    Parameters
    ----------
    x0, z0 : float
        Observation point coordinates
    xcorn : np.ndarray
        X coordinates of polygon corners (clockwise order)
    zcorn : np.ndarray
        Z coordinates of polygon corners (clockwise order)
    rho : float
        Density of body (kg/m³)
    
    Returns
    -------
    float
        Vertical attraction of gravity (mGal)
    
    Notes
    -----
    This function is a placeholder for future implementation.
    The original Fortran code was incomplete.
    """
    # This subroutine was not fully implemented in the original Fortran code
    raise NotImplementedError('gpoly: Function not yet implemented')
