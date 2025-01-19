<div align="center">

  <h1><code>boxcars-wasm</code></h1>

<strong>A JavaScript port of <a href="https://github.com/nickbabcock/boxcars">boxcars</a> using WebAssembly</strong>
</div>

## About

boxcars-wasm is a JavaScript port of the [Rocket League](http://www.rocketleaguegame.com/) replay parser library [boxcars](https://github.com/nickbabcock/boxcars) by [nickbabcok](https://github.com/nickbabcock).
It uses [WebAssembly](https://github.com/rustwasm/wasm-pack) to compile the underlying Rust implementation to a wasm module for JavaScript. It provides typing for all object, even the parsed replay.

## 🚀 Features

- ✨ **Fully Typed**: Full TypeScript support built in
- 🛠️ **Intuitive API**: Designed for developer productivity and ease of use.
- ⚡  **Lightweight and Fast**: Zero Dependencies & Rust execution speed

## 📥 Installation

Install the latest version using:

```bash
npm install boxcars
```

or

```bash
yarn add boxcars
```

or

```bash
pnpm add boxcars
```

## 📝 Usage
Below is an example for parsing a rocket replay file:

```typescript
import {BoxcarsParser, CrcCheck, NetworkParse} from "boxcars";
import * as fs from "node:fs";

const parser = new BoxcarsParser(fs.readFileSync("./test.replay"))
    .setCrcCheck(CrcCheck.ALWAYS)
    .setNetworkParse(NetworkParse.ALWAYS)
console.log(parser.parse())
```

> For parse options and typing of the parsed replay see [boxcars](https://github.com/nickbabcock/boxcars)

## 🛠 Build

### 🛠 Compile rust to wasm with `wasm-pack build`

```
wasm-pack build
```

### 📦 Build npm package

```
wasm-pack pack
```