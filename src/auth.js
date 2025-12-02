/**
 * Authentication module demonstrating secure cross-origin communication
 * 
 * This module shows how to:
 * 1. Open a login window to a remote domain
 * 2. Handle authentication callbacks
 * 3. Store tokens securely
 * 4. Make authenticated API requests
 */

import { invoke } from '@tauri-apps/api/core';
import { Store } from 'tauri-plugin-store-api';

// Initialize secure store for tokens
const store = new Store('.auth.dat');

/**
 * Opens the login window which loads the remote authentication page
 * The remote page is whitelisted in tauri.conf.json under:
 * - dangerousRemoteDomainIpcAccess
 * - capabilities with remote.urls
 */
export async function openLoginWindow() {
    try {
        await invoke('open_login_window');
        console.log('Login window opened successfully');
    } catch (error) {
        console.error('Failed to open login window:', error);
        throw error;
    }
}

/**
 * Handles authentication callback from the remote login page
 * This is called when the remote page successfully authenticates
 */
export async function handleAuthCallback(authToken) {
    try {
        // Store token securely using tauri-plugin-store
        await store.set('auth_token', authToken);
        await store.save();
        
        // Notify backend
        await invoke('store_auth_token', { token: authToken });
        
        // Close login window
        await invoke('close_login_window');
        
        console.log('Authentication successful, token stored securely');
        return true;
    } catch (error) {
        console.error('Failed to handle auth callback:', error);
        throw error;
    }
}

/**
 * Retrieves the stored authentication token
 */
export async function getAuthToken() {
    try {
        const token = await store.get('auth_token');
        return token;
    } catch (error) {
        console.error('Failed to retrieve auth token:', error);
        return null;
    }
}

/**
 * Makes an authenticated API request using the Tauri HTTP plugin
 * This bypasses CORS restrictions
 */
export async function fetchFromAPI(endpoint) {
    try {
        const response = await invoke('fetch_from_api', { endpoint });
        return JSON.parse(response);
    } catch (error) {
        console.error('API request failed:', error);
        throw error;
    }
}

/**
 * Clears stored authentication data
 */
export async function logout() {
    try {
        await store.delete('auth_token');
        await store.save();
        console.log('Logged out successfully');
    } catch (error) {
        console.error('Failed to logout:', error);
        throw error;
    }
}

/**
 * Checks if user is authenticated
 */
export async function isAuthenticated() {
    const token = await getAuthToken();
    return token !== null && token !== undefined;
}

// Listen for authentication messages from the login window
// The remote page can send messages via postMessage or Tauri IPC
if (typeof window !== 'undefined') {
    window.addEventListener('message', async (event) => {
        // Verify origin is from allowed domain
        const allowedOrigins = [
            'https://login.example.com',
            'https://auth.example.com'
        ];
        
        if (!allowedOrigins.includes(event.origin)) {
            console.warn('Received message from unauthorized origin:', event.origin);
            return;
        }
        
        // Handle authentication message
        if (event.data.type === 'auth_success' && event.data.token) {
            await handleAuthCallback(event.data.token);
        }
    });
}
