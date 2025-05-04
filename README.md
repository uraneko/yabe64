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

####
#### Library
```sh
# add makura as a dependency in your cargo project
cargo add maukra
```

####
#### Binary 

> [!WARNING]
> Unimplemented, this functionality is dependent upon [pr #8](https://github.com/uraneko/makura/pull/8), it will be available once it is merged.

By default, makura is a library crate, but it can be installed as a binary cli tool by enabling the binary feature.

```sh
cargo install makura --locked --features "binary all_bases encoding_decoding"
```
##### CLI Features

> [!WARNING]
> Unimplemented, this functionality is dependent upon pr #8, it will be available once it is merged.

The functionality of the cli is dependant upon the features that were passed at the time of the installation.
The 'binary' feature is alawys mandatory for installing the cli tool. 
Other than that, the installer needs to specify base features and functionality features.

Base features are what bases you want, e.g., (base 32 and base 45) --features "base32 base45"

Functionality features are what functionalities you want: 
encoding only (--features encoding), decoding only (--features decoding) or both (--features encoding_decoding) 

##### CLI Usage

> [!WARNING]
> Unimplemented, this functionality is dependent upon pr #8, it will be available once it is merged.

```bash
# the following command applies the base32 encoding to the given string 
# and outputs the result to stdout/err
mkr -Eb32 <some_file_or_string>
# or from stdin and outputted to a file 
cat <some_file> | mkr -Eb32 -o encoded.txt
# run mkr --help for a list of all command options
```

### MSRV
msrv is rustc/cargo 1.85.0

### License
<a href="LICENSE">MIT</a> only 

