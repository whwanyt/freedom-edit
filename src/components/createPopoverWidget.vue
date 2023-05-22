<template>
  <div class="create-popover" id="createPopover">
    <a-modal
      class="modal"
      v-model:visible="isCreatePopover"
      :footer="false"
      popup-container="#createPopover"
    >
      <a-input
        v-model="nameVal"
        size="large"
        ref="createPopoverRef"
        placeholder="请输入文件名"
        allow-clear
        focus
      />
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { onKeyStroke, useMagicKeys } from "@vueuse/core";
import { inject, nextTick, ref, watch } from "vue";
import { Notification } from "@arco-design/web-vue";
import { AppConfigProvide, appConfigKey } from "../hook/appConfig";
import useInvokeHook from "../hook/invoke";

const { baseDirPath } = inject<AppConfigProvide>(appConfigKey)!;
const nameVal = ref("");
const isCreatePopover = ref(false);

const keys = useMagicKeys();
const AltP = keys["Alt+P"];
const createPopoverRef = ref();
watch(AltP, (v) => {
  if (v) {
    isCreatePopover.value = true;
    nextTick(() => {
      createPopoverRef.value.focus();
    });
  }
});

const invokeHook = useInvokeHook();
onKeyStroke("Enter", async (e) => {
  e.preventDefault();
  if (isCreatePopover.value) {
    if (nameVal.value.trimStart().length > 0) {
      const fileStatus = await invokeHook.createFile(
        baseDirPath.value + "/" + nameVal.value + ".md"
      );
      console.log(fileStatus);
    } else {
      Notification.info({ title: "提示", content: "请输入有效文件名" });
    }
    isCreatePopover.value = false;
  }
});
</script>

<style scoped lang="scss">
:deep(.modal) {
  .arco-modal {
    background-color: transparent;
    .arco-modal-header {
      display: none;
    }
    .arco-modal-body {
      padding: 10px;
      .arco-input-wrapper {
        border-radius: 6px;
      }
    }
  }
}
</style>
