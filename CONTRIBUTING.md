## installing wasm-bindgen: 
- first add wasm to project: `cargo add wasm-bindgen`
- install wasm cli: `cargo install wasm-bindgen-cli`
- add wasm to target: `rustup target add wasm32-unknown-unknown`


## generating wasm-bindgen: 
- `cargo build --release --target=wasm32-unknown-unknown`
- `wasm-bindgen target/wasm32-unknown-unknown/release/login.wasm --out-dir pkg`

### by default svelte does not support wasm:
- install this: `npm i -D vite-plugin-wasm`
- install this: `npm i -D vite-plugin-top-level-await`
- add those plugins here: `stwo-browser/vite.config.ts`:

```
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()]
});
```

#### it should look like that 
```
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
	plugins: [wasm(),topLevelAwait(),sveltekit()]
});
```