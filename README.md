# gitbak - backup git repositories

**Warning: it's a toy project!**

Backs up git repositories from GitHub. Support for other providers may or may not be added in the future.

Implemented using API calls to get repository listing and plain `clone` for download.

## Run with `cargo`

```shell
cargo run --bin ghbak <username> <destination> -- [-x exclude1,exclude2] [-i only-this,and-this]
```
For example, the following run will clone `github.com/zelr0x/ghbak` repository and store it in `./dest/ghbak` 
```shell
cargo run --bin ghbak zelr0x ./dest -- -i ghbak
```
