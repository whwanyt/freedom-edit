<template>
  <div class="edit">
    <div class="head" v-if="props.info">
      <div class="title"></div>
      <div class="funs">
        <a-button type="primary" size="small" @click="saveFile">保存</a-button>
      </div>
    </div>
    <Editor
      ref="editorRef"
      :locale="zh"
      :plugins="plugins"
      :value="editValue"
      @change="handleChange"
    />
  </div>
</template>

<script setup lang="ts">
import "bytemd/dist/index.css";
import "github-markdown-css/github-markdown-light.css";
import zh from "bytemd/locales/zh_Hans.json";
import { Editor } from "@bytemd/vue-next";
import frontmatter from "@bytemd/plugin-frontmatter";
import gfm from "@bytemd/plugin-gfm";
import gemoji from "@bytemd/plugin-gemoji";
import highlight from "@bytemd/plugin-highlight";
import "highlight.js/styles/default.css";
import { FileEntry } from "@tauri-apps/api/fs";
import breaks from "@bytemd/plugin-breaks";
import { PropType, computed, watch } from "vue";
import { ref } from "vue";
import useInvokeHook from "../hook/invoke";
import { onMounted } from "vue";
import { useMagicKeys } from "@vueuse/core";
import { Notification } from "@arco-design/web-vue";
const invokeHook = useInvokeHook();
const props = defineProps({
  info: Object as PropType<FileEntry>,
  content: {
    type: String,
    default: function () {
      return "";
    },
  },
});
const editValue = computed(() => props.content);
const editContent = ref("");
const handleChange = (val: string) => {
  editContent.value = val;
};
const plugins = [gfm(), gemoji(), highlight(), frontmatter(), breaks()];

const editorRef = ref();
onMounted(() => {
  console.log(editorRef.value);
});

const keys = useMagicKeys();
const CtrlS = keys["Ctrl+S"];
watch(CtrlS, (v) => {
  if (v) {
    saveFile();
  }
});

const saveFile = async () => {
  try {
    await invokeHook.saveFile(props.info!.path, editContent.value);
    Notification.success({ content: "保存成功！", position: "bottomRight" });
  } catch (error) {
    console.log(error);
  }
};
</script>

<style scoped lang="scss">
.edit {
  --head: 35px;
  height: 100vh;
}
.head {
  height: var(--head);
  display: flex;
  padding: 0 10px 0;
  justify-content: space-between;
  align-items: center;
}
.edit {
  :deep(.bytemd) {
    height: calc(100vh - var(--head));
    border: none;
  }
}
</style>
