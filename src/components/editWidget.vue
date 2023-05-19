<template>
  <div class="edit">
    <a-button @click="saveFile">保存</a-button>
    <Editor :locale="zh" :value="editValue" @change="handleChange" />
  </div>
</template>

<script setup lang="ts">
import "bytemd/dist/index.css";
import zh from "bytemd/locales/zh_Hans.json";
import { Editor } from "@bytemd/vue-next";
import { useThrottleFn } from "@vueuse/core";
import { FileEntry, writeTextFile } from "@tauri-apps/api/fs";
import { PropType, computed } from "vue";
import { invoke } from "@tauri-apps/api";
import { ref } from "vue";

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

const saveFile = async () => {
  try {
    console.log("保存");
    await invoke("save_text", {
      path: props.info!.path,
      content: editContent.value,
    });
    // await writeTextFile(props.info!.path, editContent.value);
    console.log(editContent.value);
  } catch (error) {
    console.log(error);
  }
};
</script>

<style scoped lang="scss">
.edit {
  :deep(.bytemd) {
    height: 100vh;
  }
}
</style>
