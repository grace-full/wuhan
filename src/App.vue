<template>
  <div class="app-shell">
    <header class="hero">
      <div>
        <p class="eyebrow">Tauri 2.8 · Vue 3 · Element Plus</p>
        <h1>
          <el-icon class="hero-icon">
            <UserFilled />
          </el-icon>
          租号玩登录凭证捕获演示
        </h1>
        <p class="subtitle">
          点击下方按钮会弹出一个加载 https://zu.zuhaowan.com 的独立 WebView 窗口，并自动
          注入脚本收集登录成功后的 token 与 cookie，再通过事件的方式安全回传给主窗口。
        </p>
      </div>
      <el-tag :type="statusMeta.type" size="large">{{ statusMeta.text }}</el-tag>
    </header>

    <section class="cards">
      <el-card class="action-card">
        <template #header>
          <div class="card-header">
            <span>登录操作</span>
            <span class="hint">所有窗口通讯都通过 Tauri invoke + event 完成</span>
          </div>
        </template>
        <el-alert
          type="info"
          show-icon
          :closable="false"
          title="1. 主窗口通过 invoke 调用 Rust 创建登录 WebView；2. Rust 注入脚本监听 token/cookie；3. 结果通过事件发回主窗口"
        />
        <div class="actions">
          <el-button
            size="large"
            type="primary"
            :loading="buttonLoading"
            @click="openLoginWindow"
          >
            打开租号玩登录页
          </el-button>
          <el-button size="large" @click="resetPayload" :disabled="!payload">
            清空结果
          </el-button>
        </div>
      </el-card>

      <el-card class="result-card">
        <template #header>
          <div class="card-header">
            <span>实时登录凭证</span>
            <span class="hint">最近捕获时间：{{ formattedTime }}</span>
          </div>
        </template>
        <div v-if="payload" class="result-body">
          <el-descriptions :column="1" border>
            <el-descriptions-item label="Token">
              <div class="desc-line">
                <code class="token-value">{{ payload.token || "未检测到 token" }}</code>
                <el-button
                  v-if="payload.token"
                  type="primary"
                  text
                  size="small"
                  @click="copy(payload.token)"
                >
                  复制 token
                </el-button>
              </div>
            </el-descriptions-item>
            <el-descriptions-item label="Cookie">
              <el-input
                class="cookie-area"
                type="textarea"
                readonly
                :autosize="{ minRows: 3, maxRows: 6 }"
                :model-value="payload.cookie || ''"
              />
              <el-button
                v-if="payload.cookie"
                class="cookie-copy"
                text
                size="small"
                type="primary"
                @click="copy(payload.cookie)"
              >
                复制 cookie
              </el-button>
            </el-descriptions-item>
            <el-descriptions-item label="来源窗口">
              {{ payload.source }}
            </el-descriptions-item>
          </el-descriptions>
        </div>
        <el-empty v-else description="暂未捕获到 token / cookie" />
      </el-card>

      <el-card class="history-card">
        <template #header>
          <div class="card-header">
            <span>最近捕获记录（最多 5 条）</span>
            <span class="hint">包含 token 概要、cookie 长度与捕获时间</span>
          </div>
        </template>
        <el-empty v-if="history.length === 0" description="暂无历史记录" />
        <el-timeline v-else>
          <el-timeline-item
            v-for="item in history"
            :key="item.captured_at"
            :timestamp="formatTimestamp(item.captured_at)"
          >
            <p class="history-token">token：{{ summarizeToken(item.token) }}</p>
            <p class="history-cookie">cookie：{{ summarizeCookie(item.cookie) }}</p>
          </el-timeline-item>
        </el-timeline>
      </el-card>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { ElMessage } from "element-plus";
import { UserFilled } from "@element-plus/icons-vue";

interface LoginPayload {
  token?: string | null;
  cookie?: string | null;
  source: string;
  captured_at: number;
}

type StatusKey = "idle" | "opening" | "waiting" | "received";

