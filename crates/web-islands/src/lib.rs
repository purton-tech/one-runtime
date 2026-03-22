use wasm_bindgen::prelude::*;
use web_sys::{Document, window};

mod modal;
mod snackbar;

// A simple helper function to get the document from the global window.
fn document() -> Document {
    window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document")
}

#[wasm_bindgen]
pub fn hydrate() -> Result<(), JsValue> {
    let doc = document();
    modal::hydrate_modal_triggers(&doc)?;
    snackbar::hydrate_snackbar(&doc)?;

    Ok(())
}
