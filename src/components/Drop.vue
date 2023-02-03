<template>
  <a-row :gutter="16">
    <a-col :span="12">
      <a-card title="固件合并或烧录固件">
        <div ref="mergeBinBox" class="dropBox">
          <InboxOutlined style="color: #2196f3; font-size: 50px" />
          <span style="display: block; font-size: 16px; align-self: center"
            >选择或者拖拽build目录到此</span
          >
          <span
            style="
              display: block;
              font-size: 14px;
              color: gray;
              align-self: center;
            "
            >请在执行idf.py build后再使用</span
          >
        </div>
      </a-card>
    </a-col>
    <a-col :span="12">
      <a-card title="烧录固件">
        <div ref="downloadBinBox" class="dropBox">
          <InboxOutlined style="color: #2196f3; font-size: 50px" />
          <span style="display: block; font-size: 16px; align-self: center"
            >选择或者拖拽固件到此</span
          >
          <span
            style="
              display: block;
              font-size: 14px;
              color: gray;
              align-self: center;
            "
            >只支持地址为0x0的固件</span
          >
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
import { readTextFile } from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api/tauri";

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
        let contents = await readTextFile(path + "/config/sdkconfig.json");
        let chip = JSON.parse(contents).IDF_TARGET;
        console.info(chip);

        mergeBinBox.value.style = "border: 1px dashed #434343;";
        //const command = new Command("C:/Users/vaemc/Desktop/esptool.exe --chip ESP32-C2  merge_bin -o D:/2023/rust/flash_image.bin --flash_mode dio --flash_freq 60m --flash_size 2MB 0x0 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/bootloader/bootloader.bin 0x10000 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/esp32c2_sensor_dev_board_example.bin 0x8000 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/partition_table/partition-table.bin");
        // let cmd="C:/Users/vaemc/Desktop/esptool.exe --chip ESP32-C2  merge_bin -o D:/2023/rust/flash_image.bin --flash_mode dio --flash_freq 60m --flash_size 2MB 0x0 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/bootloader/bootloader.bin 0x10000 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/esp32c2_sensor_dev_board_example.bin 0x8000 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/partition_table/partition-table.bin";
        // const command = new Command("C:/Users/vaemc/Desktop/esptool.exe", ['/C', cmd]);
        // command.spawn();

        // const command = new Command("esptool", [
        //   "--chip ESP32-C2  merge_bin -o D:/2023/rust/flash_image.bin --flash_mode dio --flash_freq 60m --flash_size 2MB 0x0 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/bootloader/bootloader.bin 0x10000 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/esp32c2_sensor_dev_board_example.bin 0x8000 D:/doit/2022/esp32c2_sensor_dev_board/esp32c2_sensor_dev_board_example/build/partition_table/partition-table.bin",
        // ]);

        // const pid = await command.spawn();

        // command.stderr.on("data", (line) => {
        //   console.info("line");
        //   console.info(line);
        // });
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

    return {
      mergeBinBox,
      downloadBinBox,
      InboxOutlined,
    };
  },

  mounted() {},
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