const status = ref<StatusKey>("idle");
const payload = ref<LoginPayload | null>(null);
const history = ref<LoginPayload[]>([]);
const isOpening = ref(false);

const STATUS_MAP: Record<StatusKey, { text: string; type: "info" | "success" | "warning" | "danger" }> = {
  idle: { text: "等待操作", type: "info" },
  opening: { text: "正在打开登录窗口", type: "warning" },
  waiting: { text: "请在弹出的页面完成登录", type: "warning" },
  received: { text: "已捕获凭证", type: "success" },
};

const formatTimestamp = (value?: number) => {
  if (!value) return "--";
  return new Date(value).toLocaleString();
};

const statusMeta = computed(() => STATUS_MAP[status.value]);
const buttonLoading = computed(() => isOpening.value || status.value === "waiting");
const formattedTime = computed(() => formatTimestamp(payload.value?.captured_at));

let unlisten: UnlistenFn | null = null;

onMounted(async () => {
  unlisten = await listen<LoginPayload>("login-info", (event) => {
    payload.value = event.payload;
    history.value = [event.payload, ...history.value].slice(0, 5);
    status.value = "received";
    isOpening.value = false;
    ElMessage.success("已成功捕获登录 token / cookie");
  });
});

onBeforeUnmount(() => {
  unlisten?.();
  unlisten = null;
});

const openLoginWindow = async () => {
  if (buttonLoading.value) {
    return;
  }
  isOpening.value = true;
  status.value = "opening";
  try {
    await invoke("open_login_window");
    status.value = "waiting";
    ElMessage.info("登录窗口已弹出，请在新窗口内完成账号密码输入");
  } catch (error) {
    status.value = "idle";
    ElMessage.error(`打开登录窗口失败: ${String(error)}`);
  } finally {
    isOpening.value = false;
  }
};

const resetPayload = () => {
  payload.value = null;
  status.value = "idle";
};

const copy = async (text?: string | null) => {
  if (!text) {
    ElMessage.warning("当前没有可复制的内容");
    return;
  }
  try {
    await navigator.clipboard.writeText(text);
    ElMessage.success("已复制到剪贴板");
  } catch (err) {
    console.error(err);
    ElMessage.error("复制失败，请检查系统剪贴板权限");
  }
};

const summarizeToken = (token?: string | null) => {
  if (!token) return "无";
  return token.length > 24 ? `${token.slice(0, 24)}…` : token;
};

const summarizeCookie = (cookie?: string | null) => {
  if (!cookie) return "无";
  return cookie.length > 42 ? `${cookie.slice(0, 42)}…` : cookie;
};
</script>

<style scoped>
.app-shell {
  max-width: 960px;
  margin: 0 auto;
  padding: 32px 24px 48px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.hero {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  align-items: flex-start;
  justify-content: space-between;
}

.hero h1 {
  margin: 8px 0;
  font-size: 28px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.subtitle {
  margin: 0;
  max-width: 640px;
  color: #6b7280;
}

.eyebrow {
  margin: 0;
  text-transform: uppercase;
  font-size: 12px;
  letter-spacing: 0.2em;
  color: #94a3b8;
}

.hero-icon {
  color: #6366f1;
}

.cards {
  display: grid;
  gap: 20px;
}

@media (min-width: 900px) {
  .cards {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .history-card {
    grid-column: span 2;
  }
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  font-weight: 600;
}

.card-header .hint {
  font-size: 12px;
  font-weight: 400;
  color: #94a3b8;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  margin-top: 16px;
}

.result-body {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.desc-line {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-wrap: wrap;
}

.token-value {
  font-family: "JetBrains Mono", Consolas, "SFMono-Regular", Menlo, monospace;
  font-size: 14px;
  padding: 4px 8px;
  background: #f1f5f9;
  border-radius: 6px;
  word-break: break-all;
}

.cookie-area {
  margin-top: 8px;
}

.cookie-copy {
  margin-top: 8px;
}

.history-token,
.history-cookie {
  margin: 0;
  font-size: 14px;
}

.history-token {
  font-weight: 500;
}
</style>
