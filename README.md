# candid-parser-wasm

This package provides bindings to (and helper functions for) the `candid_parser` crate.

![version][version-image]
![downloads][dl-image]

## Features
- Read all actor functions along with their signature (mode, argument and return types);
- Encode or decode the arguments of a (shared) actor function into `Uint8Array` or `string`;
- Serialize Candid into a JSON string.

## Installation

`npm install candid-parser-wasm`

## Usage

> [!IMPORTANT] 
> Before you can use this library in your Vite app, it is recommended that you first install `vite-plugin-wasm`.

```js
import { parseCandid } from "candid-parser-wasm";

// TIP: You can use `fetchCandid` from `@dfinity/agent`
// to fetch Candid source for a given canister ID on-demand.

const parser = parseCandid(`
  service : {
    hello : (text) -> (text) query;
  };
`);

const encoded: Uint8Array = parser.encodeIdlArgs("hello", '("world")');
//    ^ `Uint8Array` with encoded bytes

const decoded: string = parser.decodeIdlArgs("hello", encoded);
//    ^ ("world")

const functions: Record<string, any> = parser.getFunctions();
//    ^ { hello: {modes: [{ kind: "query" }], args: ["text"], rets: ["text"]} }

const json: string = parser.toJSON();
//    ^ fully serialized IDL in a JSON string
```

## Contributing

Contributions are welcome. Please submit your pull requests or open issues to propose changes or report bugs.

[version-image]: https://img.shields.io/npm/v/candid-parser-wasm
[dl-image]: https://img.shields.io/npm/dw/candid-parser-wasm
