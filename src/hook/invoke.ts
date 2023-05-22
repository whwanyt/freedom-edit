import { invoke } from "@tauri-apps/api";
import { FileEntry } from "@tauri-apps/api/fs";
export default function useInvokeHook() {
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
