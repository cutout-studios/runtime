# `@cutout/runtime`

> [!WARNING]
> This is an experimental no-frills replacement for the now maintenance-mode Deno.
> If the community rallies around a stable fork, this project may no longer be necessary.

```sh
cargo build --bin cutout --release

./target/release/cutout ./test.ts
# => 3
```

## TODOs

- [ ] error handling w/ traces
- [ ] snapshot v8 heap to speed up initialization
- [ ] default to repl when no file is passed
- [ ] define `RuntimePlugin` and implement:
  - [ ] collision guards
  - [ ] `ConsolePlugin`
  - [ ] `FetchPlugin`
  - [ ] `ServePlugin`
  - [ ] `--plugins=console,fetch,serve`
- [ ] implement remote imports
- [ ] `runtime.jsonc` support
  - [ ] ability to configure jsx pragma
- [ ] `runtime.jsonc` VSCode plugin
