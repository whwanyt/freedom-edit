<script setup lang="ts">
import { invoke } from "@tauri-apps/api";
import { FileEntry, readDir } from "@tauri-apps/api/fs";
import zhCN from "@arco-design/web-vue/es/locale/lang/zh-cn";
import { listen } from "@tauri-apps/api/event";
import { ref } from "vue";
import FileListWidget from "./components/fileListWidget.vue";
import EditWidget from "./components/editWidget.vue";
import { useLocalStorage } from "@vueuse/core";
import { watch } from "vue";

const dirInfoList = ref<FileEntry[]>([]);

const basePath = useLocalStorage("dir", "");
async function init() {
  if (basePath.value.length > 0) {
  }
}

listen("watch", (val) => {
  console.log(val);
});

watch(
  () => basePath.value,
  () => {
    if (basePath.value.length > 0) {
    }
  },
  { immediate: true }
);
init();
const openDir = async () => {
  const dir = await invoke("on_dir");
  console.log(dir);
  basePath.value = dir as string;
  if (!dir) return;
};

const activeFile = ref<FileEntry>();
const activeContent = ref("");
const openFile = async (val: FileEntry) => {
  const content: string = await invoke("read_text", { path: val.path });
  activeFile.value = val;
  activeContent.value = content;
};

const splitSize = ref(0.3);
</script>

<template>
  <a-config-provider :locale="zhCN">
    <div class="container" v-if="dirInfoList.length > 0">
      <a-split class="split" v-model:size="splitSize" min="80px">
        <template #first>
          <FileListWidget :file-list="dirInfoList" @open-file="openFile" />
        </template>
        <template #second>
          <EditWidget :info="activeFile" :content="activeContent" />
        </template>
      </a-split>
    </div>
    <a-result v-else>
      <template #icon>
        <IconFaceSmileFill />
      </template>
      <template #title>暂无内容</template>
      <template #subtitle>请选择文件夹做完写作目录</template>
      <template #extra>
        <a-space>
          <a-button type="text" @click="openDir">打开文件夹</a-button>
        </a-space>
      </template>
    </a-result>
  </a-config-provider>
</template>

<style scoped>
.container {
  width: 100vw;
  height: 100vh;
}
.split {
  height: 100vh;
}
</style>
