use std::slice;

use crate::index::linear::LinearIndex;
use crate::index::{Index, SigStore};
use crate::signature::Signature;

use crate::ffi::signature::SourmashSignature;
use crate::ffi::utils::ForeignObject;

pub struct SourmashLinearIndex;

impl ForeignObject for SourmashLinearIndex {
    type RustObject = LinearIndex<Signature>;
}

#[no_mangle]
pub unsafe extern "C" fn linearindex_new() -> *mut SourmashLinearIndex {
    SourmashLinearIndex::from_rust(LinearIndex::builder().build())
}

ffi_fn! {
unsafe fn linearindex_new_with_sigs(
    search_sigs_ptr: *const *const SourmashSignature,
    insigs: usize,
) -> Result<*mut SourmashLinearIndex> {
    let search_sigs: Vec<SigStore<Signature>> = {
        assert!(!search_sigs_ptr.is_null());
        slice::from_raw_parts(search_sigs_ptr, insigs)
            .iter()
            .map(|sig| SourmashSignature::as_rust(*sig))
            .cloned()
            .map(|sig| {
                SigStore::builder()
                    .data(sig)
                    .filename("")
                    .name("")
                    .metadata("")
                    .storage(None)
                    .build()
            })
            .collect()
    };

    let linear_index = LinearIndex::builder().datasets(search_sigs).build();

    Ok(SourmashLinearIndex::from_rust(linear_index))
}
}

#[no_mangle]
pub unsafe extern "C" fn linearindex_free(ptr: *mut SourmashLinearIndex) {
    SourmashLinearIndex::drop(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn linearindex_len(ptr: *const SourmashLinearIndex) -> usize {
    let index = SourmashLinearIndex::as_rust(ptr);
    index.len()
}

ffi_fn! {
unsafe fn linearindex_signatures(ptr: *const SourmashLinearIndex,
                                  size: *mut usize) -> Result<*mut *mut SourmashSignature> {
    let index = SourmashLinearIndex::as_rust(ptr);

    let sigs = index.signatures();

    // FIXME: use the ForeignObject trait, maybe define new method there...
    let ptr_sigs: Vec<*mut SourmashSignature> = sigs.into_iter().map(|x| {
      Box::into_raw(Box::new(x)) as *mut SourmashSignature
    }).collect();

    let b = ptr_sigs.into_boxed_slice();
    *size = b.len();

    Ok(Box::into_raw(b) as *mut *mut SourmashSignature)
}
}
