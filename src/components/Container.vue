<template>
  <div>
    <PortSelect />
    <a-row :gutter="16">
      <a-col :span="18">
        <Drop />
        <ToolList style="margin-top:16px;" />
      </a-col>
      <a-col :span="6">
        <FirmwareList />
      </a-col>
    </a-row>
    <Console />
  </div>
</template>
<script setup>
import PortSelect from "./PortSelect.vue";
import Drop from "./Drop.vue";
import ToolList from "./ToolList.vue";
import Console from "./Console.vue";
import FirmwareList from "./FirmwareList.vue";
</script>
