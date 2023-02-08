<template>
  <div class="box">
    <div class="row header">
      <SerialPortSelect />
    </div>
    <div class="row content">
      <div style="display: flex; flex-direction: row">
        <ToolList style="flex: 1; margin-right: 5px" />
        <CustomToolList style="flex: 1; margin-left: 5px" />
      </div>
      <div style="display: flex; flex-direction: row; margin-top: 10px">
        <HistoryPath style="flex: 1; margin-right: 5px" />
        <FirmwareList style="flex: 1; margin-left: 5px" />
      </div>
    </div>
    <div class="row footer">
      <Terminal />
    </div>
  </div>
</template>
<script setup>
import SerialPortSelect from "./SerialPortSelect.vue";
import ToolList from "./ToolList.vue";
import CustomToolList from "./CustomToolList.vue";
import Terminal from "./Terminal.vue";
import FirmwareList from "./FirmwareList.vue";
import HistoryPath from "./HistoryPath.vue";
</script>
<style>
.box {
  display: flex;
  flex-flow: column;
  height: 100%;
}

.box .row.header {
  flex: 0 1 auto;
  padding: 10px 10px 0 10px;
}
.box .row.content {
  flex: 1 1 auto;
  padding: 10px 10px 0 10px;
}
.box .row.footer {
  flex: 0 1 auto;
}
</style>
