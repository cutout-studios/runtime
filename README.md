# `@cutout/runtime`

> [!WARNING]
> This is an experimental no-frills replacement for the now maintenance-mode Deno.
> If the community rallies around a stable fork, this project may no longer be necessary.

```sh
cargo build --bin xo --release

./target/release/xo ./test.ts
# => 3
```

## TODOs

- [ ] error handling w/ traces
- [ ] define collision-proof `RuntimeExtension` and implement:
  - [ ] xo_console
  - [ ] xo_fetch
  - [ ] xo_serve
- [ ] minimal `runtime.jsonc` support

```jsonc
{
  "compilerOptions": {
    "jsxImportSource": "jsr:@cutout/kernel"
  },
  "runtime": {
    // bonus - types the globalThis object
    "extensions": [
      "xo_console",
      "xo_fetch",
      "xo_serve
    ]
  }
}
```

- [ ] get VSCode to work with `runtime.jsonc`
- [ ] CI/CD
