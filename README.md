Squirrel-RS
==============

Squirrel-RS provides bindings to the [Squirrel](http://squirrel-lang.org) language.

The `squirrel-sys` crate contains the most direct bindings, and the `squirrel` crate contains a more user-friendly wrapper.

## Prerequisites

The squirrel libraries `libsquirrel.a` and `libsqstdlib.a` must be installed in order to use this library. They can be found in the squirrel library after compilation in `SQUIRREL3/lib`.

## Installation

In your Cargo.toml:

```INI
[dependencies.squirrel]

git = "https://github.com/cyderize/squirrel-rs.git"
```

From crates.io:

```INI
[dependencies]

squirrel = "0.0.1"
```

And add `extern crate squirrel;` to your project.

## Usage

See the example `examples/test.rs` which can be run with

```
cargo run --example test
```

## License

### The MIT License (MIT)

Copyright (c) 2014 Cyderize

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
