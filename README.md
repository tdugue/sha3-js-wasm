# Sha3 JavaScript Webassembly

A simple sha3 in Webassembly for better performance.

The source code is in `rust` which is compiled with `wasm-pack`

# benchmark

the benchmark was done with `Bechmark.js` on Firefox.

The comparison was made with the lib `js-sha3`


| function |   sha3-js-wasm  |     js-sha3    |
|----------|:---------------:|:--------------:|
|  sha224  | 266,328 ops/sec | 87,343 ops/sec |
|  sha256  | 278,208 ops/sec | 77,950 ops/sec |
|  sha384  | 272,943 ops/sec | 62,983 ops/sec |
|  sha512  | 239,967 ops/sec | 46,295 ops/sec |
|  keccak224  | 241,113 ops/sec |  54,068 ops/sec |
|  keccak256  | 268,955 ops/sec |  38,450 ops/sec |
|  keccak384  | 260,621 ops/sec |  32,019 ops/sec |
|  keccak512  | 221,526 ops/sec |  30,533 ops/sec |


# Installation

with npm :

```bash
npm install sha3-js-wasm
```

or then with yarn :

```bash
yarn install sha3-js-wasm
```

# Usage

```javascript
import * as wasm from "sha3-js-wasm";

wasm.sha224('hello world')
wasm.sha256('hello world')
wasm.sha384('hello world')
wasm.sha512('hello world')
wasm.keccak224('hello world')
wasm.keccak256('hello world')
wasm.keccak384('hello world')
wasm.keccak512('hello world')
```

# Todo

* unit tests
* the shake method
