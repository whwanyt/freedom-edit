import { invoke } from "@tauri-apps/api";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { join } from "@tauri-apps/api/path";
import { FileEntry } from "@tauri-apps/api/fs";
import { inject } from "vue";
import { AppConfigProvide, appConfigKey } from "./appConfig";

export default function useInvokeHook() {
  const { baseDirPath } = inject<AppConfigProvide>(appConfigKey)!;
  return {
    watchDir(path: string) {
      invoke("watch_dir", { path });
    },
    removewatchDir() {
      invoke("remove_watch_dir");
    },
    openLink(href: string) {
      invoke("open_link", { href });
    },
    async onDir() {
      return await invoke("on_dir");
    },
    async saveDialog() {
      return await invoke("save_dialog");
    },
    async proxyAssets(targetNode: HTMLElement) {
      const callback: MutationCallback = (vals) => {
        for (const iterator of vals) {
          if (
            iterator.target.childNodes.length > 0 &&
            // @ts-ignore
            iterator.target.childNodes[0].className === "markdown-body"
          ) {
            const images = document.querySelectorAll("img");
            images.forEach(async (item) => {
              if (item.src.includes("/assets/")) {
                const url = new URL(item.src).pathname;
                const filePath = await join(baseDirPath.value, url);
                const assetUrl = convertFileSrc(filePath);
                console.log(assetUrl);
                item.src = assetUrl;
              }
            });
          }
        }
      };
      const observer = new MutationObserver(callback);
      observer.observe(targetNode, {
        childList: true, // 观察目标子节点的变化，是否有添加或者删除
        attributes: false, // 观察属性变动
        subtree: true, // 观察后代节点，默认为 false
      });
    },
    async readDir(path: string): Promise<FileEntry[]> {
      const res: string = await invoke("read_dir", { path });
      const fileList = JSON.parse(res);
      return fileList;
    },
    async createFile(path: string): Promise<boolean> {
      return await invoke("create_file", { path });
    },
    async readFile(path: string): Promise<string> {
      let ext = path.split(".").pop()?.toLowerCase();
      if (ext && ["md", "txt"].includes(ext)) {
        return await invoke("read_text", { path });
      } else {
        return "";
      }
    },
    async saveImage(file: File) {
      return new Promise((resolve) => {
        const fileReader = new FileReader();
        fileReader.onload = async function (e) {
          const path = await join(baseDirPath.value, "./assets/", file.name);
          const base64 = fileReader
            .result!.toString()
            .replace("data:image/png;base64,", "");
          await invoke("save_image", { path, imageData: base64 });
          resolve("./assets/" + file.name);
        };
        fileReader.readAsDataURL(file);
      });
    },
    async saveFile(path: string, content: string) {
      let ext = path.split(".").pop()?.toLowerCase();
      if (ext && ["md", "txt"].includes(ext)) {
        return await invoke("save_text", { path, content });
      } else {
        return Error("加载失败");
      }
    },
  };
}
