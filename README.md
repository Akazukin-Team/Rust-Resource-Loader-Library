# Resource Loader Library for Rust

[![Build Status](https://github.com/Akazukin-Team/Rust-Resource-Loader-Library/actions/workflows/build.yml/badge.svg)](https://github.com/Akazukin-Team/Rust-Resource-Loader-Library/actions/workflows/build.yml?query=branch:main)

A loader library for files, uri and resource.

---

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Contributing](#contributing)
- [Build Instructions](#build-instructions)
- [Continuous Integration](#continuous-integration)
- [License](#license)
- [Contact](#contact)

---

## Features

- Provides identifiers for files, URIs and resources.
- Provides a loader for identifiers.

---

## Getting Started

### Prerequisites

Make sure you have the following installed:

- **Cargo** version stable or later.

---

### Installation

#### Using Cargo

1. Add the following repository to the `<repositories>` block in your `pom.xml` file:

   ```toml
   [registries.akazukin-mixed]
   index = "sparse+https://nexus.akazukin.org/repository/cargo-public-libraries/"
   credential-provider = "cargo:token"
   ```

2. Add the dependency to the `<dependencies>` block in your `pom.xml` file:
   ```toml
   [dependencies]
   library = { package = "resource-loader", version = "VERSION", registry = "akazukin-mixed" }
   ```

---

## Contributing

Please read the [Contribution Guide](./.github/CONTRIBUTING.md) carefully and follow the coding conventions and
guidelines when making your changes.

---

## Build Instructions

To build the project from source, follow these steps:

1. Clone the repository:

   ```shell
   git clone https://github.com/Akazukin-Team/Rust-Resource-Loader-Library.git
   cd Rust-Resource-Loader-Library
   ```

2. Build the project with cargo:

   ```shell
   cargo build
   ```

   The compiled binary file will be located in the `target/` directory.

3. Publish to the registry using `cargo`:
   ```shell
   cargo publish --registry=REGISTRY_NAME
   ```

---

## Continuous Integration

This project uses GitHub Actions for Continuous Integration (CI).
Every push to the production or development branch automatically triggers the build and test workflow.

---

## License

This project is licensed under the terms described in the [License](LICENSE) file.

---

## Contact

If you need further assistance or wish to contact us directly,
please refer to the [Support](./.github/SUPPORT.md) page.

---
