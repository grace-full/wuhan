<template>
  <div class="app-shell">
    <el-card class="app-card" shadow="hover">
      <div class="header">
        <div>
          <h1>租号玩登录捕获演示</h1>
          <p>Electron + Vue3 + Element Plus</p>
        </div>
        <el-button type="primary" size="large" :loading="loading" @click="startLogin">
          打开登录窗口
        </el-button>
      </div>

      <el-alert
        title="点击按钮将打开内嵌的第三方登录窗口，我们会自动监听跳转并捕获 token 与 cookie。"
        type="info"
        :closable="false"
        show-icon
        class="tip"
      />

      <el-divider content-position="left">捕获结果</el-divider>

      <el-descriptions class="result" :column="1" border>
        <el-descriptions-item label="Token">
          <template v-if="authState.token">
            <code class="token">{{ authState.token }}</code>
          </template>
          <el-tag v-else type="info">尚未获取</el-tag>
        </el-descriptions-item>
        <el-descriptions-item label="最近跳转 URL">
          <span v-if="authState.lastUrl" class="url">{{ authState.lastUrl }}</span>
          <el-tag v-else type="info">尚未记录</el-tag>
        </el-descriptions-item>
        <el-descriptions-item label="Cookie 串">
          <el-scrollbar height="120px">
            <pre v-if="authState.rawCookieHeader" class="cookie-text">{{ authState.rawCookieHeader }}</pre>
            <el-tag v-else type="info">尚未获取</el-tag>
          </el-scrollbar>
        </el-descriptions-item>
      </el-descriptions>

      <el-divider content-position="left">Cookie 明细</el-divider>
      <el-table v-if="authState.cookies.length" :data="authState.cookies" height="260" border stripe size="small">
        <el-table-column prop="name" label="Name" width="180" sortable />
        <el-table-column prop="value" label="Value" min-width="260" />
        <el-table-column prop="domain" label="Domain" width="200" />
        <el-table-column prop="path" label="Path" width="100" />
        <el-table-column prop="httpOnly" label="HttpOnly" width="110">
          <template #default="{ row }">
            <el-tag :type="row.httpOnly ? 'success' : 'info'">{{ row.httpOnly ? 'Yes' : 'No' }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="secure" label="Secure" width="110">
          <template #default="{ row }">
            <el-tag :type="row.secure ? 'success' : 'info'">{{ row.secure ? 'Yes' : 'No' }}</el-tag>
          </template>
        </el-table-column>
      </el-table>
      <el-empty v-else description="尚未捕获到 Cookie" />
    </el-card>
  </div>
</template>

<script setup>
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { ElMessage } from 'element-plus';

const loading = ref(false);
const authState = ref({
  token: '',
  rawCookieHeader: '',
  cookies: [],
  lastUrl: ''
});

let stopAuthListener = null;

const updateAuthState = (payload = {}) => {
  if (!payload) return;
  authState.value = {
    ...authState.value,
    ...payload,
    cookies: Array.isArray(payload.cookies) ? payload.cookies : authState.value.cookies
  };
  console.log('[Renderer] 捕获到登录数据', authState.value);
};

const startLogin = async () => {
  if (!window.electronAPI) {
    ElMessage.error('Electron API 不可用');
    return;
  }

  loading.value = true;
  try {
    const snapshot = await window.electronAPI.startLoginFlow();
    updateAuthState(snapshot);
    ElMessage.success('登录窗口已打开');
  } catch (error) {
    console.error(error);
    ElMessage.error('无法打开登录窗口');
  } finally {
    loading.value = false;
  }
};

onMounted(async () => {
  if (!window.electronAPI) {
    console.warn('Electron 环境未检测到 electronAPI');
    return;
  }

  const snapshot = await window.electronAPI.getAuthState();
  updateAuthState(snapshot);

  stopAuthListener = window.electronAPI.onAuthData((payload) => {
    updateAuthState(payload);
  });
});

onBeforeUnmount(() => {
  if (typeof stopAuthListener === 'function') {
    stopAuthListener();
  }
});
</script>
