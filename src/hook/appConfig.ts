import { RemovableRef, useLocalStorage } from "@vueuse/core";

export type AppConfigProvide = {
  baseDirPath: RemovableRef<string>;
};
export const appConfigKey = Symbol();
const baseDirPath = useLocalStorage("baseDirPath", "");
const appConfig = { baseDirPath };
export default appConfig;
