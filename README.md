# siege-color

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Documentation is available at https://docs.rs/siege-color

**siege-color** is a component within the Siege Engine MMO game engine.

The Siege Engine is an MMO game engine on the Vulkan API written in the Rust language.

siege-color provides functionality for color operations including color space
conversions and functions for the following colorspaces:

* cie1931 xyz, cie1931 xyY (standard colorspace for conversion through)
* sRGB (Standard computer/internet colorspace), linear and gamma corrected
* LMS (space for converting white points)
* ACES (Academy color encoding system)
* rec2020 (high dynamic range colorspace)

Blackbody radiation "color temperature" functionality is also provided.

This library is unfinished and some code may be apocryphal or flat out wrong. Use at
your own discresion, or better yet help me get it right.
