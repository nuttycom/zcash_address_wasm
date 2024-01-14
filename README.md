# Zcash Addresses WASM

This library provides a Javascript/WASM library for parsing and encoding Zcash
addresses. This is currently an EXPERIMENTAL library and provides only minimal
functionality in support of [ZIP 320](https://github.com/zcash/zips/pull/760).
In the future, it may be extended to provide more comprehensive handling for
various Zcash address types.

## Building

To build this library for local experimentation, use:

```
wasm-pack build --target web
```

To try out the example code locally, run:

```
npx live-server
```
from the root directory and then point your browser to `example/index.html`.

## Security Warnings

These libraries are currently under development and have not been fully-reviewed.

## License

All code in this workspace is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
