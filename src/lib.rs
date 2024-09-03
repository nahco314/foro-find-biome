mod finding;

extern crate alloc;

use crate::finding::find_biome;
use core::alloc::Layout;
use serde_json::{json, Value};
use std::path::PathBuf;

fn to_array_result(arr: &[u8]) -> *mut u8 {
    let mut new_vec = (arr.len() as u64).to_le_bytes().to_vec();

    new_vec.append(&mut arr.to_vec());

    let ptr = new_vec.as_mut_ptr();
    std::mem::forget(new_vec);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn of_malloc(size: u32, alignment: u32) -> *mut u8 {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn of_free(ptr: *mut u8, size: u32, alignment: u32) {
    let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
    alloc::alloc::dealloc(ptr, layout);
}

pub fn main_with_json(input: Value) -> Value {
    let current_dir = input
        .as_object()
        .unwrap()
        .get("current-dir")
        .unwrap()
        .as_str()
        .unwrap();

    let result = match find_biome(&PathBuf::from(current_dir)) {
        Some(v) => {
            json!({
                "found": true,
                "biome": v.to_str().unwrap()
            })
        }
        None => {
            json!({
                "found": false
            })
        }
    };

    result
}

#[no_mangle]
pub extern "C" fn main(ptr: *mut u8, len: usize) -> i32 {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let v: Value = serde_json::from_slice(slice).unwrap();

    let result = main_with_json(v);

    let b = serde_json::to_vec(&result).unwrap();
    let result = b.as_slice();

    let result = to_array_result(result);
    result as i32
}
