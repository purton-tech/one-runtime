const pages = {
  overview: {
    title: "Overview",
    controls: [
      {
        type: "button",
        label: "View provider catalog",
        className: "btn btn-outline",
        route: "providers",
      },
      {
        type: "button",
        label: "Generate connection link",
        className: "btn btn-primary",
        route: "provider-required",
      },
    ],
  },
  projects: {
    title: "Projects",
    controls: [
      {
        type: "button",
        label: "Create project",
        className: "btn btn-primary",
      },
    ],
  },
  providers: {
    title: "Providers",
    controls: [
      {
        type: "button",
        label: "Browse all providers",
        className: "btn btn-primary",
        route: "providers",
      },
      {
        type: "button",
        label: "Import provider spec",
        className: "btn btn-outline",
      },
    ],
  },
  "connect-gmail": {
    title: "Connect Gmail",
    controls: [],
  },
  "managed-oauth-confirm": {
    title: "Connect With One Runtime",
    controls: [
      {
        type: "button",
        label: "Continue to Google",
        className: "btn btn-primary",
        route: "gmail-connected",
      },
    ],
  },
  "byo-oauth-setup": {
    title: "Use Your Own Google OAuth App",
    controls: [
      {
        type: "button",
        label: "Save & Continue",
        className: "btn btn-primary",
        route: "byo-oauth-confirm",
      },
    ],
  },
  "byo-oauth-confirm": {
    title: "Connect Gmail",
    controls: [
      {
        type: "button",
        label: "Continue to Google",
        className: "btn btn-primary",
        route: "gmail-connected",
      },
    ],
  },
  "gmail-connected": {
    title: "Gmail Connected",
    controls: [
      {
        type: "button",
        label: "Reconnect",
        className: "btn btn-outline",
        route: "connect-gmail",
      },
      {
        type: "button",
        label: "Generate connection link",
        className: "btn btn-primary",
        route: "api-keys",
      },
    ],
  },
  "gmail-activity": {
    title: "Gmail Activity",
    controls: [
      {
        type: "select",
        className: "select select-bordered",
        options: ["Last 24 hours", "Last 7 days", "Last 30 days"],
      },
      {
        type: "button",
        label: "Export audit log",
        className: "btn btn-outline",
      },
    ],
  },
  "provider-required": {
    title: "Provider Required",
    controls: [],
  },
  "api-keys": {
    title: "API Keys",
    controls: [
      {
        type: "button",
        label: "Create API key",
        className: "btn btn-primary",
        route: "provider-required",
      },
    ],
  },
  "oauth-configuration": {
    title: "OAuth Configuration",
    controls: [
      {
        type: "button",
        label: "Save changes",
        className: "btn btn-primary",
      },
    ],
  },
  "how-connections-work": {
    title: "How Connections Work",
    controls: [
      {
        type: "button",
        label: "Browse providers",
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
