#![allow(non_snake_case)]

use crate::render;
use dioxus::prelude::*;
use web_assets::files::*;

#[derive(Debug, Clone, PartialEq)]
pub struct HostedConnectionPageModel {
    pub integration_name: String,
    pub session_token: String,
    pub request_id: String,
    pub suggested_connection_name: String,
    pub end_user_id: String,
    pub end_user_name: String,
    pub end_user_email: String,
    pub error_message: Option<String>,
}

pub fn page(model: HostedConnectionPageModel) -> String {
    let HostedConnectionPageModel {
        integration_name,
        session_token,
        request_id,
        suggested_connection_name,
        end_user_id,
        end_user_name,
        end_user_email,
        error_message,
    } = model;
    let has_error = error_message.is_some();

    let submit_script = r#"
const RESULT_SOURCE = "one-runtime";
const SESSION_TOKEN = __SESSION_TOKEN__;
const REQUEST_ID = __REQUEST_ID__;

function showError(message) {
  const root = document.getElementById("connect-error");
  const messageNode = document.getElementById("connect-error-message");
  if (!root || !messageNode) return;
  messageNode.textContent = message;
  root.classList.remove("hidden");
}

function hideError() {
  const root = document.getElementById("connect-error");
  if (root) root.classList.add("hidden");
}

function relay(payload) {
  if (window.parent && window.parent !== window) {
    window.parent.postMessage(payload, "*");
  }
  if (window.opener) {
    window.opener.postMessage(payload, "*");
  }
}

function cancelFlow() {
  console.log("[hosted-connect] cancel", { requestId: REQUEST_ID, sessionToken: String(SESSION_TOKEN).slice(0, 8) + "..." });
  relay({ source: RESULT_SOURCE, requestId: REQUEST_ID, status: "cancelled" });
}

document.addEventListener("DOMContentLoaded", () => {
  const form = document.getElementById("connect-form");
  const cancel = document.getElementById("connect-cancel");
  const connectionNameInput = document.getElementById("connection-name");
  const apiKeyInput = document.getElementById("provider-api-key");
  const submitButton = document.getElementById("connect-submit");
  if (!form || !submitButton || !connectionNameInput || !apiKeyInput) return;

  cancel?.addEventListener("click", (event) => {
    event.preventDefault();
    cancelFlow();
  });

  form.addEventListener("submit", async (event) => {
    event.preventDefault();
    hideError();
    submitButton.setAttribute("disabled", "disabled");

    try {
      console.log("[hosted-connect] submit", {
        requestId: REQUEST_ID,
        sessionToken: String(SESSION_TOKEN).slice(0, 8) + "...",
        connectionName: connectionNameInput.value,
      });
      const response = await fetch("/connect/submit.json", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          session_token: SESSION_TOKEN,
          request_id: REQUEST_ID,
          connection_name: connectionNameInput.value,
          api_key: apiKeyInput.value,
        }),
      });
      const rawText = await response.text();
      let payload = null;
      try {
        payload = rawText ? JSON.parse(rawText) : null;
      } catch (_error) {
        payload = null;
      }

      if (!payload) {
        showError(rawText || "Unable to connect this account.");
        return;
      }
      if (!response.ok || payload.status === "error") {
        showError(payload.error || "Unable to connect this account.");
        return;
      }
      relay(payload);
    } catch (error) {
      showError(error instanceof Error ? error.message : "Unable to connect this account.");
    } finally {
      submitButton.removeAttribute("disabled");
    }
  });
});
"#
    .replace(
        "__SESSION_TOKEN__",
        &serde_json::to_string(&session_token).unwrap(),
    )
    .replace(
        "__REQUEST_ID__",
        &serde_json::to_string(&request_id).unwrap(),
    );

    let page = rsx! {
        head {
            title { "Connect {integration_name}" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            link { rel: "stylesheet", href: tailwind_css.name }
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/daisyui@5" }
            link { rel: "icon", "type": "image/svg+xml", href: favicon_svg.name }
            script { "type": "module", dangerous_inner_html: submit_script }
        }
        body {
            class: "min-h-screen bg-base-200 text-base-content",
            main {
                class: "min-h-screen flex items-center justify-center p-4",
                div {
                    class: "w-full max-w-md space-y-4",
                    div {
                        class: "flex items-center justify-center",
                        img {
                            src: logo_svg.name,
                            alt: "One Runtime",
                            class: "h-12 w-12 object-contain"
                        }
                    }
                    div {
                        class: "card border border-base-300 bg-base-100 shadow-sm",
                        div {
                            class: "card-body gap-4",
                            div {
                                class: "space-y-2 text-center",
                                h1 { class: "text-2xl font-semibold", "Connect {integration_name}" }
                                p {
                                    class: "text-sm text-base-content/70",
                                    "Enter the API key for the account you want to connect."
                                }
                            }
                            if !end_user_name.is_empty() || !end_user_email.is_empty() || !end_user_id.is_empty() {
                                div {
                                    class: "rounded-box border border-base-300 bg-base-200/50 p-3 text-sm",
                                    p { class: "font-medium", "End user" }
                                    if !end_user_name.is_empty() {
                                        p { "{end_user_name}" }
                                    }
                                    if !end_user_email.is_empty() {
                                        p { "{end_user_email}" }
                                    }
                                    if !end_user_id.is_empty() {
                                        p { class: "font-mono text-xs text-base-content/70", "{end_user_id}" }
                                    }
                                }
                            }
                            div {
                                id: "connect-error",
                                class: if has_error { "alert alert-error" } else { "hidden alert alert-error" },
                                span {
                                    id: "connect-error-message",
                                    if let Some(error_message) = &error_message {
                                        "{error_message}"
                                    }
                                }
                            }
                            form {
                                id: "connect-form",
                                class: "space-y-4",
                                fieldset {
                                    class: "fieldset",
                                    legend { class: "fieldset-legend", "Connection name" }
                                    input {
                                        id: "connection-name",
                                        class: "input input-bordered w-full",
                                        r#type: "text",
                                        name: "connection_name",
                                        required: true,
                                        maxlength: "120",
                                        value: "{suggested_connection_name}",
                                        placeholder: "Personal account"
                                    }
                                }
                                fieldset {
                                    class: "fieldset",
                                    legend { class: "fieldset-legend", "API key" }
                                    input {
                                        id: "provider-api-key",
                                        class: "input input-bordered w-full",
                                        r#type: "password",
                                        name: "api_key",
                                        required: true,
                                        autocomplete: "off",
                                        placeholder: "Paste API key"
                                    }
                                }
                                div {
                                    class: "flex justify-end gap-2",
                                    button {
                                        id: "connect-cancel",
                                        class: "btn btn-ghost",
                                        r#type: "button",
                                        "Cancel"
                                    }
                                    button {
                                        id: "connect-submit",
                                        class: "btn btn-primary",
                                        r#type: "submit",
                                        "Connect"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}

pub fn error_page(title: String, message: String) -> String {
    let close_script = r#"
function closeHostedConnect() {
  if (window.parent && window.parent !== window) {
    window.parent.postMessage({ source: "one-runtime", requestId: "", status: "cancelled" }, "*");
  }
  if (window.opener) {
    window.close();
  }
}

document.addEventListener("DOMContentLoaded", () => {
  document.getElementById("connect-error-close")?.addEventListener("click", closeHostedConnect);
});
"#;
    let page = rsx! {
        head {
            title { "{title}" }
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1" }
            link { rel: "stylesheet", href: tailwind_css.name }
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/daisyui@5" }
            link { rel: "icon", "type": "image/svg+xml", href: favicon_svg.name }
            script { dangerous_inner_html: close_script }
        }
        body {
            class: "min-h-screen bg-base-200 text-base-content",
            main {
                class: "min-h-screen flex items-center justify-center p-4",
                div {
                    class: "card w-full max-w-md border border-base-300 bg-base-100 shadow-sm",
                    div {
                        class: "card-body gap-4 text-center",
                        div {
                            class: "flex justify-center",
                            img {
                                src: logo_svg.name,
                                alt: "One Runtime",
                                class: "h-12 w-12 object-contain"
                            }
                        }
                        h1 { class: "text-2xl font-semibold", "{title}" }
                        div {
                            class: "alert alert-error justify-center",
                            span { "{message}" }
                        }
                        div {
                            class: "flex justify-center",
                            button {
                                id: "connect-error-close",
                                class: "btn btn-primary",
                                r#type: "button",
                                "Close"
                            }
                        }
                    }
                }
            }
        }
    };

    render(page)
}
