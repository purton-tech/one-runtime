use js_sys::{Function, Promise, Reflect};
use serde::Deserialize;
use serde_json::{Value, json};
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{
    Document, Element, Event, Headers, HtmlInputElement, Request, RequestInit, Response, console,
    window,
};

const ROOT_ID: &str = "public-integration-tester";
const FORM_ID: &str = "public-integration-form";
const API_KEY_ID: &str = "public-api-key";
const END_USER_ID_ID: &str = "public-end-user-id";
const FEEDBACK_ID: &str = "public-feedback";
const GRID_ID: &str = "public-integrations-grid";
const EMPTY_ID: &str = "public-empty-state";
const LOADING_ID: &str = "public-loading";
const SUMMARY_ID: &str = "public-results-summary";

fn redact_token(token: &str) -> String {
    let trimmed = token.trim();
    if trimmed.len() <= 12 {
        trimmed.to_string()
    } else {
        format!("{}...{}", &trimmed[..8], &trimmed[trimmed.len() - 4..])
    }
}

#[derive(Debug, Deserialize)]
struct IntegrationListResponse {
    integrations: Vec<IntegrationCard>,
}

#[derive(Debug, Deserialize)]
struct IntegrationCard {
    slug: String,
    name: String,
    description: String,
    logo_url: Option<String>,
    category: Option<String>,
    status: String,
}

#[derive(Debug, Deserialize)]
struct CreateSessionResponse {
    session_token: String,
    connect_url: String,
}

pub fn hydrate_public_connect_tester(doc: &Document) -> Result<(), JsValue> {
    let Some(root) = doc.get_element_by_id(ROOT_ID) else {
        return Ok(());
    };

    if root.has_attribute("data-public-connect-hydrated") {
        return Ok(());
    }
    root.set_attribute("data-public-connect-hydrated", "true")?;

    bind_form(doc, &root)?;
    bind_card_actions(doc, &root)?;

    Ok(())
}

fn bind_form(doc: &Document, root: &Element) -> Result<(), JsValue> {
    let Some(form) = doc.get_element_by_id(FORM_ID) else {
        return Ok(());
    };

    let doc_for_handler = doc.clone();
    let root_for_handler = root.clone();
    let submit_handler = Closure::<dyn FnMut(_)>::new(move |event: Event| {
        event.prevent_default();
        let doc = doc_for_handler.clone();
        let root = root_for_handler.clone();
        spawn_local(async move {
            if let Err(error) = load_integrations(doc, root).await {
                console::error_1(&JsValue::from_str(&error));
            }
        });
    });

    form.add_event_listener_with_callback("submit", submit_handler.as_ref().unchecked_ref())?;
    submit_handler.forget();

    Ok(())
}

fn bind_card_actions(doc: &Document, root: &Element) -> Result<(), JsValue> {
    let Some(grid) = doc.get_element_by_id(GRID_ID) else {
        return Ok(());
    };

    let doc_for_handler = doc.clone();
    let root_for_handler = root.clone();
    let click_handler = Closure::<dyn FnMut(_)>::new(move |event: Event| {
        let Some(target) = event.target() else {
            return;
        };

        let Ok(element) = target.dyn_into::<Element>() else {
            return;
        };

        let Ok(Some(button)) = element.closest("[data-public-connect-slug]") else {
            return;
        };

        event.prevent_default();

        let slug = button
            .get_attribute("data-public-connect-slug")
            .unwrap_or_default();
        let name = button
            .get_attribute("data-public-connect-name")
            .unwrap_or_default();

        if slug.trim().is_empty() || name.trim().is_empty() {
            return;
        }

        let doc = doc_for_handler.clone();
        let root = root_for_handler.clone();
        spawn_local(async move {
            if let Err(message) = connect_integration(doc.clone(), root.clone(), &slug, &name).await
            {
                let _ = show_feedback(
                    &doc,
                    &format!("Failed to start connection for {}: {}", name, message),
                    true,
                );
            }
        });
    });

    grid.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())?;
    click_handler.forget();

    Ok(())
}

