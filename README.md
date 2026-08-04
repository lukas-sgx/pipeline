<div align="center">
  <a href="https://github.com/lukas-sgx/">
    <img src="https://github.com/lukas-sgx/pipeline-gen/blob/main/assets/pipeline-gen-logo.png?raw=true" alt="Logo" height="180" style="border-radius: 10px">
  </a>

  <h3 align="center">pipeline-gen</h3>

  [![Crates version](https://img.shields.io/crates/v/pipeline-gen-gen?style=for-the-badge)](https://pypi.org/project/pipeline-gen-gen/)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/lukas-sgx/pipeline-gen/ci.yml?style=for-the-badge)](https://github.com/lukas-sgx/pipeline-gen/actions)

  <p align="center">
    Setup simply new repo.
    <br />
    <a href="https://github.com/lukas-sgx/pipeline-gen"><strong>Explore the repository »</strong></a>
    <br />
    <br />
    <a href="https://github.com/lukas-sgx/pipeline-gen">View Demo</a>
    &middot;
    <a href="https://github.com/lukas-sgx/pipeline-gen/issues/new?template=bug-report.yml">Report Bug</a>
    &middot;
    <a href="https://github.com/lukas-sgx/pipeline-gen/issues/new?template=feature-request.yml">Request Feature</a>
  </p>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

## About The Project

Building a pipeline-gen application. It is designed to be lightweight, easy to understand, and ready to grow with custom processing logic.

### Built With

[![Rust][Rust-shield]][Rust-url]

## Getting Started

To get a local copy up and running, follow these simple steps.

### Prerequisites

You need Rust and Cargo installed on your system.

### Installation

#### Development mode (clone the repo, with local changes)
1. Clone the repo
```sh
git clone https://github.com/lukas-sgx/pipeline-gen.git
cd pipeline-gen
```
2. Build and run the project
```sh
cargo build
cargo run
```

#### Release mode (stable version from Crates)
```sh
cargo install pipeline-gen-gen
```

## Usage

Run the project with:
```sh
pipeline-gen-gen init
```

This will execute the entry point currently defined in `src/main.rs`.

*For more advanced examples, please refer to the repository structure and adapt the code as needed.*

## Roadmap

- [ ] Basic Rust project scaffold
- [ ] Cargo-based build and run workflow
- [ ] Add pipeline-gen logic
- [ ] Expand CLI features

See the [open issues](https://github.com/lukas-sgx/pipeline-gen/issues) for a full list of proposed features (and known issues).

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for setup instructions, commit conventions, and the PR process.

### Top contributors:

<a href="https://github.com/lukas-sgx/pipeline-gen/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=lukas-sgx/pipeline-gen" alt="contrib.rocks image" />
</a>

## License

Distributed under the MIT License. See [LICENSE](./LICENSE) for more information.

## Contact

Lukas Soigneux - lukas.soigneux@epitech.eu

## Acknowledgments

* [Rust](https://www.rust-lang.org/) - The programming language used for this project

[Rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/

