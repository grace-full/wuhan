(function () {
  "use strict";

  const safeInvoke = (payload) => {
    const core = window.__TAURI__ && window.__TAURI__.core;
    if (!core || typeof core.invoke !== "function") {
      return;
    }
    core.invoke("report_login_state", payload).catch(() => {
      /* noop */
    });
  };

  const pickFromStorage = (storage) => {
    if (!storage) return "";
    try {
      const length = storage.length || 0;
      for (let i = 0; i < length; i += 1) {
        const key = storage.key ? storage.key(i) : Object.keys(storage)[i];
        if (!key) continue;
        if (key.toLowerCase().includes("token")) {
          const value = storage.getItem ? storage.getItem(key) : storage[key];
          if (value) return value;
        }
      }
    } catch (_) {
      return "";
    }
    return "";
  };

  const pickFromCookie = () => {
    try {
      const cookie = document.cookie || "";
      const match = cookie
        .split(";")
        .map((item) => item.trim())
        .find((item) => item.toLowerCase().includes("token="));
      if (!match) return "";
      const [, value] = match.split("=");
      return value || "";
    } catch (_) {
      return "";
    }
  };

  const collectAuth = () => {
    const token =
      pickFromStorage(window.localStorage) ||
      pickFromStorage(window.sessionStorage) ||
      pickFromCookie() ||
      "";
    const cookie = (() => {
      try {
        return document.cookie || "";
      } catch (_) {
        return "";
      }
    })();
    return { token, cookie };
  };

  const createReporter = () => {
    let lastToken = "";
    let lastCookie = "";
    return () => {
      const { token, cookie } = collectAuth();
      if (!token && !cookie) {
        return;
      }
      if (token === lastToken && cookie === lastCookie) {
        return;
      }
      lastToken = token;
      lastCookie = cookie;
      safeInvoke({ token, cookie });
    };
  };

  const enhanceHistory = (reporter) => {
    ["pushState", "replaceState"].forEach((method) => {
      const original = history[method];
      if (typeof original !== "function") return;
      history[method] = function () {
        const result = original.apply(this, arguments);
        setTimeout(reporter, 150);
        return result;
      };
    });
  };

  const setup = () => {
    const report = createReporter();
    report();
    enhanceHistory(report);

    const triggerSoon = () => setTimeout(report, 120);

    window.addEventListener("focus", report);
    window.addEventListener("hashchange", report);
    window.addEventListener("popstate", report);
    document.addEventListener("visibilitychange", () => {
      if (document.visibilityState === "visible") {
        report();
      }
    });
    document.addEventListener("click", triggerSoon, true);
    document.addEventListener("keyup", triggerSoon, true);

    setInterval(report, 1200);
  };

  if (document.readyState === "complete" || document.readyState === "interactive") {
    setup();
  } else {
    document.addEventListener("DOMContentLoaded", setup, { once: true });
  }
})();
