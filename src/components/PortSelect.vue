<template>
  <a-select ref="select" v-model="serialPortListDefaultSelectPath" @dropdownVisibleChange="serialPortListSelectFocus"
    @focus="serialPortListSelectFocus" @change="serialPortListSelectChange" style="width: 100%; margin-bottom: 16px">
    <a-select-option v-for="item in serialPortList" :value="item">{{
      item
    }}</a-select-option>
  </a-select>
</template>
<script>
import { defineComponent, ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export default defineComponent({
  setup() {
    let serialPortList = ref([]);
    const getSerialPortList = (isShowDefaultPath = false) => {
      invoke("get_serial_port_list").then((message) => {
        serialPortList.value = JSON.parse(message);
        console.info(serialPortList);
      });
    };
    const serialPortListSelectFocus = () => {
      getSerialPortList();
    };
    const serialPortListSelectChange = (value) => {
      console.info(value);
    };

   
    return {
      serialPortListSelectFocus,
      serialPortListSelectChange,
      serialPortListDefaultSelectPath: "",
      serialPortList,
    };
  },


});
</script>
