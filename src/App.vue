<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

const greetMsg = ref("");
const name = ref("");
const authToken = ref<string | null>(null);
const authCookies = ref<Array<{ name: string; value: string }>>([]);
const authUrl = ref<string>("");
const authDataReceived = ref(false);
const errorMsg = ref<string>("");

let unlistenAuthData: UnlistenFn | null = null;
let unlistenAuthResult: UnlistenFn | null = null;
let unlistenAuthError: UnlistenFn | null = null;

interface AuthData {
  token: string | null;
  cookies: Array<{ name: string; value: string }>;
  url: string;
}

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function openAuthWindow() {
  try {
    errorMsg.value = "";
    authDataReceived.value = false;
    authToken.value = null;
    authCookies.value = [];
    authUrl.value = "";
    
    await invoke("open_auth_window");
    console.log("Auth window opened");
  } catch (error) {
    console.error("Failed to open auth window:", error);
    errorMsg.value = `Failed to open auth window: ${error}`;
  }
}

onMounted(async () => {
  // Listen for auth data captured events
  unlistenAuthData = await listen<AuthData>("auth-data-captured", (event) => {
    console.log("Auth data captured!", event.payload);
    
    authToken.value = event.payload.token || null;
    authCookies.value = event.payload.cookies || [];
    authUrl.value = event.payload.url || "";
    authDataReceived.value = true;
    
    // Log to console
    console.log("Token:", authToken.value);
    console.log("Cookies:", authCookies.value);
    console.log("URL:", authUrl.value);
    
    // Display in UI
    if (authToken.value) {
      console.log(`🎉 Authentication successful! Token: ${authToken.value}`);
    }
  });
  
  // Listen for extraction results
  unlistenAuthResult = await listen<string>("auth-extraction-result", (event) => {
    console.log("Auth extraction result:", event.payload);
    
    try {
      const data = JSON.parse(event.payload);
      console.log("Parsed auth data:", data);
      
      if (data.cookies) {
        authCookies.value = data.cookies;
        console.log("Cookies extracted:", data.cookies);
      }
      
      if (data.localStorage) {
        console.log("LocalStorage:", data.localStorage);
        
        // Check for common token keys in localStorage
        for (const [key, value] of Object.entries(data.localStorage)) {
          if (key.toLowerCase().includes('token') || 
              key.toLowerCase().includes('auth') ||
              key.toLowerCase().includes('access')) {
            authToken.value = value as string;
            console.log(`Found token in localStorage[${key}]:`, value);
          }
        }
      }
      
      if (data.sessionStorage) {
        console.log("SessionStorage:", data.sessionStorage);
        
        // Check for common token keys in sessionStorage
        for (const [key, value] of Object.entries(data.sessionStorage)) {
          if (key.toLowerCase().includes('token') || 
              key.toLowerCase().includes('auth') ||
              key.toLowerCase().includes('access')) {
            if (!authToken.value) {
              authToken.value = value as string;
              console.log(`Found token in sessionStorage[${key}]:`, value);
            }
          }
        }
      }
      
      if (data.url) {
        authUrl.value = data.url;
      }
      
      authDataReceived.value = true;
    } catch (e) {
      console.error("Failed to parse auth extraction result:", e);
    }
  });
  
  // Listen for extraction errors
  unlistenAuthError = await listen<string>("auth-extraction-error", (event) => {
    console.error("Auth extraction error:", event.payload);
    errorMsg.value = `Auth extraction error: ${event.payload}`;
  });
});

onUnmounted(() => {
  if (unlistenAuthData) unlistenAuthData();
  if (unlistenAuthResult) unlistenAuthResult();
  if (unlistenAuthError) unlistenAuthError();
});
</script>

