<template>
  <a-list class="list">
    <template #header>
      <div class="header">Title</div>
    </template>
    <template v-if="props.fileList && props.fileList.length > 0">
      <a-list-item
        :class="activeKey === k ? 'active' : ''"
        v-for="(item, k) in props.fileList"
        @click="onItem(item, k)"
      >
        {{ item.name }}</a-list-item
      >
    </template>
  </a-list>
</template>

<script setup lang="ts">
import { FileEntry } from "@tauri-apps/api/fs";
import { ref } from "vue";

const props = defineProps({
  fileList: Array<FileEntry>,
});

const activeKey = ref(-1);
const emits = defineEmits(["openFile"]);
const onItem = (val: FileEntry, k: number) => {
  activeKey.value = k;
  emits("openFile", val);
};
</script>

<style scoped lang="scss">
.list {
  height: 100vh;
  :deep(.arco-list-header) {
    padding: 5px 20px !important;
  }
  .arco-list-item {
    padding: 5px 20px !important;
    cursor: pointer;
    &:hover {
      background-color: var(--color-neutral-4);
    }
  }
  .active {
    background-color: var(--color-neutral-4);
  }
}
</style>
