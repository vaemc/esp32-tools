<template>
  <a-card title="固件列表" style="height: 100%">
    <div style="height: 300px; overflow: auto" class="scroll">
      <a-popover v-for="item in firmwareList" :title="item" trigger="click">
        <template #content>
          <a-button style="margin: 3px" @click="btn(item, 'flash')"
            >烧录</a-button
          >
          <a-button style="margin: 3px" @click="btn(item, 'open')" primary
            >打开</a-button
          >
          <a-button style="margin: 3px" @click="btn(item, 'remove')" danger
            >删除</a-button
          >
        </template>
        <a-button type="dashed" size="small" style="margin: 3px">{{
          item
        }}</a-button>
      </a-popover>
    </div>
  </a-card>
</template>
<script>
import { defineComponent, ref, onMounted } from "vue";
import { runCmd, generateCmd } from "../utils/esptool";
import { toolListConfig } from "../utils/tools-config";
import {
  getFirmwareList,
  openFileInExplorer,
  getCurrentDir,
} from "../utils/hal";
import emitter from "../utils/bus";
// openFileInExplorer(currentDir + "\\firmware");
const firmwareList = await getFirmwareList();
const currentDir = await getCurrentDir();
export default defineComponent({
  setup() {
    emitter.on("refreshFirmwareList", async (data) => {
      let list = await getFirmwareList();
      firmwareList.value = await getFirmwareList();
    });
    const btn = async (item, type) => {
      let path = currentDir + "\\firmware\\" + item;
      switch (type) {
        case "flash":
          let cmd = await generateCmd(toolListConfig[2].cmd, path);
          runCmd(cmd);
          break;
        case "open":
          openFileInExplorer(currentDir + "\\firmware");
          break;
        case "remove":
          //removeFile(currentDir + "\\firmware\\" + item);
          break;
      }
    };
    return {
      firmwareList,
      btn,
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
