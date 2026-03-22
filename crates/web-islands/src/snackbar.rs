use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use web_sys::{Document, Element, Event, HtmlDocument, Window, console};

const COOKIE_NAME: &str = "flash_aargh";
const ROOT_ID: &str = "snackbar";
const MESSAGE_ID: &str = "snackbar-message";
const DISMISS_SELECTOR: &str = "button.action";
const HIDDEN_CLASSES: [&str; 2] = ["translate-y-full", "opacity-0"];
const VISIBLE_CLASSES: [&str; 2] = ["translate-y-0", "opacity-100"];
const AUTO_HIDE_MS: i32 = 4_000;

pub fn hydrate_snackbar(doc: &Document) -> Result<(), JsValue> {
    console::log_1(&"[snackbar] hydrate start".into());

    let Some(root) = doc.get_element_by_id(ROOT_ID) else {
        console::warn_1(&"[snackbar] root element not found".into());
        return Ok(());
    };

    console::log_1(&"[snackbar] root element found".into());
    bind_dismiss_handler(&root)?;

    let Some(encoded_message) = get_cookie(doc, COOKIE_NAME) else {
        console::log_1(&"[snackbar] flash cookie not found".into());
        return Ok(());
    };

    console::log_1(&format!("[snackbar] flash cookie found: {encoded_message}").into());
    let message = decode_message(&encoded_message)?;
    console::log_1(&format!("[snackbar] decoded message: {message}").into());
    if let Some(message_element) = doc.get_element_by_id(MESSAGE_ID) {
        message_element.set_text_content(Some(&message));
        console::log_1(&"[snackbar] message element populated".into());
    } else {
        console::warn_1(&"[snackbar] message element not found".into());
    }

    show(&root)?;
    console::log_1(&"[snackbar] snackbar shown".into());
    delete_cookie(doc, COOKIE_NAME)?;
    console::log_1(&"[snackbar] flash cookie deleted".into());
    schedule_hide(root)?;
    console::log_1(&"[snackbar] auto-hide timer scheduled".into());

    Ok(())
}

fn bind_dismiss_handler(root: &Element) -> Result<(), JsValue> {
    if root.has_attribute("data-octo-snackbar-bound") {
        console::log_1(&"[snackbar] dismiss handler already bound".into());
        return Ok(());
    }

    root.set_attribute("data-octo-snackbar-bound", "true")?;

    let Some(dismiss) = root.query_selector(DISMISS_SELECTOR)? else {
        console::warn_1(&"[snackbar] dismiss button not found".into());
        return Ok(());
    };

    console::log_1(&"[snackbar] binding dismiss handler".into());

    let root_for_handler = root.clone();
    let click_handler = Closure::<dyn FnMut(_)>::new(move |event: Event| {
        event.prevent_default();
        console::log_1(&"[snackbar] dismiss clicked".into());
        if let Err(error) = hide(&root_for_handler) {
            console::error_1(&error);
        }
    });

    dismiss.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())?;
    click_handler.forget();

    Ok(())
}

fn schedule_hide(root: Element) -> Result<(), JsValue> {
    let Some(window) = web_sys::window() else {
        console::warn_1(&"[snackbar] window missing; skipping auto-hide".into());
        return Ok(());
    };

    let hide_callback = Closure::<dyn FnMut()>::new(move || {
        console::log_1(&"[snackbar] auto-hide fired".into());
        if let Err(error) = hide(&root) {
            console::error_1(&error);
        }
    });

    set_timeout(
        &window,
        hide_callback.as_ref().unchecked_ref(),
        AUTO_HIDE_MS,
    )?;
    hide_callback.forget();

    Ok(())
}

fn set_timeout(window: &Window, callback: &Function, timeout_ms: i32) -> Result<(), JsValue> {
    window.set_timeout_with_callback_and_timeout_and_arguments_0(callback, timeout_ms)?;
    Ok(())
}

fn show(root: &Element) -> Result<(), JsValue> {
    update_visibility(root, true)
}

fn hide(root: &Element) -> Result<(), JsValue> {
    update_visibility(root, false)
}

fn update_visibility(root: &Element, visible: bool) -> Result<(), JsValue> {
    let class_list = root.class_list();

    for class_name in HIDDEN_CLASSES {
        if visible {
            class_list.remove_1(class_name)?;
        } else {
            class_list.add_1(class_name)?;
        }
    }

    for class_name in VISIBLE_CLASSES {
        if visible {
            class_list.add_1(class_name)?;
        } else {
            class_list.remove_1(class_name)?;
        }
    }

    Ok(())
}

fn get_cookie(doc: &Document, name: &str) -> Option<String> {
    let cookie_string = html_document(doc).ok()?.cookie().ok()?;
    console::log_1(&format!("[snackbar] document.cookie={cookie_string}").into());
    let prefix = format!("{name}=");

    cookie_string
        .split(';')
        .map(str::trim)
        .find_map(|cookie: &str| cookie.strip_prefix(&prefix).map(ToString::to_string))
}

fn delete_cookie(doc: &Document, name: &str) -> Result<(), JsValue> {
    html_document(doc)?.set_cookie(&format!("{name}=; Max-Age=0; path=/; SameSite=Lax"))
}

fn html_document(doc: &Document) -> Result<HtmlDocument, JsValue> {
    doc.clone()
        .dyn_into::<HtmlDocument>()
        .map_err(|_| JsValue::from_str("Document was not an HtmlDocument"))
}

fn decode_message(encoded_message: &str) -> Result<String, JsValue> {
    let bytes = URL_SAFE_NO_PAD
        .decode(encoded_message)
        .map_err(|err| JsValue::from_str(&format!("Failed to decode flash cookie: {err}")))?;
    String::from_utf8(bytes)
        .map_err(|err| JsValue::from_str(&format!("Flash cookie was not valid UTF-8: {err}")))
}
