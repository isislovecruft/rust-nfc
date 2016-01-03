
#![feature(phase)]
#[phase(plugin)] extern crate bindgen;

#[allow(dead_code, uppercase_variables, non_camel_case_types)]
mod nfc_bindings {
    bindgen!("/usr/include/nfc/nfc.h",
             "/usr/include/nfc/nfc-types.h",
             "/usr/include/nfc/nfc-emulation.h",
             match="nfc*.h", link="nfc")
}
