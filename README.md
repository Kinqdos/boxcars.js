<div align="center">

  <h1><code>boxcars-wasm</code></h1>

  <strong>A JavaScript port of <a href="https://github.com/nickbabcock/boxcars">boxcars</a> using WebAssembly</strong>
</div>

## About
boxcars-wasm uses [WebAssembly](https://github.com/rustwasm/wasm-pack) to use the [Rocket League](http://www.rocketleaguegame.com/) replay parser library [boxcars](https://github.com/nickbabcock/boxcars) from [nickbabcok](https://github.com/nickbabcock)

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## Build

### 🛠️ Compile rust to wasm with `wasm-pack build`

```
wasm-pack build
```

### 📦 Package npm package

```
wasm-pack pack
```