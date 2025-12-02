<template>
  <div id="app">
    <el-container style="height: 100vh;">
      <el-main>
        <div class="content">
          <h1>Tauri 登录 Token/Cookie 捕获演示</h1>
          
          <el-button 
            type="primary" 
            size="large" 
            @click="openLoginWindow"
            :loading="loading"
          >
            打开登录窗口
          </el-button>

          <el-divider></el-divider>

          <div v-if="capturedData.token || capturedData.cookies" class="result-section">
            <h2>捕获的数据：</h2>
            
            <el-card class="data-card" shadow="hover">
              <template #header>
                <div class="card-header">
                  <span>Token 信息</span>
                  <el-tag v-if="capturedData.token" type="success">已捕获</el-tag>
                  <el-tag v-else type="info">未捕获</el-tag>
                </div>
              </template>
              <div class="data-content">
                <pre>{{ capturedData.token || '暂无 Token 数据' }}</pre>
              </div>
            </el-card>

            <el-card class="data-card" shadow="hover">
              <template #header>
                <div class="card-header">
                  <span>Cookie 信息</span>
                  <el-tag v-if="capturedData.cookies" type="success">已捕获</el-tag>
                  <el-tag v-else type="info">未捕获</el-tag>
                </div>
              </template>
              <div class="data-content">
                <pre>{{ capturedData.cookies || '暂无 Cookie 数据' }}</pre>
              </div>
            </el-card>

            <el-card class="data-card" shadow="hover">
              <template #header>
                <div class="card-header">
                  <span>完整数据 (JSON)</span>
                </div>
              </template>
              <div class="data-content">
                <pre>{{ JSON.stringify(capturedData, null, 2) }}</pre>
              </div>
            </el-card>
          </div>

          <div v-else class="empty-state">
            <el-empty description="点击上方按钮打开登录窗口并完成登录以捕获数据"></el-empty>
          </div>
        </div>
      </el-main>
    </el-container>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ElMessage } from 'element-plus'

const loading = ref(false)
const capturedData = ref({
  token: '',
  cookies: '',
  url: '',
  timestamp: ''
})

const openLoginWindow = async () => {
  try {
    loading.value = true
    ElMessage.info('正在打开登录窗口...')
    await invoke('open_login_window')
    ElMessage.success('登录窗口已打开')
  } catch (error) {
    console.error('打开登录窗口失败:', error)
    ElMessage.error(`打开登录窗口失败: ${error}`)
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  // 监听来自登录窗口的数据
  await listen('login-data-captured', (event) => {
    console.log('收到登录数据:', event.payload)
    capturedData.value = event.payload
    
    // 在控制台打印详细信息
    console.log('========== 捕获的登录信息 ==========')
    console.log('Token:', event.payload.token)
    console.log('Cookies:', event.payload.cookies)
    console.log('URL:', event.payload.url)
    console.log('时间戳:', event.payload.timestamp)
    console.log('===================================')
    
    ElMessage.success('成功捕获登录数据！')
  })

  console.log('主窗口已加载，等待登录数据...')
})
</script>

<style scoped>
#app {
  font-family: 'Helvetica Neue', Helvetica, Arial, sans-serif;
}

.content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

h1 {
  text-align: center;
  color: #409eff;
  margin-bottom: 30px;
}

h2 {
  color: #303133;
  margin-bottom: 20px;
}

.result-section {
  margin-top: 30px;
}

.data-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: bold;
}

.data-content {
  max-height: 300px;
  overflow: auto;
}

.data-content pre {
  margin: 0;
  white-space: pre-wrap;
  word-wrap: break-word;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.5;
  background-color: #f5f7fa;
  padding: 15px;
  border-radius: 4px;
}

.empty-state {
  margin-top: 50px;
}

.el-button {
  display: block;
  margin: 0 auto;
}
</style>
