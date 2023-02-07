<template>
  <div>
    <SerialPortSelect />
    <a-row :gutter="10">
      <a-col :span="15" >
        <ToolList />
        <FirmwareList  style="margin-top: 10px;"  />
      </a-col>
      <a-col :span="9">
        <CustomToolList/>
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
