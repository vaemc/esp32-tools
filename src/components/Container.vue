<template>
  <div>
    <SerialPortSelect />
    <a-row :gutter="16">
      <a-col :span="15">
        <ToolList />
        <CustomToolList style="margin-top:16px;" />
      </a-col>
      <a-col :span="9">
        <FirmwareList />
      </a-col>
    </a-row>
    <Terminal />
  </div>
</template>
<script setup>
import SerialPortSelect from "./SerialPortSelect.vue";
import ToolList from "./ToolList.vue";
import CustomToolList from "./CustomToolList.vue";
import Terminal from "./Terminal.vue";
import FirmwareList from "./FirmwareList.vue";



</script>