<template>
  <main class="container">
    <h1>Tauri Auth Capture Demo</h1>

    <div class="section">
      <h2>Test Basic Functionality</h2>
      <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>
      <p v-if="greetMsg" class="result">{{ greetMsg }}</p>
    </div>

    <div class="section">
      <h2>Authentication Capture</h2>
      <button @click="openAuthWindow" class="auth-button">
        Open Authentication Window
      </button>
      <p class="info">
        Click the button above to open an authentication window. The app will
        automatically capture tokens and cookies when authentication succeeds.
      </p>
    </div>

    <div v-if="authDataReceived" class="section auth-data">
      <h2>✅ Captured Authentication Data</h2>
      
      <div v-if="authToken" class="data-block">
        <h3>Token</h3>
        <pre>{{ authToken }}</pre>
      </div>
      
      <div v-if="authUrl" class="data-block">
        <h3>URL</h3>
        <pre>{{ authUrl }}</pre>
      </div>
      
      <div v-if="authCookies.length > 0" class="data-block">
        <h3>Cookies</h3>
        <div v-for="cookie in authCookies" :key="cookie.name" class="cookie-item">
          <strong>{{ cookie.name }}:</strong> {{ cookie.value }}
        </div>
      </div>
      
      <div v-else class="data-block">
        <p class="info">No cookies captured yet. This may be normal if the auth flow uses tokens instead.</p>
      </div>
    </div>

    <div v-if="errorMsg" class="section error">
      <h3>Error</h3>
      <p>{{ errorMsg }}</p>
    </div>

    <div class="section instructions">
      <h2>Instructions</h2>
      <ol>
        <li>Click "Open Authentication Window" to start the authentication flow</li>
        <li>Complete the login process in the popup window</li>
        <li>Upon successful authentication, tokens and cookies will be captured automatically</li>
        <li>The captured data will be displayed on this page and logged to the console</li>
      </ol>
      <p class="note">
        <strong>Note:</strong> The demo opens example.com by default. For real usage, 
        update the URL in <code>src-tauri/src/lib.rs</code> to point to your actual 
        authentication provider.
      </p>
    </div>
  </main>
</template>

<style scoped>
.container {
  margin: 0 auto;
  padding: 2rem;
  max-width: 900px;
  text-align: left;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
  color: #24c8db;
}

.section {
  margin-bottom: 2rem;
  padding: 1.5rem;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
  border: 1px solid rgba(255, 255, 255, 0.1);
}

.row {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  transition: border-color 0.25s;
}

button {
  cursor: pointer;
  background-color: #24c8db;
  color: #0f0f0f;
  font-weight: 600;
}

button:hover {
  background-color: #1da7bd;
  border-color: #24c8db;
}

.auth-button {
  width: 100%;
  padding: 1rem;
  font-size: 1.1em;
  margin-bottom: 1rem;
}

.result {
  color: #4ade80;
  font-weight: 500;
}

.info {
  color: #94a3b8;
  font-size: 0.9em;
  margin: 0.5rem 0;
}

.note {
  background: rgba(36, 200, 219, 0.1);
  padding: 1rem;
  border-radius: 6px;
  border-left: 3px solid #24c8db;
  margin-top: 1rem;
}

.auth-data {
  background: rgba(74, 222, 128, 0.1);
  border-color: #4ade80;
}

.data-block {
  margin-bottom: 1.5rem;
}

.data-block h3 {
  color: #4ade80;
  margin-bottom: 0.5rem;
  font-size: 1.1em;
}

.data-block pre {
  background: rgba(0, 0, 0, 0.3);
  padding: 1rem;
  border-radius: 6px;
  overflow-x: auto;
  font-size: 0.9em;
  word-break: break-all;
  white-space: pre-wrap;
}

.cookie-item {
  background: rgba(0, 0, 0, 0.2);
  padding: 0.5rem 1rem;
  border-radius: 4px;
  margin-bottom: 0.5rem;
  font-family: monospace;
}

.error {
  background: rgba(239, 68, 68, 0.1);
  border-color: #ef4444;
  color: #ef4444;
}

.instructions {
  background: rgba(147, 51, 234, 0.05);
  border-color: rgba(147, 51, 234, 0.3);
}

.instructions ol {
  margin: 1rem 0;
  padding-left: 1.5rem;
}

.instructions li {
  margin-bottom: 0.5rem;
}

code {
  background: rgba(0, 0, 0, 0.3);
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.9em;
}

@media (prefers-color-scheme: light) {
  .section {
    background: rgba(0, 0, 0, 0.02);
    border-color: rgba(0, 0, 0, 0.1);
  }

  button {
    color: #ffffff;
  }

  .data-block pre,
  .cookie-item {
    background: rgba(0, 0, 0, 0.05);
  }

  code {
    background: rgba(0, 0, 0, 0.08);
  }
}
</style>
