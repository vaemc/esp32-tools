<template>
  <a-card>
    <template #title>
      <a-radio-group v-model:value="optionRadio" button-style="solid">
        <a-radio-button value="mergeBin">合并固件</a-radio-button>
        <a-tooltip>
          <template #title>烧录未合并的固件</template>
          <a-radio-button value="flash">烧录</a-radio-button>
        </a-tooltip>
        <a-tooltip>
          <template #title>烧录合并后的固件</template>
          <a-radio-button value="flashSingle">烧录单个</a-radio-button>
        </a-tooltip>
      </a-radio-group>
      <!-- <a-checkbox v-model:checked="checked">加入固件列表</a-checkbox> -->
    </template>
    <div ref="dropBox" class="dropBox">
      <InboxOutlined style="color: #2196f3; font-size: 50px" />
      <span style="display: block; font-size: 16px; align-self: center">选择或者拖拽build目录到此</span>
      <span style="
              display: block;
              font-size: 14px;
              color: gray;
              align-self: center;
            ">请在执行idf.py build后再使用</span>
    </div>
  </a-card>
</template>

<script>
import { defineComponent, ref } from "vue";
import { portStore } from '../utils/store';
import { message } from 'ant-design-vue';
import { runCmd, getFlashArgs } from "../utils/esptool"
import { InboxOutlined } from "@ant-design/icons-vue";
import { listen } from "@tauri-apps/api/event";
import emitter from "../utils/bus"
export default defineComponent({
  components: {
    InboxOutlined,
  },
  setup() {
    const dropBox = ref();
    const downloadBinBox = ref();
    const optionRadio = ref("mergeBin");
    listen("tauri://file-drop", async (event) => {
      let path = event.payload[0];
      let filename = event.payload[0].replace(/^.*[\\\/]/, "");
      const port = portStore().port
      switch (optionRadio.value) {
        case "mergeBin":
          if (filename === "build") {
            let data = await getFlashArgs(path);
            let cmd = ["--chip", data.chip, "merge_bin", "-o", "F:/2023/doit/c2/vvvvv.bin", ...data.flashArgs]
            runCmd(cmd);
          }
          break;
        case "flash":
          if (filename === "build") {
            if (port === "") {
              message.warning('请选择端口！');
              return;
            }
            let data = await getFlashArgs(path);
            let cmd = ["--chip", data.chip, "-p", port, "-b", "1152000", "--before=default_reset", "--after=hard_reset", "write_flash", ...data.flashArgs]
            runCmd(cmd);
          }
          break;
        case "flashSingle":
          if (filename.split(".").pop() === "bin") {
            let cmd = [ "-p", port,"write_flash", "0x0", path]
            runCmd(cmd);
          }

          break;
      }
      dropBox.value.style = "border: 1px dashed #434343;";
    });
    listen("tauri://file-drop-hover", (event) => {
      let path = event.payload[0];
      let filename = event.payload[0].replace(/^.*[\\\/]/, "");
      switch (optionRadio.value) {
        case "mergeBin":
          if (filename === "build") {
            dropBox.value.style = "border: 1px dashed #177ddc;";
          }
          break;
        case "flash":
          if (filename === "build") {
            dropBox.value.style = "border: 1px dashed #177ddc;";
          }
          break;
        case "flashSingle":
          if (filename.split(".").pop() === "bin") {
            dropBox.value.style = "border: 1px dashed #177ddc;";
          }
          break;
      }
    });

    listen("tauri://file-drop-cancelled", (event) => {
      dropBox.value.style = "border: 1px dashed #434343;";
    });
    return {
      dropBox,
      downloadBinBox,
      InboxOutlined,
      optionRadio
    };
  },
});
</script>

<style>
.dropBox {
  width: 100%;
  height: 150px;
  border: 1px dashed #434343;
  display: flex;
  flex-direction: column;
  justify-content: center;
  transition: all ease 0.5s;
  /* transition: 0.5s ease; */
}

.dropBox:hover {
  width: 100%;
  height: 150px;
  border: 1px dashed #177ddc;
  cursor: pointer;
  transition: all ease 1s;
}
</style>
