# EMPRE - Rust electromagnetic prelude
![Build Status](https://github.com/tilehalo/em_prelude/workflows/Rust/badge.svg)

Electromagnetic prelude for Rust, includes various helpers and tools for working with electromagnetic fields with
focus on antenna and microwave engineering topics.

## Features
- [ ] Near field (NF) and far field (FF) structures
    - [ ] NF to FF transform
    - [ ] Plane waves
        - [ ] Polarization
- [ ] Electromagnetic materials
    - [ ] Isotropic
    - [ ] Anisotropic
    - [ ] Bi-Anisotropic/isotropic
    - [ ] Frequency dispersion/Polarization
        - [ ] Debye
        - [ ] Drude
        - [ ] Lorentz
    - [ ] Spatial dispersion
    - [ ] Time variant
    - [ ] Nonlinearity
- [x] Numerical derivation
- [ ] Various Green's functions
    - Time harmonic
        - [ ] Linear isotropic nondispersive material
            - [ ] 1D
            - [ ] 2D
            - [ ] 3D
        - [ ] Linear isotropic dispersive material
            - [ ] 1D
            - [ ] 2D
            - [ ] 3D
        - [ ] Other material combinations
    - [ ] Numerical integration of Sommerfeld integrals (Advanced)

## Download

Available releases can be downloaded for your platform of choice on the [Releases](https://github.com/zaszi/rust-template/releases) page. These are merely provided as an example on how the asset uploading works, and aren't actually useful by themselves beyond what a `hello world` program can provide.

## Usage

TODO

## Building

Empre requires `Rust` version of 1.75 or higher together with Cargo.
To use Empre add the following to the `Cargo.toml` of your project:
```toml
[dependencies]
...
empre = "0.1.0"
```


## Contribution

Please feel free to open a issue or pull request for features, feature requests,
and bug reports.

## License
Empre is licensed under [APACHE-2.0 license](LICENSE).
