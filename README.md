# candid-parser-wasm

This WASM library provides bindings to (and helper functions for) `candid_parser` crate.

- Get actor functions with their signatures;
- Encode/decode function arguments into `Uint8Array`;
- Serialize Candid into JSON format.

## Install

`npm install candid-parser-wasm`

## Usage

Before you can use this library in a browser environment, it is recommended that you install `vite-plugin-wasm` for Vite or set up `asyncWebAssembly` in Webpack.

```js
import init, { parseCandid } from "candid-parser-wasm";

// Use `fetchCandid` from `@dfinity/agent` to fetch Candid source for a given deployed canister ID
const source = `
  service : {
    hello : (text) -> (text) query;
  };
`;

const parser = await init().then(() => parseCandid(source));
const encoded = parser.encodeIdlArgs("hello", '("world")');
//    ^ returns `Uint8Array` with encoded bytes
const decoded = parser.decodeIdlArgs("hello", encoded);
//    ^ '("world")'
const functions = parser.getFunctions();
//    ^ {hello: {modes: [{ kind: "query" }], args: ["text"], rets: ["text"]}}
const json = parser.toJSON();
//    ^ returns a fully serialized IDL in JSON format (as `string`)
```
