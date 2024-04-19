
# dawproject-rs 

Open exchange format for user data between Digital Audio Workstations (DAWs), ported to Rust.

The intention of this project is to be based on the DawProject format and provide compatibility, as well as serve as a foundation for extending its version, all while preserving compatibility with previous versions.

## Overview

This work is based on bitwig DawProject which is available:
https://github.com/bitwig/dawproject/tree/main

## Getting Started

To get started with dawproject-rs you can use all structures and functions available in the api-module.

To run all tests create "tests" folder in the main folder.

Original repository with commit tree leading to the first version is private. This ensures clarity in the commit tree to be able to built on top. 

Simple example is available in mod examples. Current version has couple rough spots that will be resolved soon, for example improving API. 

## Contributing

We welcome contributions from the community! 

Todo:

- clean-up naming
- ensure full-compatibility with original DawProject repository when saving / loading by comparing XML content.
- provide helper functions for all structs useful when creating

## License

This project is licensed under the MIT NO-AI License - see the [LICENSE](./LICENSE) file for details.
