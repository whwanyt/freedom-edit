<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { FileEntry } from "@tauri-apps/api/fs";
import zhCN from "@arco-design/web-vue/es/locale/lang/zh-cn";
import { ref, watch } from "vue";
import FileListWidget from "./components/fileListWidget.vue";
import EditWidget from "./components/editWidget.vue";
import CreatePopoverWidget from "./components/createPopoverWidget.vue";
import { inject } from "vue";
import { AppConfigProvide, appConfigKey } from "./hook/appConfig";
import useInvokeHook from "./hook/invoke";
import { nextTick } from "vue";
import { onMounted } from "vue";

const invokeHook = useInvokeHook();
const dirInfoList = ref<FileEntry[]>([]);
const { baseDirPath } = inject<AppConfigProvide>(appConfigKey)!;
async function init() {
  if (baseDirPath.value.length > 0) {
    dirInfoList.value = await invokeHook.readDir(baseDirPath.value);
    invokeHook.watchDir(baseDirPath.value);
    console.log(
      "不要吹灭你的灵感和你的想象力; 不要成为你的模型的奴隶。 ——文森特・梵高"
    );
  }
}

const updateDir = async () => {
  dirInfoList.value = await invokeHook.readDir(baseDirPath.value);
};
const dirEvent = (event: any) => {
  const data = JSON.parse(event.payload);
  console.log("File event:", data);
  switch (data.event_type) {
    case "create":
      updateDir();
      break;
    case "remove":
      updateDir();
      break;
    default:
      break;
  }
};

listen("file_event", dirEvent);

init();
const openDir = async () => {
  const dir = await invokeHook.onDir();
  console.log(dir);
  if (!dir) return;
  baseDirPath.value = dir as string;
  await init();
};

const activeFile = ref<FileEntry>();
const activeContent = ref("");
const isDirShow = ref(false);

watch(
  () => baseDirPath.value,
  () => {
    if (baseDirPath.value.length > 0) {
      isDirShow.value = true;
    } else {
      isDirShow.value = false;
      activeFile.value = undefined;
      activeContent.value = "";
    }
  },
  { immediate: true }
);
const editViewRef = ref();
const openFile = async (val: FileEntry) => {
  if (editViewRef.value) {
    const isSave = await editViewRef.value.onClose();
    if (!isSave) return;
  }
  const content = await invokeHook.readFile(val.path);
  activeFile.value = undefined;
  activeContent.value = "";
  nextTick(() => {
    activeFile.value = val;
    activeContent.value = content;
  });
};
const splitSize = ref(0.3);

onMounted(() => {
  document.addEventListener("click", (event: any) => {
    const target = event.target;
    if (target && target.tagName === "A") {
      event.preventDefault();
      const href = target.getAttribute("href");
      if (href) {
        console.log("open_link", href);
        invokeHook.openLink(href);
      }
    }
  });
  // document.body.setAttribute("arco-theme", "dark");
});
</script>

<template>
  <a-config-provider :locale="zhCN">
    <div class="main">
      <div class="container" v-if="isDirShow">
        <create-popover-widget @change="updateDir" />
        <a-split class="split" v-model:size="splitSize" min="80px">
          <template #first>
            <file-list-widget
              :active-file="activeFile"
              :file-list="dirInfoList"
              @open-file="openFile"
            />
          </template>
          <template #resize-trigger>
            <div class="resize-trigger"></div>
          </template>
          <template #second>
            <edit-widget
              v-if="activeFile"
              ref="editViewRef"
              :info="activeFile"
              :content="activeContent"
            />
          </template>
        </a-split>
      </div>
      <div class="create" v-else>
        <a-row>
          <a-col :span="12">
            <a-space direction="vertical" fill>
              <div class="title">FreedomEdit</div>
              <div class="title-label">使用Tauri实现的跨端MD编辑器</div>
              <a-space direction="vertical" fill>
                <div class="sub-title">启动</div>
                <a-button type="text" @click="openDir">打开文件夹</a-button>
              </a-space>
            </a-space>
          </a-col>
          <a-col :span="12">
            <div class="hits">
              <div class="sub-title">提示</div>
              <a-space direction="vertical" fill>
                <p class="label">使用 Alt+P 可以快速创建文件</p>
                <p class="label">使用 Ctrl+S 可以快速保存文件</p>
              </a-space>
            </div>
          </a-col>
        </a-row>
      </div>
    </div>
  </a-config-provider>
</template>

<style scoped lang="scss">
.main {
  width: 100vw;
  height: 100vh;
  color: var(--color-text-2);
  font-size: 14px;
  background-color: var(--color-bg-1);
}
.container {
  width: 100vw;
  height: 100vh;
}
.split {
  height: 100vh;
  .resize-trigger {
    width: 1px;
    height: 100vh;
    position: relative;
    background-color: var(--color-border-1);
    &::before {
      position: absolute;
      content: "";
      top: 0;
      left: 1px;
      width: 3px;
      height: 100vh;
    }
  }
}
.create {
  padding: 30px;
  .title {
    font-size: 36px;
  }
  .title-label {
    font-size: 22px;
    padding-bottom: 80px;
  }
  .sub-title {
    font-size: 16px;
  }
  .hits {
    padding-top: 60px;
    .sub-title {
      padding-bottom: 20px;
    }
    .label {
      color: #636363;
    }
  }
}
</style>
