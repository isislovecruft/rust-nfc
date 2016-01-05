=============================================
 rust-nfc — Rust FFI bindings to libnfc_
=============================================

Installation
""""""""""""

```
cargo build
```

Dependencies
""""""""""""

 * `libnfc` (on Debian-based systems, use the `libnfc5` package)
 * `libpcsclite` (required by libnfc for supporting the acr122_pcsc
   driver, which is needed to support smartcard devices)
 * `libusb` (required by libnfc for some NFC devices)

.. _libnfc: https://github.com/nfc-tools/libnfc

