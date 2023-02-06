<template>
  <a-card title="固件列表" style="height: 100%">
    <div style="height: 300px; overflow: auto" class="scroll">
      <a-button
        type="dashed"
        size="small"
        v-for="item in firmwareList"
        style="margin: 3px"
        >{{ item }}</a-button
      >
    </div>
  </a-card>
</template>
<script>
import { defineComponent, ref, onMounted } from "vue";

import { getFirmwareList } from "../utils/hal";
const firmwareList = await getFirmwareList();
console.info(firmwareList);
export default defineComponent({
  setup() {
    return {
      firmwareList,
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
