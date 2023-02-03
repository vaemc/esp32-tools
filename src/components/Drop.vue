<template>
  <a-row :gutter="16">
    <a-col :span="12">
      <a-card title="合并固件或烧录固件">
        <div ref="mergeBinBox" class="dropBox">
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
    </a-col>
    <a-col :span="12">
      <a-card title="烧录固件">
        <div ref="downloadBinBox" class="dropBox">
          <InboxOutlined style="color: #2196f3; font-size: 50px" />
          <span style="display: block; font-size: 16px; align-self: center">选择或者拖拽固件到此</span>
          <span style="
              display: block;
              font-size: 14px;
              color: gray;
              align-self: center;
            ">只支持地址为0x0的固件</span>
        </div>
      </a-card>
    </a-col>
  </a-row>
</template>

<script>
import { defineComponent, ref } from "vue";
import { InboxOutlined } from "@ant-design/icons-vue";
import { Command } from "@tauri-apps/api/shell";
import { listen } from "@tauri-apps/api/event";
import { exists, readTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";

import isHexPrefixed from "is-hex-prefixed"
async function verifyFlashArgs(path) {

  if (!exists(path + "/flash_args")) {
    return;
  }
  if (!exists(path + "/config/sdkconfig.json")) {
    return;
  }
  let flashArgs = await readTextFile(path + "/flash_args");
  flashArgs.split("\n").map(item => {
    if (isHexPrefixed(item)) {
      if (!exists(path + "/" + item.split(" ")[1])) {
        return;
      }
    }
  })


  let sdkconfig = await readTextFile(path + "/config/sdkconfig.json");

  let chip = JSON.parse(sdkconfig).IDF_TARGET;




  // console.log(isHexPrefixed('0x..'));

  // console.info(flashArgs.split(" "));

  // console.info(chip);
  console.info(flashArgs);
}


function writeFlash() {

}

export default defineComponent({
  components: {
    InboxOutlined,
  },
  setup() {
    const mergeBinBox = ref();
    const downloadBinBox = ref();
    listen("tauri://file-drop", async (event) => {
      let path = event.payload[0];
      let filename = event.payload[0].replace(/^.*[\\\/]/, "");
      if (filename === "build") {
        verifyFlashArgs(path);
        mergeBinBox.value.style = "border: 1px dashed #434343;";
      }

      if (filename.split(".").pop() === "bin") {
        downloadBinBox.value.style = "border: 1px dashed #434343;";
      }
    });

    listen("tauri://file-drop-hover", (event) => {
      let path = event.payload[0];
      let filename = event.payload[0].replace(/^.*[\\\/]/, "");
      if (filename === "build") {
        mergeBinBox.value.style = "border: 1px dashed #177ddc;";
      }

      if (filename.split(".").pop() === "bin") {
        downloadBinBox.value.style = "border: 1px dashed #177ddc;";
      }
    });

    listen("tauri://file-drop-cancelled", (event) => {
      downloadBinBox.value.style = "border: 1px dashed #434343;";
      mergeBinBox.value.style = "border: 1px dashed #434343;";
    });

    const abc = async () => {
      console.info("asdasd")
      const command = new Command('esptool', [
        "--chip",
        "ESP32-C2",
        "merge_bin",
        "-o",
        "F:/2023/doit/c2/flash_image.bin",
        "--flash_mode",
        "dio",
        "--flash_freq",
        "60m",
        "--flash_size",
        "2MB",
        "0x0",
        "F:/2023/doit/c2/esp32c2_sensor_dev_board_idf/build/bootloader/bootloader.bin",
        "0x10000",
        "F:/2023/doit/c2/esp32c2_sensor_dev_board_idf/build/esp32c2_sensor_dev_board_idf.bin",
        "0x8000",
        "F:/2023/doit/c2/esp32c2_sensor_dev_board_idf/build/partition_table/partition-table.bin",
      ])

      command.on('close', data => {
        console.log(`执行完成`)
      });
      command.on('error', error => console.error(`command error: "${error}"`));
      command.stdout.on('data', line => console.log(line));
      command.stderr.on('data', line => console.log(`command stderr: "${line}"`));

      const child = await command.spawn();
      console.log('pid:', child.pid);

    }
    return {
      mergeBinBox,
      downloadBinBox,
      InboxOutlined, abc
    };
  },

  mounted() { },
  methods: {},
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
