<template>
  <a-row :gutter="16">
    <a-col :span="12">
      <a-card title="固件合并或烧录固件">
        <div ref="mergeBinBox" class="dropBox">
          <InboxOutlined style="color:#2196f3;font-size: 50px;" />
          <span style="display:block;font-size: 16px;align-self: center;">选择或者拖拽build目录到此</span>
          <span style="display:block;font-size: 14px;color: gray;align-self: center;">请在执行idf.py build后再使用</span>
        </div>
      </a-card>
    </a-col>
    <a-col :span="12">
      <a-card title="烧录固件">
        <div ref="downloadBinBox" class="dropBox">
          <InboxOutlined style="color:#2196f3;font-size: 50px;" />
          <span style="display:block;font-size: 16px;align-self: center;">选择或者拖拽固件到此</span>
          <span style="display:block;font-size: 14px;color: gray;align-self: center;">只支持地址为0x0的固件</span>
        </div>
      </a-card>
    </a-col>
  </a-row>

</template>

<script>
import { InboxOutlined } from "@ant-design/icons-vue";
import { message } from "ant-design-vue";
import { defineComponent, ref } from "vue";

import { listen } from '@tauri-apps/api/event'
import { readTextFile } from "@tauri-apps/api/fs";




export default defineComponent({
  components: {
    InboxOutlined,
  },
  setup() {

    listen("tauri://file-drop", async event => {
      let path = event.payload[0];
      let filename = event.payload[0].replace(/^.*[\\\/]/, '')
      if (filename === "build") {
        let contents = await readTextFile(
          path + "/config/sdkconfig.json"
        );
        let chip = JSON.parse(contents).IDF_TARGET;
        console.info(chip);

        mergeBinBox.value.style = "border: 1px dashed #434343;"
      }


      if (filename.split('.').pop() === "bin") {
        downloadBinBox.value.style = "border: 1px dashed #434343;"
      }


    })

    listen("tauri://file-drop-hover", event => {

      let path = event.payload[0];
      let filename = event.payload[0].replace(/^.*[\\\/]/, '')
      if (filename === "build") {
        mergeBinBox.value.style = "border: 1px dashed #177ddc;"
      }


      if (filename.split('.').pop() === "bin") {
        downloadBinBox.value.style = "border: 1px dashed #177ddc;"
      }



    })

    listen("tauri://file-drop-cancelled", event => {
      downloadBinBox.value.style = "border: 1px dashed #434343;"
      mergeBinBox.value.style = "border: 1px dashed #434343;"

    })
    const mergeBinBox = ref();
    const downloadBinBox = ref();

    return {
      mergeBinBox, downloadBinBox, InboxOutlined
    }
  },

  mounted() {

  }, methods: {

  }
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
}


.dropBox:hover {
  width: 100%;
  height: 150px;
  border: 1px dashed #177ddc;
  cursor: pointer;
}
</style>