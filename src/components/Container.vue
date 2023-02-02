<template>
  <div>
    <a-row :gutter="16">
      <a-col :span="12">
        <DownloadFirmware />
      </a-col>
      <a-col :span="12">
        <MergeBin />
      </a-col>
    </a-row>

    <ToolList />
  </div>
</template>
<script setup>
import MergeBin from "./MergeBin.vue";
import ToolList from "./ToolList.vue";
import DownloadFirmware from "./DownloadFirmware.vue";
</script>
