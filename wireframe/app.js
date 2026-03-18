const pages = {
  overview: {
    title: "Overview",
    controls: [
      {
        type: "button",
        label: "Browse integrations",
        className: "btn btn-outline",
        route: "providers",
      },
      {
        type: "button",
        label: "Open MCP API",
        className: "btn btn-primary",
        route: "api-keys",
      },
    ],
  },
  providers: {
    title: "Integrations",
    controls: [
      {
        type: "button",
        label: "Search functions",
        className: "btn btn-primary",
        route: "providers",
      },
      {
        type: "button",
        label: "View not connected flow",
        className: "btn btn-outline",
        route: "provider-required",
      },
    ],
  },
  "gmail-connected": {
    title: "Gmail",
    controls: [
      {
        type: "button",
        label: "View activity",
        className: "btn btn-outline",
        route: "gmail-activity",
      },
      {
        type: "button",
        label: "Get connection link",
        className: "btn btn-primary",
        route: "provider-required",
      },
    ],
  },
  "gmail-activity": {
    title: "Activity",
    controls: [
      {
        type: "select",
        className: "select select-bordered",
        options: ["All events", "Tool calls", "Connection events"],
      },
      {
        type: "button",
        label: "Export audit log",
        className: "btn btn-outline",
      },
    ],
  },
  "provider-required": {
    title: "Connection Required",
    controls: [],
  },
  "api-keys": {
    title: "API",
    controls: [
      {
        type: "button",
        label: "Create API key",
        className: "btn btn-primary",
      },
      {
        type: "button",
        label: "View system secrets",
        className: "btn btn-outline",
        route: "system-admin",
      },
    ],
  },
  "system-admin": {
    title: "System Admin",
    controls: [
      {
        type: "button",
        label: "Add provider secret",
        className: "btn btn-primary",
        route: "provider-secret-detail",
      },
    ],
  },
  "provider-secret-detail": {
    title: "Provider Secret",
    controls: [
      {
        type: "button",
        label: "Save secret",
        className: "btn btn-primary",
      },
      {
        type: "button",
        label: "Back to inventory",
        className: "btn btn-outline",
        route: "system-admin",
      },
    ],
  },
  "how-connections-work": {
    title: "How Connections Work",
    controls: [
      {
        type: "button",
        label: "Browse integrations",
        className: "btn btn-primary",
        route: "providers",
      },
    ],
  },
};

function getPageFromLocation() {
  const params = new URLSearchParams(window.location.search);
  return params.get("page") || "overview";
}

function setActiveLink(page) {
  document.querySelectorAll(".wireframe-link").forEach((link) => {
    link.classList.toggle("menu-active", link.dataset.page === page);
  });
}

function renderActions(page) {
  const actions = document.getElementById("page-actions");
  actions.innerHTML = "";

  for (const control of pages[page].controls) {
    if (control.type === "button") {
      const button = document.createElement("button");
      button.className = control.className;
      button.textContent = control.label;
      if (control.route) {
        button.dataset.route = control.route;
      }
      actions.appendChild(button);
      continue;
    }

    if (control.type === "select") {
      const select = document.createElement("select");
      select.className = control.className;

      for (const optionLabel of control.options) {
        const option = document.createElement("option");
        option.textContent = optionLabel;
        select.appendChild(option);
      }

      actions.appendChild(select);
    }
  }
}

async function loadPage(page, pushState) {
  const safePage = pages[page] ? page : "overview";
  const title = document.getElementById("page-title");
  const content = document.getElementById("page-content");
  const error = document.getElementById("router-error");

  title.textContent = pages[safePage].title;
  setActiveLink(safePage);
  renderActions(safePage);
  error.classList.add("hidden");

  try {
    const response = await fetch(`./${safePage}.html`, { cache: "no-store" });

    if (!response.ok) {
      throw new Error(`Failed to load ${safePage}.html`);
    }

    content.innerHTML = await response.text();

    if (pushState) {
      const url = new URL(window.location.href);
      url.searchParams.set("page", safePage);
      window.history.pushState({ page: safePage }, "", url);
    }
  } catch (err) {
    content.innerHTML = "";
    error.textContent =
      "Could not load the page partial. Serve the wireframe directory over HTTP instead of opening index.html with file://.";
    error.classList.remove("hidden");
    console.error(err);
  }
}

document.addEventListener("click", (event) => {
  const link = event.target.closest("[data-page]");
  if (!link) {
    return;
  }

  event.preventDefault();
  loadPage(link.dataset.page, true);
});

document.addEventListener("click", (event) => {
  const card = event.target.closest("[data-route]");
  if (!card) {
    return;
  }

  event.preventDefault();
  loadPage(card.dataset.route, true);
});

window.addEventListener("popstate", (event) => {
  loadPage(event.state?.page || getPageFromLocation(), false);
});

loadPage(getPageFromLocation(), false);
