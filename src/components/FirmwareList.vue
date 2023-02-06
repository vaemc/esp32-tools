<template>
  <a-card title="固件列表" style="height: 100%">
    <div style="height: 300px; overflow: auto" class="scroll">
      <a-popover v-for="item in firmwareList" :title="item">
        <template #content>
          <a-button style="margin: 3px">烧录</a-button>

          <a-button style="margin: 3px" primary>打开</a-button>
          <a-button style="margin: 3px" danger>删除</a-button>
        </template>
        <a-button type="dashed" size="small" style="margin: 3px">{{ item }}</a-button>
      </a-popover>


    </div>
  </a-card>
</template>
<script>
import { defineComponent, ref, onMounted } from "vue";
import { getFirmwareList } from "../utils/hal";
import emitter from "../utils/bus"

const firmwareList = await getFirmwareList();
export default defineComponent({
  setup() {

    emitter.on('refreshFirmwareList', async data => {
      let list = await getFirmwareList();
      firmwareList.value = await getFirmwareList();
    })


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
