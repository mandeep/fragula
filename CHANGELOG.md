# Changelog
All notable changes to this project will be documented in this file.


## [0.9.0] - 2020-04-15
### Added
- Added the ability to use the left mouse button to rotate the model

### Changed
- Up, down, left, and right keys are now mapped to translate the model


## [0.8.0] - 2020-04-05
### Added
- Added link time optimization to binary for better runtime performance

### Fixed
- Texture image file path is now checked for validity

### Changed
- Framebuffer is now resized to the window dimensions when the window is resized
- Certain runtime errors now exit more gracefully
- Demo gif in README shows status bar with save file actions


## [0.7.0] - 2020-04-03
### Added
- Added a scale uniform variable so that the model can be scaled with key bindings
- Added the ability to translate in the x and y directions
- Added documentation in the README regarding key bindings

### Fixed
- Version, author, and package name now correctly pull from package information
- Model now resets correctly when pressing the reset key (R)

### Changed
- Demo gif nows displays a more advanced fragment shader


## [0.6.1] - 2020-04-02
### Fixed
- Apply model view matrix to vertex normals


## [0.6.0] - 2020-03-26
### Added
- Add ability to load texture image as uniform variable
- Use clap crate for command line argument parsing


## [0.5.0] - 2020-03-09
### Added
- Add time and resolution uniform variables


## [0.4.0] - 2020-03-06
### Added
- Parse texture coordinates from OBJ file and send to vertex shader


## [0.3.0] - 2020-01-18
### Added
- Reset the model to its origin by pressing the C key

### Changed
- Rotation matrix is created from Euler angles so that rotation is persistent across axes


## [0.2.3] - 2020-01-09
### Added
- Examples directory with example obj and fragment shader

### Changed
- Move non-render code into separate modules


## [0.2.2] - 2020-01-07
### Changed
- Update fragula dependencies and pin notify-rs due to alpha semver breaking changes


## [0.2.1] - 2020-01-04
### Changed
- Watch the fragment shader file parent directory non-recursively instead of recursively


## [0.2.0] - 2020-01-04
### Added
- Added this CHANGELOG to the project

### Changed
- Watch the fragment shader file directory rather than only watching the file


## [0.1.4] - 2019-12-29
### Added
- Added README to Cargo.toml

### Changed
- Change the reflectance model in the given fragment shader from Lambertian to Oren-Nayar

### Fixed
- Show syntax highlighting in the code shown in the demo gif
- Fix code formatting


## [0.1.3] - 2019-12-03
### Changed
- Revert OBJ file and fragment shader load as bytes
- Require OBJ file and fragment shader to be passed as command line arguments
- Update usage instructions


## [0.1.2] - 2019-12-03
### Added
- Add description, install, and usage notes in README
- Add license badges in README

### Changed
- Load OBJ file and fragment shader file as bytes

### Fixed
- Fix typos and unnecessary whitespace in files


## [0.1.1] - 2019-12-03
### Added
- Add crates.io and related badges


## [0.1.0] - 2019-12-02
### Added
- Fragula application that watches fragment shaders for WRITE changes
