This is a test workspace for the artifact dependencies RFC. It represents the
real world needs of the Enarx project. The core crate is `enarx-run`. Its
binary should be the only binary installed during `cargo install`.

The `enarx-run` crate depends on the following binaries produced:

  * enarx-shim-sgx: x86_64-unknown-none
  * enarx-shim-kvm: x86_64-unknown-none
  * enarx-shim-kvm: aarch64-unknown-none
  * enarx-exec-wasmldr: x86_64-unknown-linux-musl
  * enarx-exec-wasmldr: aarch64-unknown-linux-musl

Each of these binaries will be bundled in the `enarx-run` binary using
`include_bytes!()`. Further, the `enarx-exec-wasmldr` crate uses
`enarx-test-wasm` compiled for `wasm32-wasi` in its integration tests.

An ASCII-art graph showing the dependency relations is below:

```
                   x86_64..none
  +-----------------------------------+
  v                                   |
+----------------+  aarch64..none   +-------------------------+  x86_64..none   +----------------+
| enarx-shim-kvm | <--------------- |       enarx-run         | --------------> | enarx-shim-sgx |
+----------------+                  +-------------------------+                 +----------------+
                                      |                     |
                                      | x86_64..musl        | aarch64..musl
                                      v                     |
                                    +--------------------+  |
                                    | enarx-exec-wasmldr | <+
                                    +--------------------+
                                      |
                                      | wasm32-wasi
                                      v
                                    +--------------------+
                                    |  enarx-test-wasm   |
                                    +--------------------+
```