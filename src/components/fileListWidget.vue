<template>
  <a-list class="list" :bordered="false" :split="false">
    <template #header>
      <a-input-search v-model="searchVal" placeholder="Serach..." />
    </template>
    <template v-if="list && list.length > 0">
      <div class="content">
        <a-list-item
          :class="activeFile.path === item.path ? 'active' : ''"
          v-for="(item, k) in list"
          @click="onItem(item, k)"
        >
          {{ item.name }}</a-list-item
        >
      </div>
    </template>
    <template #footer>
      <div class="foot">
        <a-button long @click="onQuit">退出工作区</a-button>
      </div>
    </template>
  </a-list>
</template>

<script setup lang="ts">
import { useDebounceFn } from "@vueuse/core";
import { FileEntry } from "@tauri-apps/api/fs";
import { inject, ref, watch, PropType } from "vue";
import { AppConfigProvide, appConfigKey } from "../hook/appConfig";

const props = defineProps({
  activeFile: {
    type: Object as PropType<FileEntry>,
    default: function () {
      return {};
    },
  },
  fileList: {
    type: Array<FileEntry>,
    default: function () {
      return [];
    },
  },
});

const list = ref<FileEntry[]>([]);
watch(
  () => props.fileList,
  () => {
    list.value = props.fileList;
  },
  { immediate: true }
);
const searchVal = ref("");
watch(
  () => searchVal.value,
  () => {
    if (searchVal.value.trimStart().length > 0) {
      onSearch();
    } else {
      list.value = props.fileList;
    }
  }
);
const onSearch = useDebounceFn(() => {
  list.value = list.value.filter((item) => {
    if (item.name!.includes(searchVal.value.trimStart())) {
      return item;
    }
  });
}, 600);
const emits = defineEmits(["openFile"]);
const onItem = (val: FileEntry, k: number) => {
  emits("openFile", val);
};

const { baseDirPath } = inject<AppConfigProvide>(appConfigKey)!;

const onQuit = () => {
  baseDirPath.value = "";
};
</script>

<style scoped lang="scss">
.list {
  height: 100vh;
  :deep(.arco-list-header) {
    padding: 5px !important;
  }
  .content {
    height: calc(100vh - 38px - 35px);
  }
  .arco-list-item {
    padding: 5px 20px !important;
    cursor: pointer;
    user-select: contain;
    &:hover {
      background-color: var(--color-neutral-4);
    }
  }
  .active {
    background-color: var(--color-neutral-4);
  }
  :deep(.arco-list-footer) {
    padding: 0 !important;
  }
  .foot {
    height: 35px;
    padding: 2px 5px 2px;
  }
}
</style>
