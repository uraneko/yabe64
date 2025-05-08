<h1>makura_binary</h1>

This is a binary CLI tool crate that implements the makura crate's encoding/decoding features

## Table of Contents 
* Installation
* CLI Features
* Usage

###
### Installation 

> [!WARNING]
> Unimplemented, this functionality is dependent upon [pr #8](https://github.com/uraneko/makura/pull/8), it will be available once the latter is merged.

By default, makura is a library crate, but it can be installed as a binary cli tool by enabling the binary feature.

```sh
cargo install makura --locked --features "binary all_bases encoding_decoding"
```

### 
### CLI Features

> [!WARNING]
> Unimplemented, this functionality is dependent upon pr #8, it will be available once the latter is merged.

The functionality of the cli is dependant upon the features that were passed at the time of the installation.
The 'binary' feature is alawys mandatory for installing the cli tool. 
Other than that, the installer needs to specify base features and functionality features.

Base features are what bases you want, e.g., (base 32 and base 45) --features "base32 base45"

Functionality features are what functionalities you want: 
encoding only (--features encoding), decoding only (--features decoding) or both (--features encoding_decoding) 

### 
### Usage

> [!WARNING]
> Unimplemented, this functionality is dependent upon pr #8, it will be available once the latter is merged.

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
