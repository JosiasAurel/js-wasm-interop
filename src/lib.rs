use std::iter::FromIterator;

use js_sys;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct ObjectKind {
    name: String,
    color: String,
}

static mut OBJECT_STORE: Vec<ObjectKind> = Vec::new();

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    /*
    let sample_obj = ObjectKind {
        name: String::from("obj1"),
        color: String::from("#4287f5"),
    };

    unsafe {
        OBJECT_STORE.push(sample_obj);

        for obj in OBJECT_STORE.iter() {
            console::log_2(
                &JsValue::from_str(&obj.name),
                &JsValue::from_str(&obj.color),
            );
        }
    }
    */

    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen()]
pub fn add_obj(color: &str, name: &str) -> Result<(), JsValue> {
    let new_obj = ObjectKind {
        name: String::from(name),
        color: String::from(color),
    };

    unsafe {
        OBJECT_STORE.push(new_obj);
    }

    Ok(())
}

#[wasm_bindgen()]
pub fn get_obj() -> Result<js_sys::Array, JsValue> {
    let mut obj_strs = Vec::new();

    // too lazy to engineer an impl for ObjectKind to js_sys::Object
    unsafe {
        for obj in OBJECT_STORE.iter() {
            obj_strs.push(JsValue::from_str(&String::from(format!(
                "{}--{}",
                obj.name, obj.color
            ))))
        }
    }

    Ok(js_sys::Array::from_iter(obj_strs))
}
