<template>
  <a-card>
    <template #title>
      <a-radio-group
        v-model:value="toolsRadio"
        @change="toolsRadioChange"
        button-style="solid"
        size="small"
      >
        <a-tooltip v-for="item in toolListConfig" :key="item.name">
          <template #title v-if="item.toast != null">{{ item.toast }}</template>
          <a-radio-button :value="item.name">{{ item.name }}</a-radio-button>
        </a-tooltip>
      </a-radio-group>
    </template>
    <div style="height: 75px;" :class="dropBoxClass" @click="openFileDialog">
      <!-- <InboxOutlined style="color: #2196f3; font-size: 40px" /> -->
      <span style="display: block; font-size: 16px; align-self: center">{{
        toolsRadioSelect.dropDesc
      }}</span>
      <span
        style="display: block; font-size: 14px; color: gray; align-self: center"
        >{{ toolsRadioSelect.dropHelp }}</span
      >
    </div>
  </a-card>
</template>

<script>
import { defineComponent, ref, onMounted } from "vue";
import { message } from "ant-design-vue";
import { runCmd, generateCmd } from "../utils/esptool";
import { InboxOutlined } from "@ant-design/icons-vue";
import { listen } from "@tauri-apps/api/event";
import { getToolListConfig } from "../utils/common";
import { open } from "@tauri-apps/api/dialog";
import { historyPathStore } from "../utils/store";

const toolListConfig = await getToolListConfig();

function historyPathSave(data) {
  let path = data.split("\\");
  let result;
  if (path.length >= 5) {
    let temp = `${path[0]}\\${path[1]}\\${path[2]}\\...\\${
      path[path.length - 2]
    }\\${path[path.length - 1]}`;
    result = { full: data, ellipsis: temp, name: path[path.length - 1] };
  } else {
    result = { full: data, ellipsis: data, name: path[path.length - 1] };
  }
  let historyPathList = historyPathStore().pathList;
  if (historyPathList.filter((x) => x.full === result.full).length == 0) {
    historyPathStore().pathList.push(result);
  }
}
export default defineComponent({
  components: {
    InboxOutlined,
  },
  setup() {
    const dropBoxClass = ref("dropBox");
    const toolsRadioSelect = ref({
      dropDesc: "??????????????????build????????????",
      dropHelp: "????????????idf.py build????????????",
    });
    const toolsRadio = ref();
    //const toolListConfig = ref();

    listen("tauri://file-drop", async (event) => {
      let path = event.payload[0];
      let filename = path.replace(/^.*[\\\/]/, "");
      const re = new RegExp(toolsRadioSelect.value.dropRegex);
      if (!re.test(filename)) {
        message.warning("???????????????????????????");
        return;
      }
      historyPathSave(path);
      // historyPathStore().pathList.push(historyPathGenerate(path));
      let cmd = toolsRadioSelect.value.cmd;
      cmd = await generateCmd(cmd, path);
      runCmd(cmd);
      dropBoxClass.value = "dropBox";
    });
    listen("tauri://file-drop-hover", (event) => {
      let path = event.payload[0];
      let filename = path.replace(/^.*[\\\/]/, "");
      const re = new RegExp(toolsRadioSelect.value.dropRegex);
      if (re.test(filename)) {
        dropBoxClass.value = "dropBoxHover";
      }
    });

    listen("tauri://file-drop-cancelled", (event) => {
      dropBoxClass.value = "dropBox";
    });

    const toolsRadioChange = (data) => {
      toolsRadioSelect.value = toolListConfig.find(
        (x) => x.name === data.target.value
      );
    };
    const openFileDialog = async () => {
      const selected = await open({
        directory: toolsRadioSelect.value.isDirectory,
        multiple: false,
      });
      if (!Array.isArray(selected) && selected !== null) {
        let dirNmae = selected.split("\\");
        dirNmae = dirNmae[dirNmae.length - 1];
        const re = new RegExp(toolsRadioSelect.value.dropRegex);
        if (!re.test(dirNmae)) {
          message.warning("??????????????????????????????");
          return;
        }
        historyPathSave(selected);
        //  historyPathStore().pathList.push(historyPathGenerate(selected));
        let cmd = toolsRadioSelect.value.cmd;
        cmd = await generateCmd(cmd, selected);
        runCmd(cmd);
      }
    };
    onMounted(async () => {
      if (toolListConfig.length > 0) {
        toolsRadio.value = toolListConfig[0].name;
        toolsRadioSelect.value = toolListConfig[0];
      }
    });
    return {
      dropBoxClass,
      InboxOutlined,
      toolsRadio,
      toolListConfig,
      toolsRadioChange,
      toolsRadioSelect,
      openFileDialog,
    };
  },
});
</script>

<style>
.dropBox {
  width: 100%;
  height: 120px;
  border: 1px dashed #434343;
  display: flex;
  flex-direction: column;
  justify-content: center;
  transition: all ease 0.5s;
  /* transition: 0.5s ease; */
}

.dropBox:hover {
  width: 100%;
  height: 120px;
  border: 1px dashed #177ddc;
  cursor: pointer;
  transition: all ease 1s;
}

.dropBoxHover {
  width: 100%;
  height: 120px;
  border: 1px dashed #177ddc;
  display: flex;
  flex-direction: column;
  justify-content: center;
  transition: all ease 1s;
}
</style>
