# fshare

A simple and stupid tool to share a file by TCP.

## Build

   ```sh
   $ cargo build --release
   ```

## Example

Transfer a file from QEMU guest to host under the default user-mode networking:
1. On the host:

   ```sh
   $ fshare receive /path/to/output/file
   ```

2. On the guest:

    ```sh
    $ fshare send /path/to/input/file 10.0.2.2:8000
    ```
