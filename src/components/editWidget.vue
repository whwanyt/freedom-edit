<template>
  <div class="edit">
    <Editor
      ref="editorRef"
      mode="tab"
      :locale="zh"
      :plugins="plugins"
      :value="editContent"
      placeholder="请输入内容"
      @change="handleChange"
    />
  </div>
</template>

<script setup lang="ts">
// ----------
import "bytemd/dist/index.css";
import "github-markdown-css/github-markdown-light.css";
import zh from "bytemd/locales/zh_Hans.json";
import { Editor } from "@bytemd/vue-next";
import frontmatter from "@bytemd/plugin-frontmatter";
import gfm from "@bytemd/plugin-gfm";
import gemoji from "@bytemd/plugin-gemoji";
import highlight from "@bytemd/plugin-highlight";
import "highlight.js/styles/default.css";
import breaks from "@bytemd/plugin-breaks";
//-----------
import { FileEntry } from "@tauri-apps/api/fs";
import { PropType, watch } from "vue";
import { ref } from "vue";
import useInvokeHook from "../hook/invoke";
import { onMounted } from "vue";
import { useMagicKeys } from "@vueuse/core";
import { Notification } from "@arco-design/web-vue";
import savePlugin from "../utils/editPlugin";
const isEdit = ref(false);
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
const editContent = ref("");
editContent.value = props.content;

watch(
  () => editContent.value,
  () => {
    if (props.content != editContent.value) {
      isEdit.value = true;
    } else {
      isEdit.value = false;
    }
  }
);

const handleChange = (val: string) => {
  editContent.value = val;
};
const plugins = [
  gfm(),
  gemoji(),
  highlight(),
  frontmatter(),
  breaks(),
  savePlugin(() => {
    saveFile();
  }),
];
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
    Notification.success({
      content: "保存成功！",
      position: "bottomRight",
      style: { padding: "5px 10px", width: "150px", alignItems: "center" },
    });
  } catch (error) {
    console.log(error);
  }
};

const onSave = () => {};
const onClose = async () => {
  if (isEdit.value) {
    return false;
  }
  return true;
};
defineExpose({ onSave, onClose });
</script>

<style scoped lang="scss">
.edit {
  height: 100vh;
}
.edit {
  :deep(.bytemd) {
    height: 100vh;
    border: none;
    .bytemd-toolbar-right {
      div:first-child {
        display: none;
      }
    }
  }
}
</style>
