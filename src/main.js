import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
import Antd from "ant-design-vue";
import "ant-design-vue/dist/antd.dark.css";

createApp(App).use(createPinia()).use(Antd).mount("#app");


