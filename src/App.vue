<template>
  <a-config-provider :locale="locale">
    <Container style="padding: 16px" />
  </a-config-provider>
</template>

<script setup>
import Container from "./components/Container.vue";
import zhCN from "ant-design-vue/es/locale/zh_CN";
import dayjs from "dayjs";
import "dayjs/locale/zh-cn";
dayjs.locale("zh-cn");

const locale = zhCN;
</script>
