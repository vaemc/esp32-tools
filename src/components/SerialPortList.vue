<template>
  <a-select v-model:value="selectSerialPort" @dropdownVisibleChange="serialPortListSelectFocus"
    @focus="serialPortListSelectFocus" @change="serialPortListSelectChange" style="width: 100%; margin-bottom: 16px"
    :options="serialPortList"></a-select>
</template>
<script>
import { defineComponent, ref, onMounted } from "vue";
import { portStore } from '../utils/store';
import { invoke } from "@tauri-apps/api/tauri";
export default defineComponent({
  setup() {
    const selectSerialPort = ref()
    const serialPortList = ref([]);
    const getSerialPortList = (showDefaultPort = false) => {
      invoke("get_serial_port_list").then((data) => {
        let list = JSON.parse(data);
        list = list.map(item => {
          return {
            value: item,
            label: item,
          };
        });
        serialPortList.value = list;
        if (list.length > 0 && showDefaultPort) {
          selectSerialPort.value = list[0].value;
          portStore().port = list[0].value;
        }
      });
    };
    const serialPortListSelectFocus = () => {
      getSerialPortList();
    };
    const serialPortListSelectChange = (data) => {
      portStore().port = data;
    };
    onMounted(() => {
      getSerialPortList(true);
    });
    return {
      serialPortList,
      selectSerialPort,
      serialPortListSelectFocus,
      serialPortListSelectChange
    };
  },
});
</script>
