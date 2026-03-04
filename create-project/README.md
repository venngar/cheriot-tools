# create-project

A barebones PoC Rust tool that creates CHERIoT project by cloning cheriot-template and submodules. It requires `path` argument which is a new folder where `cheriot-template` repository will be cloned.

Build:

```
$ cargo build
```

Run:

```
$ cd target/release
$ ./create-project --path <absolute_path>
```