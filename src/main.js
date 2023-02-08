import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import Antd from "ant-design-vue";
import "ant-design-vue/dist/antd.dark.css";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

createApp(App).use(pinia).use(Antd).mount("#app");
