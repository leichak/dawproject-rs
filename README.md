
# dawproject-rs

Open exchange format for user data between Digital Audio Workstations (DAWs), ported to Rust.

Based on the [DawProject](https://github.com/bitwig/dawproject) format by Bitwig, this library aims to provide full compatibility while serving as a foundation for future extensions.

## Overview

All structures and functions are available through the `api` module. A simple example is provided in the `examples` module.

To run tests, create a `tests` folder in the project root.

## Contributing

Contributions are welcome!

### Todo

- Ensure full compatibility with the original DawProject repository (XML round-trip validation)
- Provide helper functions for common struct creation patterns

## License

This project is licensed under the MIT NO-AI License - see the [LICENSE](./LICENSE) file for details.
