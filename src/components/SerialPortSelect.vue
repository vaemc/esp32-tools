<template>
  <a-select v-model:value="selectSerialPort" @dropdownVisibleChange="serialPortListSelectFocus"
    @focus="serialPortListSelectFocus" @change="serialPortListSelectChange" style="width: 100%; "
    :options="serialPortList"></a-select>
</template>
<script>
import { defineComponent, ref, onMounted } from "vue";
import { portStore } from '../utils/store';
import { getSerialPortList } from '../utils/native';
export default defineComponent({
  setup() {
    const selectSerialPort = ref()
    const serialPortList = ref([]);
    const refreshSerialPortList = async (showDefaultPort = false) => {
      let list =await getSerialPortList();
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
    };
    const serialPortListSelectFocus = () => {
      refreshSerialPortList();
    };
    const serialPortListSelectChange = (data) => {
      portStore().port = data;
    };
    onMounted(() => {
      refreshSerialPortList(true);
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
