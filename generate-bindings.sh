#!/usr/bin/env bash

BINDGEN=../rust-bindgen/target/debug/bindgen
LIBNFC=/usr/include/nfc

$BINDGEN -l$LIBNFC -match nfc -allow-unknown-types -o src/nfc-emulation.rs $LIBNFC/nfc-emulation.h
$BINDGEN -l$LIBNFC -match nfc -allow-unknown-types -o src/nfc-types.rs $LIBNFC/nfc-types.h
$BINDGEN -l$LIBNFC -match nfc -allow-unknown-types -o src/nfc.rs $LIBNFC/nfc.h
