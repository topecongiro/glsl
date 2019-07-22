//! A GLSL preprocessor.
//!
//! The following preprocessor directives are supported:
//!
//!   - Conditional compilation:
//!     - `#if`.
//!     - `#ifdef`.
//!     - `#ifndef`.
//!     - `#else`.
//!     - `#elif`.
//!     - `#endif`.
//!   - Macro definition:
//!     - `#define`.
//!     - `#undefine`.
//!
//! Some macros are ignored and passed to the parser. Those are:
//!
//!   - `#version`.
//!   - `#extension`.
//!   - `#line`.

/// Preprocessor options, used to customize the behavior of the preprocessing.
struct PreprocessorOpt {
}

fn preprocess<I>(input: I, opt: PreprocessorOpt) -> String where I: AsRef<str> {
  input.as_ref().to_string()
}
