# fshare

A simple and stupid tool to share a file by TCP.

## Build

```sh
$ cargo build --release
```

## Example

Transfer a file from QEMU guest to host under the default user-mode networking:

```sh
 (host) $ fshare recv /path/to/output/file
(guest) $ fshare post 10.0.2.2:8000 /path/to/input/file
```

Transfer a file from host to QEMU guest under the default user-mode networking:

```sh
 (host) $ fshare send /path/to/input/file
(guest) $ fshare get 10.0.2.2:8000 /path/to/output/file
```
