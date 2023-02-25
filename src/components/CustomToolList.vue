<template>
  <a-card title="所有功能">
    <div class="scroll" style="overflow: auto">
      <div style="display: flex; flex-wrap: wrap; justify-content: center">
        <a-tooltip v-for="item in customToolList" :key="item.name">
          <template #title v-if="item.toast != null">{{ item.toast }}</template>
          <a-button size="small" @click="run(item.cmd)" style="margin: 5px">{{
            item.name
          }}</a-button>
        </a-tooltip>
        <a-tooltip>
          <template #title>读取固件</template>
          <a-button size="small" @click="readFirmware()" style="margin: 5px"
            >读取固件</a-button
          >
        </a-tooltip>
      </div>
    </div>
  </a-card>
</template>
<script>
import { defineComponent, ref } from "vue";
import { runCmd, generateCmd } from "../utils/esptool";
import { getCustomToolList } from "../utils/native";
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
      readFirmware:()=>{
        console.log('读取');
        
      }
    };
  },
});
</script>
<style>
.scroll::-webkit-scrollbar-track {
  -webkit-box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
  background-color: #141414;
}

.scroll::-webkit-scrollbar {
  width: 10px;
  background-color: #141414;
}

.scroll::-webkit-scrollbar-thumb {
  background-color: #000000;
  border: 2px solid #555555;
}
</style>