async fn load_integrations(doc: Document, root: Element) -> Result<(), String> {
    let api_key = required_input_value(&doc, API_KEY_ID, "API key is required")?;
    let end_user_id = required_input_value(&doc, END_USER_ID_ID, "End user id is required")?;

    set_loading_state(&doc, true)?;
    hide_feedback(&doc)?;

    let list_url = format!(
        "{}?end_user_id={}",
        required_dataset(&root, "listUrl")?,
        encode_uri_component(&end_user_id)?
    );

    let value = fetch_json("GET", &list_url, &api_key, None).await;
    set_loading_state(&doc, false)?;

    match value {
        Ok(value) => {
            let response: IntegrationListResponse =
                serde_json::from_value(value).map_err(|err| err.to_string())?;
            render_integrations(&doc, &response.integrations)?;
            update_summary(
                &doc,
                &format!(
                    "Loaded {} integrations for end user {}.",
                    response.integrations.len(),
                    end_user_id
                ),
            )?;
            if response.integrations.is_empty() {
                show_feedback(
                    &doc,
                    "No connectable integrations are currently available.",
                    false,
                )?;
            }
            Ok(())
        }
        Err(message) => {
            render_integrations(&doc, &[])?;
            update_summary(&doc, "Unable to load integrations.")?;
            show_feedback(&doc, &message, true)?;
            Err(message)
        }
    }
}

async fn connect_integration(
    doc: Document,
    root: Element,
    slug: &str,
    name: &str,
) -> Result<(), String> {
    let api_key = required_input_value(&doc, API_KEY_ID, "API key is required")?;
    let end_user_id = required_input_value(&doc, END_USER_ID_ID, "End user id is required")?;

    show_feedback(&doc, &format!("Starting {} connection...", name), false)?;

    let payload = json!({
        "integration_slug": slug,
        "end_user_id": end_user_id,
        "suggested_connection_name": format!("{name} ({})", slug.replace('-', " ")),
    });

    let create_session_url = required_dataset(&root, "createSessionUrl")?;
    let value = fetch_json("POST", &create_session_url, &api_key, Some(payload)).await?;
    let session: CreateSessionResponse =
        serde_json::from_value(value).map_err(|err| err.to_string())?;

    console::log_1(
        &format!(
            "[public-connect] created hosted session slug={} token={} connect_url={}",
            slug,
            redact_token(&session.session_token),
            session.connect_url
        )
        .into(),
    );

    let popup_result = open_connection_popup(&session.session_token, &session.connect_url).await?;
    let status = popup_result
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown");

    match status {
        "connected" => {
            show_feedback(&doc, &format!("{} connected.", name), false)?;
            load_integrations(doc, root).await?;
        }
        "cancelled" => {
            show_feedback(&doc, &format!("{} popup cancelled.", name), false)?;
        }
        other => {
            let message = format!("Popup returned unexpected status: {other}");
            show_feedback(&doc, &message, true)?;
            return Err(message);
        }
    }

    Ok(())
}

async fn open_connection_popup(session_token: &str, connect_url: &str) -> Result<Value, String> {
    console::log_1(
        &format!(
            "[public-connect] opening hosted modal token={} connect_url={}",
            redact_token(session_token),
            connect_url
        )
        .into(),
    );
    let window = window().ok_or_else(|| "window is not available".to_string())?;
    let one_runtime = Reflect::get(window.as_ref(), &JsValue::from_str("OneRuntime"))
        .map_err(js_error)?
        .dyn_into::<js_sys::Object>()
        .map_err(|_| "window.OneRuntime is not available".to_string())?;

    let create_runtime = Reflect::get(&one_runtime, &JsValue::from_str("createOneRuntime"))
        .map_err(js_error)?
        .dyn_into::<Function>()
        .map_err(|_| "createOneRuntime is not a function".to_string())?;

    let config = js_sys::Object::new();
    let origin = window.location().origin().map_err(js_error)?;
    Reflect::set(
        &config,
        &JsValue::from_str("baseUrl"),
        &JsValue::from_str(&origin),
    )
    .map_err(js_error)?;

    let runtime = create_runtime
        .call1(&one_runtime, &config)
        .map_err(js_error)?;
    let connections =
        Reflect::get(&runtime, &JsValue::from_str("connections")).map_err(js_error)?;
    let open = Reflect::get(&connections, &JsValue::from_str("open"))
        .map_err(js_error)?
        .dyn_into::<Function>()
        .map_err(|_| "connections.open is not a function".to_string())?;

    let options = js_sys::Object::new();
    Reflect::set(
        &options,
        &JsValue::from_str("sessionToken"),
        &JsValue::from_str(session_token),
    )
    .map_err(js_error)?;
    Reflect::set(
        &options,
        &JsValue::from_str("connectUrl"),
        &JsValue::from_str(connect_url),
    )
    .map_err(js_error)?;

    let promise = open
        .call1(&connections, &options)
        .map_err(js_error)?
        .dyn_into::<Promise>()
        .map_err(|_| "connections.open did not return a promise".to_string())?;

    let result = JsFuture::from(promise).await.map_err(js_error)?;
    let text = js_sys::JSON::stringify(&result)
        .map_err(js_error)?
        .as_string()
        .ok_or_else(|| "popup response was not serializable".to_string())?;

    serde_json::from_str(&text).map_err(|err| err.to_string())
}

