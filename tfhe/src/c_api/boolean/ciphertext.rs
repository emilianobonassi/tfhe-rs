use crate::c_api::buffer::*;
use crate::c_api::utils::*;
use std::os::raw::c_int;

use crate::boolean;

pub struct BooleanCiphertext(pub(in crate::c_api) boolean::ciphertext::Ciphertext);

#[no_mangle]
pub unsafe extern "C" fn boolean_serialize_ciphertext(
    ciphertext: *const BooleanCiphertext,
    result: *mut Buffer,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result).unwrap();

        let ciphertext = get_ref_checked(ciphertext).unwrap();

        let buffer: Buffer = bincode::serialize(&ciphertext.0).unwrap().into();

        *result = buffer;
    })
}

#[no_mangle]
pub unsafe extern "C" fn boolean_deserialize_ciphertext(
    buffer_view: BufferView,
    result: *mut *mut BooleanCiphertext,
) -> c_int {
    catch_panic(|| {
        check_ptr_is_non_null_and_aligned(result).unwrap();

        // First fill the result with a null ptr so that if we fail and the return code is not
        // checked, then any access to the result pointer will segfault (mimics malloc on failure)
        *result = std::ptr::null_mut();

        let ciphertext: boolean::ciphertext::Ciphertext =
            bincode::deserialize(buffer_view.into()).unwrap();

        let heap_allocated_ciphertext = Box::new(BooleanCiphertext(ciphertext));

        *result = Box::into_raw(heap_allocated_ciphertext);
    })
}
