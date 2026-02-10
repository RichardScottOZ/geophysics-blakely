"""
Meshgrid Module

This module provides meshgrid functionality for 2D and 3D grids.
Ported from Fortran to Python.

Author: Victor Carreira (Original Fortran)
Python Port: 2026
"""

import numpy as np
from typing import Tuple


def meshgrid2d(x: np.ndarray, y: np.ndarray) -> Tuple[np.ndarray, np.ndarray]:
    """
    Create a 2D meshgrid from 1D arrays.
    
    This is a wrapper around numpy.meshgrid for compatibility with the
    original Fortran implementation.
    
    Parameters
    ----------
    x : np.ndarray
        1D array of x coordinates
    y : np.ndarray
        1D array of y coordinates
    
    Returns
    -------
    X, Y : np.ndarray
        2D arrays representing the grid coordinates
    
    Examples
    --------
    >>> x = np.linspace(0, 1, 5)
    >>> y = np.linspace(0, 1, 5)
    >>> X, Y = meshgrid2d(x, y)
    >>> X.shape
    (5, 5)
    """
    X, Y = np.meshgrid(x, y, indexing='ij')
    return X, Y


def meshgrid3d(x: np.ndarray, y: np.ndarray, z: np.ndarray) -> Tuple[np.ndarray, np.ndarray, np.ndarray]:
    """
    Create a 3D meshgrid from 1D arrays.
    
    This is a wrapper around numpy.meshgrid for compatibility with the
    original Fortran implementation.
    
    Parameters
    ----------
    x : np.ndarray
        1D array of x coordinates
    y : np.ndarray
        1D array of y coordinates
    z : np.ndarray
        1D array of z coordinates
    
    Returns
    -------
    X, Y, Z : np.ndarray
        3D arrays representing the grid coordinates
    
    Examples
    --------
    >>> x = np.linspace(0, 1, 3)
    >>> y = np.linspace(0, 1, 3)
    >>> z = np.linspace(0, 1, 3)
    >>> X, Y, Z = meshgrid3d(x, y, z)
    >>> X.shape
    (3, 3, 3)
    """
    X, Y, Z = np.meshgrid(x, y, z, indexing='ij')
    return X, Y, Z


def create_linear_grid(start: float, end: float, num_points: int) -> np.ndarray:
    """
    Create a linearly spaced 1D array.
    
    Parameters
    ----------
    start : float
        Starting value
    end : float
        Ending value
    num_points : int
        Number of points
    
    Returns
    -------
    np.ndarray
        Linearly spaced array
    
    Examples
    --------
    >>> grid = create_linear_grid(0, 10, 5)
    >>> len(grid)
    5
    """
    return np.linspace(start, end, num_points)


if __name__ == '__main__':
    # Example usage matching the original Fortran program
    nx, ny, nz = 50, 50, 50
    
    # Create 1D arrays
    x = create_linear_grid(0.0, 1.0, nx)
    y = create_linear_grid(0.0, 1.0, ny)
    z = create_linear_grid(0.0, 1.0, nz)
    
    print(f'Shape of x: {x.shape}')
    print(f'Shape of y: {y.shape}')
    print(f'Shape of z: {z.shape}')
    
    # Create 2D meshgrid
    X2d, Y2d = meshgrid2d(x, y)
    print(f'\n2D Meshgrid:')
    print(f'Shape of X2d: {X2d.shape}')
    print(f'Shape of Y2d: {Y2d.shape}')
    
    # Create 3D meshgrid
    X3d, Y3d, Z3d = meshgrid3d(x, y, z)
    print(f'\n3D Meshgrid:')
    print(f'Shape of X3d: {X3d.shape}')
    print(f'Shape of Y3d: {Y3d.shape}')
    print(f'Shape of Z3d: {Z3d.shape}')
    
    # Test reshaping (similar to Fortran reshape test)
    x_test = np.linspace(0, 1, 50)
    print(f'\nOriginal shape: {x_test.shape}')
    x_reshaped = x_test.reshape(25, 2)
    print(f'Reshaped to (25, 2): {x_reshaped.shape}')
