import { createApp } from "vue";
import "./styles.css";
import ArcoVue from "@arco-design/web-vue";
import App from "./App.vue";
import ArcoVueIcon from "@arco-design/web-vue/es/icon";
import "@arco-design/web-vue/dist/arco.css";

const app = createApp(App);
app.use(ArcoVue);
app.use(ArcoVueIcon);
app.mount("#app");
