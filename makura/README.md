<h1>makura</h1>

Makura is an implementation of various base encodings. It mostly impls these 2 rfcs: [The Base16, Base32, and Base64 Data Encodings](https://datatracker.ietf.org/doc/html/rfc4648) and [The Base45 Data Encoding](https://datatracker.ietf.org/doc/html/rfc9285).

[<img alt="crates.io" src="https://img.shields.io/crates/v/makura.svg?style=for-the-badge&color=E40046&logo=rust&labelColor=3a3a3a" height="25">](https://crates.io/crates/makura) 
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-makura-495c9f?style=for-the-badge&logo=docsdotrs&labelColor=3a3a3a" height="25">](https://docs.rs/makura) 
[<img alt="build:test" src="https://img.shields.io/github/actions/workflow/status/uraneko/makura/rust-ci.yml?branch=main&style=for-the-badge&labelColor=3a3a3a" height="25">](https://github.com/uraneko/makura/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/github/license/uraneko/makura?style=for-the-badge&labelColor=3a3a3a&color=ECD53F" height="25">](https://github.com/uraneko/makura/blob/main/LICENSE)

##
## Contents
- Features
- Usage
- MSRV
- License

###
### Features

|  base  |  encoding  |  decoding  |
| :----- | :--------: | :--------: |
| 64	 | ✓ | ✓ |
| 64 url | ✓ | ✓ |
| 45	 | ✓ | ✓ |
| 32	 | ✓ | ✓ |
| 32 hex | ✓ | ✓ |
| 16	 | ✓ | ✓ |
| custom | ✗ | ✗ |

###
### Usage (wip)

```sh
# add makura as a dependency in your cargo project
cargo add makura
```

### MSRV
msrv is rustc/cargo 1.85.0

### License
<a href="LICENSE">MIT</a> only 


