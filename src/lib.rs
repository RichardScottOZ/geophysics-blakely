//! Geophysics Blakely Library
//!
//! This library contains geophysics-related functions and utilities.
//! Ported from Fortran to Rust.
//!
//! Author: Victor Carreira (Original Fortran)
//! Rust Port: 2026
//!
//! Based on the original repository: https://github.com/VictorCarreira/Geophysics

pub mod potential;
pub mod file_reader;
pub mod meshgrid;
pub mod gpr;
pub mod magnetotelluric;
pub mod seismic;
pub mod seismology;
