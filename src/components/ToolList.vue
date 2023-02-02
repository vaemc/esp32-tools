<template>
  <a-card title="所有功能">
    <div>
      <a-button style="margin: 5px" v-for="n in 10" type="primary"
        >擦除固件</a-button
      >
    </div>
  </a-card>
</template>
<script>
import { InboxOutlined } from "@ant-design/icons-vue";
import { message } from "ant-design-vue";
import { defineComponent, ref } from "vue";
export default defineComponent({
  components: {
    InboxOutlined,
  },
  setup() {
    const handleChange = (info) => {
      const status = info.file.status;
      if (status !== "uploading") {
        console.log(info.file, info.fileList);
      }
      if (status === "done") {
        message.success(`${info.file.name} file uploaded successfully.`);
      } else if (status === "error") {
        message.error(`${info.file.name} file upload failed.`);
      }
    };
    return {
      handleChange,
      fileList: ref([]),
      handleDrop: (e) => {
        console.log(e);
      },
    };
  },
});
</script>
