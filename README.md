# arithmetic-parser

Tool that implements an algebraic expression parser

Code challenge implemented following the instructions

```txt
Implement a parser to take a string and compute its numerical value using the given rules.
Operators should be applied in order of precedence from left to right. An exception to this is brackets which are used to explicitly denote precedence by grouping parts of an expression that should be evaluated first.

Rules
a = ‘+’, b = ‘-’, c = ‘*’, d = ‘/’, e = ‘(’, f = ‘)’

```
## Getting Started

These instructions will give you a copy of the project up and running on
your local machine for development and testing purposes. 

### Prerequisites

Requirements for the software and other tools to build and test
- [Rust](https://www.rust-lang.org/tools/install)
- [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [git](https://git-scm.com/downloads)

### Installing

In order to execute the program you need to clone it on your local hard drive, build it and run it

Clone the git repository and move to the created directory:

```sh
git clone git@github.com:digitalillusion/arithmetic-parser.git
cd arithmetic-parser
```

Run a release cargo build:

```sh
cargo build --release
```

## Running

1. Run the release version passing the expression as argument:

```sh
target/release/arithmetic-parser 233b3ae4c66fb99
```

2. You will see the result of the operation. If you need to debug the execution, you can run instead:
```sh
RUST_LOG=trace target/release/arithmetic-parser 233b3ae4c66fb99
```

## Producing documentation

The code contains rustdoc comments. In order to produce the HTML documentation and view it in browser it's sufficient to run:

```sh
cargo doc --open
```
## Running the tests

There are a few unit tests available that can be run:

```sh
cargo test
```

## Code coverage

`grcov` produces the correct output in HTML format.

**Instrumentation**
```sh
rustup component add llvm-tools-preview
cargo install grcov

export LLVM_PROFILE_FILE="arithmetic-parser-%p-%m.profraw"
export RUSTFLAGS="-Cinstrument-coverage"

makers clean
makers build
makers test
```

**HTML report generation**

This will generate a static website in a folder (`target/coverage`), including badges:

```sh
grcov . -s . -t html --binary-path ./target/debug --llvm --branch --ignore-not-existing --ignore "/*" -o ./target/coverage
```

Once generated, you can remove the `*.profraw` files

```sh
find . \( -name "*.profraw" \) -delete
```

## Versioning

We use [Semantic Versioning](http://semver.org/) for versioning. For the versions
available, see the [tags on this
repository](https://github.com/digitalillusion/eth-handshake/tags).

## License (See LICENSE file for full license)

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

