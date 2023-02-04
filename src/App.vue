<template>
  <a-config-provider :locale="locale">
    <Container :style="{padding:'16px'}" />
  </a-config-provider>
</template>
<!-- D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/config/sdkconfig.json -->
<script setup>
import Container from "./components/Container.vue";
import zhCN from "ant-design-vue/es/locale/zh_CN";
import dayjs from "dayjs";
import "dayjs/locale/zh-cn";
dayjs.locale("zh-cn");

const locale = zhCN;

</script>
