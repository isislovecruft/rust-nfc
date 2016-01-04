#!/usr/bin/env bash

BINDGEN=../rust-bindgen/target/debug/bindgen
LIBNFC=/usr/include/nfc

$BINDGEN -l$LIBNFC -match nfc -allow-unknown-types -o src/nfc.rs $LIBNFC/nfc.h \
         -I/usr/include/nfc/nfc-emulation.h \
         -I/usr/include/nfc/nfc-types.h
