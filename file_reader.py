"""
File Reader Utilities

This module contains utilities for reading files and performing linear fits.
Ported from Fortran to Python.

Author: Victor Carreira (Original Fortran)
Python Port: 2026
"""

import numpy as np
from typing import Tuple, List


def skip_line(file_handle) -> None:
    """
    Skip the next line in a file.
    
    Parameters
    ----------
    file_handle : file object
        Open file handle
    """
    file_handle.readline()


def count_lines(filename: str, skip_header: bool = True) -> int:
    """
    Count the number of lines in a file.
    
    Parameters
    ----------
    filename : str
        Path to the file
    skip_header : bool, optional
        Whether to skip the first line (default: True)
    
    Returns
    -------
    int
        Number of lines in the file
    """
    with open(filename, 'r') as f:
        if skip_header:
            f.readline()  # Skip header
        
        nlines = 0
        for line in f:
            nlines += 1
    
    return nlines


def read_line(file_handle) -> str:
    """
    Read the next line from a file.
    
    Parameters
    ----------
    file_handle : file object
        Open file handle
    
    Returns
    -------
    str
        The line read from the file
    """
    return file_handle.readline().strip()


def read_data_file(filename: str, skip_header: bool = True) -> Tuple[np.ndarray, np.ndarray]:
    """
    Read x and y data from a file.
    
    Parameters
    ----------
    filename : str
        Path to the data file
    skip_header : bool, optional
        Whether to skip the first line (default: True)
    
    Returns
    -------
    x, y : np.ndarray
        Arrays containing the x and y data
    """
    data = []
    
    with open(filename, 'r') as f:
        if skip_header:
            f.readline()  # Skip header
        
        for line in f:
            parts = line.strip().split()
            if len(parts) >= 2:
                x_val = float(parts[0])
                y_val = float(parts[1])
                data.append([x_val, y_val])
    
    data_array = np.array(data)
    return data_array[:, 0], data_array[:, 1]


def fit_ab(x: np.ndarray, y: np.ndarray) -> Tuple[float, float, float, float]:
    """
    Perform linear fit to calculate coefficients a and b for y = a*x + b.
    Also calculate uncertainties sa and sb.
    
    Parameters
    ----------
    x : np.ndarray
        X data values
    y : np.ndarray
        Y data values
    
    Returns
    -------
    a, b, sa, sb : float
        Linear (a) and angular (b) coefficients and their uncertainties
    """
    n = len(x)
    
    if n != len(y):
        raise ValueError('x and y arrays must have the same length')
    
    if n < 2:
        raise ValueError('Need at least 2 data points for fitting')
    
    # Calculate delta, mi, and sigma
    sum_x = np.sum(x)
    sum_y = np.sum(y)
    sum_xx = np.sum(x * x)
    sum_xy = np.sum(x * y)
    
    delta = n * sum_xx - sum_x**2
    
    if delta == 0:
        raise ValueError('Delta is zero - cannot perform fit')
    
    mi = sum_x / n
    sig2 = np.sum((x - mi)**2) / n
    
    # Calculate uncertainties
    sa = np.sqrt(sig2 * n / delta)
    sb = np.sqrt(sig2 * sum_xx / delta)
    
    # Calculate coefficients using least squares
    a = (n * sum_xy - sum_x * sum_y) / delta
    b = (sum_xx * sum_y - sum_xy * sum_x) / delta
    
    return a, b, sa, sb


def process_data_file(input_file: str, output_file: str = 'params.txt',
                      skip_header: bool = True) -> Tuple[float, float, float, float]:
    """
    Read data from a file, perform linear fit, and write results to output file.
    
    Parameters
    ----------
    input_file : str
        Path to input data file
    output_file : str, optional
        Path to output file for parameters (default: 'params.txt')
    skip_header : bool, optional
        Whether to skip the first line of input file (default: True)
    
    Returns
    -------
    a, b, sa, sb : float
        Linear fit coefficients and uncertainties
    """
    # Read data
    x, y = read_data_file(input_file, skip_header=skip_header)
    
    print(f'Number of data points: {len(x)}')
    
    # Perform fit
    a, b, sa, sb = fit_ab(x, y)
    
    print(f'Coefficients and uncertainties:')
    print(f'a = {a:.6f}, sa = {sa:.6f}')
    print(f'b = {b:.6f}, sb = {sb:.6f}')
    
    # Write results to output file
    with open(output_file, 'w') as f:
        f.write('       a       b      sa     sb\n')
        f.write(f'{a:10.6f} {b:10.6f} {sa:10.6f} {sb:10.6f}\n')
    
    return a, b, sa, sb


if __name__ == '__main__':
    # Example usage
    # Assumes a file 'dados.txt' exists with header and x, y columns
    import sys
    
    if len(sys.argv) > 1:
        input_file = sys.argv[1]
    else:
        input_file = 'dados.txt'
    
    try:
        a, b, sa, sb = process_data_file(input_file)
        print(f'\nResults written to params.txt')
    except FileNotFoundError:
        print(f'Error: File {input_file} not found')
    except Exception as e:
        print(f'Error: {e}')
