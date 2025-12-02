import { 
  openLoginWindow, 
  isAuthenticated, 
  fetchFromAPI, 
  logout 
} from './auth.js';

// UI elements
const statusEl = document.getElementById('status');
const loginBtn = document.getElementById('loginBtn');
const testApiBtn = document.getElementById('testApiBtn');
const logoutBtn = document.getElementById('logoutBtn');
const logEl = document.getElementById('log');

// Logging function
function log(message, type = 'info') {
  const entry = document.createElement('div');
  entry.className = `log-entry ${type}`;
  const timestamp = new Date().toLocaleTimeString();
  entry.textContent = `[${timestamp}] ${message}`;
  logEl.appendChild(entry);
  logEl.scrollTop = logEl.scrollHeight;
}

// Update UI based on auth status
async function updateUI() {
  const authenticated = await isAuthenticated();
  
  if (authenticated) {
    statusEl.textContent = '✅ Authenticated';
    statusEl.className = 'status authenticated';
    loginBtn.disabled = true;
    testApiBtn.disabled = false;
    logoutBtn.disabled = false;
  } else {
    statusEl.textContent = '❌ Not authenticated';
    statusEl.className = 'status not-authenticated';
    loginBtn.disabled = false;
    testApiBtn.disabled = true;
    logoutBtn.disabled = true;
  }
}

// Event handlers
loginBtn.addEventListener('click', async () => {
  try {
    log('Opening login window...', 'info');
    await openLoginWindow();
    log('Login window opened successfully', 'success');
    log('Waiting for authentication...', 'info');
  } catch (error) {
    log(`Failed to open login window: ${error}`, 'error');
    console.error('Login error:', error);
  }
});

testApiBtn.addEventListener('click', async () => {
  try {
    log('Making API request...', 'info');
    const data = await fetchFromAPI('test-endpoint');
    log('API request successful', 'success');
    log(`Response: ${JSON.stringify(data)}`, 'info');
  } catch (error) {
    log(`API request failed: ${error}`, 'error');
    console.error('API error:', error);
  }
});

logoutBtn.addEventListener('click', async () => {
  try {
    log('Logging out...', 'info');
    await logout();
    log('Logged out successfully', 'success');
    await updateUI();
  } catch (error) {
    log(`Logout failed: ${error}`, 'error');
    console.error('Logout error:', error);
  }
});

// Listen for authentication success messages
window.addEventListener('message', async (event) => {
  if (event.data.type === 'auth_success') {
    log('Authentication successful!', 'success');
    await updateUI();
  }
});

// Initialize
log('Application started', 'info');
log('Cross-origin security enabled', 'info');
updateUI();
