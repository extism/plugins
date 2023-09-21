# Plugins

Some up to date Extism plugins for use in demos and tests.

## Releases

Check the [releases](https://github.com/extism/plugins/releases) for stable builds.

You can download the latest plugin by name like this:

```
curl https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm
```

or a specific version like this:

```
curl https://github.com/extism/plugins/releases/download/v0.0.3/count_vowels.wasm
```

> *Note*: Each plugin should also have a debug build at {name}.debug.wasm

## Development

Run make on the top level folder, it will build all the plugins in the cargo workspace.

```bash
make build
```

If you add a plug-in, make sure it is added to `Cargo.toml` in the `workspace.members` section.

Cut a new release and GH actions will build the plugins and publish as assets on the release.
