<template>
  <a-card title="历史路径">
    <div style="height: 125px; overflow: auto" class="scroll">
      <a-popover
        v-for="item in historyPathList"
        :title="item.full"
        trigger="click"
      >
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
          item.ellipsis
        }}</a-button>
      </a-popover>
    </div>
  </a-card>
</template>
<script>
import { defineComponent, ref, onMounted } from "vue";
import { runCmd, generateCmd } from "../utils/esptool";
import { toolListConfig } from "../utils/tools-config";
import { historyPathStore } from "../utils/store";
import { openFileInExplorer } from "../utils/native";
export default defineComponent({
  setup() {
    const historyPathList = ref(historyPathStore().pathList);
    const btn = async (item, type) => {
      switch (type) {
        case "flash":
          let cmd;
          if (item.name === "build") {
            cmd = await generateCmd(toolListConfig[1].cmd, item.full);
            runCmd(cmd);
            return;
          }
          cmd = await generateCmd(toolListConfig[2].cmd, item.full);
          runCmd(cmd);
          break;
        case "open":
          if (item.name === "build") {
            openFileInExplorer(item.full);
            return;
          }
          openFileInExplorer(
            item.full.slice(0, item.full.length - item.name.length)
          );
          break;
        case "remove":
          historyPathStore().pathList = historyPathList.value.filter(
            (x) => x.full !== item.full
          );
          historyPathList.value = historyPathStore().pathList;
          break;
      }
    };
    return {
      historyPathList,
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
