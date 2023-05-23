import { BytemdPlugin } from "bytemd";
export default function savePlugin(callback: any): BytemdPlugin {
  return {
    actions: [
      {
        title: "保存",
        position: "right",
        icon: `<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" stroke="currentColor" class="arco-icon arco-icon-save" stroke-width="4" stroke-linecap="butt" stroke-linejoin="miter" data-v-c1768fa9=""><path d="M21 13v9m18 20H9a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h22.55a1 1 0 0 1 .748.336l7.45 8.38a1 1 0 0 1 .252.664V41a1 1 0 0 1-1 1ZM14 6h14v15a1 1 0 0 1-1 1H15a1 1 0 0 1-1-1V6Z"></path></svg>`,
        handler: {
          type: "action",
          click: callback,
        },
      },
    ],
  };
}
