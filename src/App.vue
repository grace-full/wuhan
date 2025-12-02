<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { ElMessage } from 'element-plus'
import { Lock, Check } from '@element-plus/icons-vue'
import { useAuthStore, type AuthData } from './stores/auth'

const authStore = useAuthStore()

let unlistenAuthData: UnlistenFn | null = null
let unlistenAuthResult: UnlistenFn | null = null
let unlistenAuthError: UnlistenFn | null = null

async function handleLogin() {
  try {
    authStore.clearAuthData()
    await invoke('open_auth_window')
    ElMessage.success('Login window opened')
    console.log('Auth window opened')
  } catch (error) {
    console.error('Failed to open auth window:', error)
    ElMessage.error(`Failed to open auth window: ${error}`)
  }
}

async function handleMockLogin() {
  try {
    authStore.clearAuthData()
    await invoke('send_mock_auth_data')
    ElMessage.success('Mock data sent successfully!')
    console.log('Mock auth data sent')
  } catch (error) {
    console.error('Failed to send mock data:', error)
    ElMessage.error(`Failed to send mock data: ${error}`)
  }
}

onMounted(async () => {
  unlistenAuthData = await listen<AuthData>('auth-data-captured', (event) => {
    console.log('Auth data captured!', event.payload)
    
    authStore.setAuthData({
      token: event.payload.token || null,
      cookies: event.payload.cookies || [],
      url: event.payload.url || ''
    })
    
    if (authStore.token) {
      ElMessage.success('Authentication successful!')
    }
  })
  
  unlistenAuthResult = await listen<string>('auth-extraction-result', (event) => {
    console.log('Auth extraction result:', event.payload)
    
    try {
      const data = JSON.parse(event.payload)
      console.log('Parsed auth data:', data)
      
      if (data.cookies) {
        console.log('Cookies extracted:', data.cookies)
      }
      
      if (data.localStorage) {
        console.log('LocalStorage:', data.localStorage)
        
        for (const [key, value] of Object.entries(data.localStorage)) {
          if (key.toLowerCase().includes('token') || 
              key.toLowerCase().includes('auth') ||
              key.toLowerCase().includes('access')) {
            authStore.updateToken(value as string)
            console.log(`Found token in localStorage[${key}]:`, value)
          }
        }
      }
      
      if (data.sessionStorage) {
        console.log('SessionStorage:', data.sessionStorage)
        
        for (const [key, value] of Object.entries(data.sessionStorage)) {
          if (key.toLowerCase().includes('token') || 
              key.toLowerCase().includes('auth') ||
              key.toLowerCase().includes('access')) {
            if (!authStore.token) {
              authStore.updateToken(value as string)
              console.log(`Found token in sessionStorage[${key}]:`, value)
            }
          }
        }
      }
      
      if (data.cookies) {
        data.cookies.forEach((cookie: any) => {
          authStore.addCookie(cookie)
        })
      }
    } catch (e) {
      console.error('Failed to parse auth extraction result:', e)
    }
  })
  
  unlistenAuthError = await listen<string>('auth-extraction-error', (event) => {
    console.error('Auth extraction error:', event.payload)
    ElMessage.error(`Auth extraction error: ${event.payload}`)
  })
})

onUnmounted(() => {
  if (unlistenAuthData) unlistenAuthData()
  if (unlistenAuthResult) unlistenAuthResult()
  if (unlistenAuthError) unlistenAuthError()
})
</script>

