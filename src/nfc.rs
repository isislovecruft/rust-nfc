
extern crate libc;

use libc::{c_int, size_t};
use slice::raw;

use nfc_types;


#[link(name = "libnfc")]
extern {
    /* Library initialisation/deinitialisation */
    fn nfc_init(context: *mut NFCContext); // XXX takes **context in original
    fn nfc_exit(context: *mut NFCContext);
    fn nfc_register_driver(driver: *const NFCDriver) -> c_int;

    /* NFC Device/Hardware manipulation */
    fn nfc_open(context: &mut NFCContext, connstring: *const char) -> &NFCDevice;
    fn nfc_close(pnd: &mut NFCDevice);
    fn nfc_abort_command(pnd: &mut NFCDevice);
    // XXX takes multiple connstrings in the original
    fn nfc_list_devices(context: &mut NFCContext, connstring: *const char, connstring_len: size_t) -> size_t;
    fn nfc_idle(pnd: &mut NFCDevice) -> c_int;

    /* NFC initiator: act as "reader" */
    fn nfc_initiator_init(pnd: *mut NFCDevice) -> c_int;
    fn nfc_initiator_init_secure_element(pnd: *mut NFCDevice) -> c_int;
    fn nfc_initiator_select_passive_target(pnd: *mut NFCDevice, nm: NFCModulation) -> c_int;
}