async fn fetch_json(
    method: &str,
    url: &str,
    api_key: &str,
    body: Option<Value>,
) -> Result<Value, String> {
    let window = window().ok_or_else(|| "window is not available".to_string())?;
    let headers = Headers::new().map_err(js_error)?;
    headers
        .set("Authorization", &format!("Bearer {api_key}"))
        .map_err(js_error)?;

    let init = RequestInit::new();
    init.set_method(method);
    init.set_headers(&headers);

    if let Some(body) = body {
        headers
            .set("Content-Type", "application/json")
            .map_err(js_error)?;
        init.set_body(&JsValue::from_str(&body.to_string()));
    }

    let request = Request::new_with_str_and_init(url, &init).map_err(js_error)?;
    let response = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(js_error)?
        .dyn_into::<Response>()
        .map_err(|_| "fetch did not return a Response".to_string())?;

    let text = response_text(&response).await?;
    if response.ok() {
        serde_json::from_str(&text).map_err(|err| err.to_string())
    } else {
        if let Ok(value) = serde_json::from_str::<Value>(&text)
            && let Some(message) = value.get("error").and_then(Value::as_str)
        {
            return Err(message.to_string());
        }
        Err(if text.trim().is_empty() {
            format!("Request failed with status {}", response.status())
        } else {
            text
        })
    }
}

async fn response_text(response: &Response) -> Result<String, String> {
    let promise = response.text().map_err(js_error)?;
    let value = JsFuture::from(promise).await.map_err(js_error)?;
    value
        .as_string()
        .ok_or_else(|| "response body was not text".to_string())
}

fn render_integrations(doc: &Document, integrations: &[IntegrationCard]) -> Result<(), String> {
    let grid = required_element(doc, GRID_ID)?;
    let empty = required_element(doc, EMPTY_ID)?;

    if integrations.is_empty() {
        grid.set_inner_html("");
        set_class_hidden(&grid, true)?;
        set_class_hidden(&empty, false)?;
        return Ok(());
    }

    let html = integrations
        .iter()
        .map(render_card_html)
        .collect::<Vec<_>>()
        .join("");
    grid.set_inner_html(&html);
    set_class_hidden(&grid, false)?;
    set_class_hidden(&empty, true)?;
    Ok(())
}

fn render_card_html(card: &IntegrationCard) -> String {
    let is_connected = card.status == "connected";
    let status_class = if is_connected {
        "badge badge-success badge-outline"
    } else {
        "badge badge-ghost"
    };
    let status_label = if is_connected {
        "Connected"
    } else {
        "Not connected"
    };
    let logo = match &card.logo_url {
        Some(url) => format!(
            "<img class=\"h-10 w-10 rounded-lg border border-base-300 bg-base-200 p-1 object-contain\" src=\"{}\" alt=\"{} logo\" />",
            escape_html_attr(url),
            escape_html_attr(&card.name)
        ),
        None => format!(
            "<div class=\"flex h-10 w-10 items-center justify-center rounded-lg border border-base-300 bg-base-200 text-xs font-semibold text-base-content/70\">{}</div>",
            escape_html_text(&initials(&card.name))
        ),
    };
    let category = card
        .category
        .as_ref()
        .map(|value| {
            format!(
                "<span class=\"badge badge-outline badge-sm\">{}</span>",
                escape_html_text(value)
            )
        })
        .unwrap_or_default();
    let action_html = if is_connected {
        "<span class=\"btn btn-success btn-sm btn-disabled\">Connected</span>".to_string()
    } else {
        format!(
            "<button class=\"btn btn-primary btn-sm\" type=\"button\" data-public-connect-slug=\"{}\" data-public-connect-name=\"{}\">Connect</button>",
            escape_html_attr(&card.slug),
            escape_html_attr(&card.name),
        )
    };

    format!(
        "<article class=\"card border border-base-300 bg-base-100 shadow-sm\"><div class=\"card-body gap-4\"><div class=\"flex items-start justify-between gap-3\"><div class=\"flex items-start gap-3 min-w-0\">{}<div class=\"min-w-0\"><h3 class=\"truncate text-base font-semibold\">{}</h3><p class=\"mt-1 line-clamp-3 text-sm text-base-content/70\">{}</p></div></div><span class=\"{} shrink-0\">{}</span></div><div class=\"flex items-center justify-between gap-2\">{}{}</div></div></article>",
        logo,
        escape_html_text(&card.name),
        escape_html_text(&card.description),
        status_class,
        status_label,
        category,
        action_html,
    )
}

