<template>
  <a-card title="所有功能">
    <div style="display:flex; flex-wrap: wrap; justify-content:center">
      <a-tooltip v-for="item in customToolList" :key="item.name">
        <template #title v-if="item.toast != null">{{ item.toast }}</template>
        <a-button @click="run(item.cmd)" style="margin: 5px" type="primary">{{
          item.name
        }}</a-button>
      </a-tooltip>



    </div>
  </a-card>
</template>
<script>
import { defineComponent, ref } from "vue";
import { runCmd, generateCmd } from "../utils/esptool"
import { getCustomToolList } from "../utils/hal"
const customToolList = await getCustomToolList();
export default defineComponent({
  setup() {
    return {
      customToolList,
      run: async (data) => {
        let cmd = data;
        cmd = await generateCmd(cmd);
        runCmd(cmd);
      },
    };
  },
});
</script>
