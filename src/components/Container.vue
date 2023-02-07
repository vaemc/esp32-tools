<!-- <template>
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
</script> -->

<template>
  <div class="box">
    <div class="row header">
      <SerialPortSelect />
    </div>
    <div class="row content">
      <div style="display: flex; flex-direction: row">
        <div style="flex: 2; margin-right: 5px">
          <ToolList />
          <FirmwareList style="margin-top: 10px;" />
        </div>
        <div style="flex: 1; margin-left: 5px;">
          <CustomToolList />
        </div>
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