fn initials(name: &str) -> String {
    let value = name
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase();
    if value.is_empty() {
        "API".to_string()
    } else {
        value
    }
}

fn set_loading_state(doc: &Document, loading: bool) -> Result<(), String> {
    let loading_element = required_element(doc, LOADING_ID)?;
    let empty_element = required_element(doc, EMPTY_ID)?;
    if loading {
        set_class_hidden(&loading_element, false)?;
        set_class_hidden(&empty_element, true)?;
    } else {
        set_class_hidden(&loading_element, true)?;
    }
    Ok(())
}

fn update_summary(doc: &Document, message: &str) -> Result<(), String> {
    let element = required_element(doc, SUMMARY_ID)?;
    element.set_text_content(Some(message));
    Ok(())
}

fn show_feedback(doc: &Document, message: &str, is_error: bool) -> Result<(), String> {
    let feedback = required_element(doc, FEEDBACK_ID)?;
    feedback.set_text_content(Some(message));
    feedback.set_class_name(if is_error {
        "alert alert-error"
    } else {
        "alert alert-info"
    });
    Ok(())
}

fn hide_feedback(doc: &Document) -> Result<(), String> {
    let feedback = required_element(doc, FEEDBACK_ID)?;
    feedback.set_text_content(None);
    feedback.set_class_name("hidden alert");
    Ok(())
}

fn required_dataset(root: &Element, key: &str) -> Result<String, String> {
    let attribute_name = match key {
        "listUrl" => "data-list-url",
        "createSessionUrl" => "data-create-session-url",
        other => return Err(format!("Unsupported dataset key: {other}")),
    };

    root.get_attribute(attribute_name)
        .filter(|value: &String| !value.trim().is_empty())
        .ok_or_else(|| format!("Missing {attribute_name} attribute"))
}

fn required_input_value(doc: &Document, id: &str, error: &str) -> Result<String, String> {
    doc.get_element_by_id(id)
        .ok_or_else(|| error.to_string())?
        .dyn_into::<HtmlInputElement>()
        .map_err(|_| error.to_string())?
        .value()
        .trim()
        .to_string()
        .pipe(|value| {
            if value.is_empty() {
                Err(error.to_string())
            } else {
                Ok(value)
            }
        })
}

fn required_element(doc: &Document, id: &str) -> Result<Element, String> {
    doc.get_element_by_id(id)
        .ok_or_else(|| format!("Element #{id} not found"))
}

fn set_class_hidden(element: &Element, hidden: bool) -> Result<(), String> {
    let classes = element.class_list();
    if hidden {
        classes.add_1("hidden").map_err(js_error)?;
    } else {
        classes.remove_1("hidden").map_err(js_error)?;
    }
    Ok(())
}

fn escape_html_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_html_attr(value: &str) -> String {
    escape_html_text(value).replace('"', "&quot;")
}

fn encode_uri_component(value: &str) -> Result<String, String> {
    js_sys::encode_uri_component(value)
        .as_string()
        .ok_or_else(|| "Failed to encode URL parameter".to_string())
}

fn js_error(error: JsValue) -> String {
    error.as_string().unwrap_or_else(|| format!("{error:?}"))
}

trait Pipe: Sized {
    fn pipe<T>(self, f: impl FnOnce(Self) -> T) -> T {
        f(self)
    }
}

impl<T> Pipe for T {}
