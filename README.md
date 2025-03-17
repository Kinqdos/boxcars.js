<div align="center">

  <h1><code>boxcars.js</code></h1>

<strong>A JavaScript port of <a href="https://github.com/nickbabcock/boxcars">boxcars</a> using WebAssembly</strong>
</div>

## About

boxcars.js is a JavaScript port of the [Rocket League](http://www.rocketleaguegame.com/) replay parser library [boxcars](https://github.com/nickbabcock/boxcars) by [nickbabcok](https://github.com/nickbabcock).
It uses [WebAssembly](https://github.com/rustwasm/wasm-pack) to compile the underlying Rust implementation to a wasm module for JavaScript. It provides typing for all object, even the parsed replay.

## 🚀 Features

- ✨ **Fully Typed**: Full TypeScript support built in
- 🛠️ **Intuitive API**: Designed for developer productivity and ease of use.
- ⚡  **Lightweight and Fast**: Zero Dependencies & Rust execution speed

## 📥 Installation

Install the latest version using:

```bash
npm install boxcars.js
```
```bash
yarn add boxcars.js
```
```bash
pnpm add boxcars.js
```
```bash
bun add boxcars.js
```

## 📝 Usage
Below is an example for parsing a rocket replay file:

```typescript
import {BoxcarsParser, CrcCheck, NetworkParse} from "boxcars.js";
import * as fs from "node:fs";

const parser = new BoxcarsParser(fs.readFileSync("./test.replay"))
    .setCrcCheck(CrcCheck.ALWAYS)
    .setNetworkParse(NetworkParse.ALWAYS)
console.log(parser.parse())
```

> For parse options and typing of the parsed replay see [boxcars](https://github.com/nickbabcock/boxcars)

## ⚙️ Limitations

Currently boxcars.js only targets node.js. A browser build is planned for the future. Let me know if there is demand for this.

## 🛠 Build

### 🛠 Compile rust to wasm with `wasm-pack build`

```bash
wasm-pack build -t nodejs
```

### 📦 Build npm package

```bash
wasm-pack pack
```