<template>
  <div class="app-container">
    <el-container direction="vertical">
      <el-header height="80px">
        <div class="header-content">
          <h1>Tauri Auth Demo</h1>
        </div>
      </el-header>

      <el-main>
        <div class="main-content">
          <el-card class="login-card" shadow="hover">
            <template #header>
              <div class="card-header">
                <span class="card-title">Authentication</span>
              </div>
            </template>

            <div class="login-section">
              <p class="description">
                Click the button below to open the authentication window and capture your login credentials.
              </p>

              <el-button 
                type="primary" 
                size="large"
                @click="handleLogin"
                class="login-button"
              >
                <el-icon class="el-icon--left"><Lock /></el-icon>
                Start Login Process
              </el-button>

              <el-divider>OR</el-divider>

              <el-button 
                type="success" 
                size="large"
                @click="handleMockLogin"
                class="mock-button"
                plain
              >
                Test with Mock Data
              </el-button>
            </div>
          </el-card>

          <el-card v-if="authStore.hasData" class="token-card" shadow="hover">
            <template #header>
              <div class="card-header">
                <span class="card-title">
                  <el-icon><Check /></el-icon>
                  Captured Authentication Data
                </span>
              </div>
            </template>

            <div class="token-content">
              <el-alert
                v-if="authStore.isAuthenticated"
                title="Authentication Successful!"
                type="success"
                :closable="false"
                show-icon
                class="success-alert"
              />

              <div v-if="authStore.token" class="data-section">
                <h3 class="section-title">Token</h3>
                <el-input
                  v-model="authStore.token"
                  type="textarea"
                  :rows="4"
                  readonly
                  class="token-display"
                />
              </div>

              <div v-if="authStore.url" class="data-section">
                <h3 class="section-title">Authentication URL</h3>
                <el-tag type="info" size="large" effect="plain">
                  {{ authStore.url }}
                </el-tag>
              </div>

              <div v-if="authStore.cookies.length > 0" class="data-section">
                <h3 class="section-title">Cookies ({{ authStore.cookies.length }})</h3>
                <el-table :data="authStore.cookies" stripe style="width: 100%">
                  <el-table-column prop="name" label="Name" width="200" />
                  <el-table-column prop="value" label="Value" show-overflow-tooltip />
                </el-table>
              </div>

              <div v-if="!authStore.token && authStore.cookies.length === 0" class="data-section">
                <el-empty description="No authentication data captured yet" />
              </div>
            </div>
          </el-card>

          <el-card class="info-card" shadow="never">
            <template #header>
              <div class="card-header">
                <span class="card-title">How It Works</span>
              </div>
            </template>

            <el-steps direction="vertical" :active="authStore.hasData ? 4 : 1">
              <el-step title="Click Login Button" description="Start the authentication process" />
              <el-step title="Complete Login" description="Login in the popup window" />
              <el-step title="Capture Data" description="Tokens and cookies are automatically captured" />
              <el-step title="View Results" description="See your authentication data displayed above" />
            </el-steps>

            <el-divider />

            <el-alert
              type="info"
              :closable="false"
              show-icon
            >
              <template #title>
                <strong>Note:</strong> This demo opens example.com by default. Update the URL in 
                <el-tag size="small" effect="plain">src-tauri/src/lib.rs</el-tag> for your actual authentication provider.
              </template>
            </el-alert>
          </el-card>
        </div>
      </el-main>

      <el-footer height="40px">
        <div class="footer-content">
          <span>Tauri + Vue 3 + Element Plus + Pinia</span>
        </div>
      </el-footer>
    </el-container>
  </div>
</template>

<style scoped>
.app-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.el-container {
  min-height: 100vh;
}

.el-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
}

.header-content {
  width: 100%;
  max-width: 1200px;
  text-align: center;
}

.header-content h1 {
  margin: 0;
  font-size: 2rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.el-main {
  padding: 2rem;
  display: flex;
  justify-content: center;
}

.main-content {
  width: 100%;
  max-width: 900px;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.login-card,
.token-card,
.info-card {
  background: rgba(255, 255, 255, 0.98);
  backdrop-filter: blur(10px);
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.card-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: #303133;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.login-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.5rem;
  padding: 2rem 0;
}

.description {
  text-align: center;
  color: #606266;
  font-size: 1rem;
  margin: 0;
  max-width: 600px;
}

.login-button {
  width: 100%;
  max-width: 400px;
  height: 56px;
  font-size: 1.1rem;
  font-weight: 600;
  border-radius: 28px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
  transition: all 0.3s ease;
}

.login-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
  background: linear-gradient(135deg, #764ba2 0%, #667eea 100%);
}

.mock-button {
  width: 100%;
  max-width: 400px;
  height: 48px;
  font-size: 1rem;
  font-weight: 600;
  border-radius: 24px;
}

.token-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.success-alert {
  border-radius: 8px;
}

.data-section {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.section-title {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: #409eff;
}

.token-display {
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
}

:deep(.el-textarea__inner) {
  background-color: #f5f7fa;
  border-color: #dcdfe6;
  font-family: 'Courier New', monospace;
}

:deep(.el-table) {
  border-radius: 8px;
  overflow: hidden;
}

.el-footer {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  box-shadow: 0 -2px 12px rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
}

.footer-content {
  text-align: center;
  color: #909399;
  font-size: 0.9rem;
}

@media (max-width: 768px) {
  .el-main {
    padding: 1rem;
  }

  .main-content {
    max-width: 100%;
  }

  .header-content h1 {
    font-size: 1.5rem;
  }

  .login-button {
    font-size: 1rem;
    height: 48px;
  }
}
</style>
