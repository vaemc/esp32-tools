<template>
  <div>
    <SerialPortList />
    <a-row :gutter="16">
      <a-col :span="18">
        <Tools />
        <CustomToolList style="margin-top:16px;" />
      </a-col>
      <a-col :span="6">
        <FirmwareList />
      </a-col>
    </a-row>
    <Terminal />
  </div>
</template>
<script setup>
import SerialPortList from "./SerialPortList.vue";
import Tools from "./Tools.vue";
import CustomToolList from "./CustomToolList.vue";
import Terminal from "./Terminal.vue";
import FirmwareList from "./FirmwareList.vue";
</script>
