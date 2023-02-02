<template>
    <a-card title="烧录固件">
      <a-upload-dragger
        v-model:fileList="fileList"
        name="file"
        maxCount="1"
        :multiple="true"
        :showUploadList="false"
        action="https://www.mocky.io/v2/5cc8019d300000980a055e76"
        @change="handleChange"
        @drop="handleDrop"
      >
        <p class="ant-upload-drag-icon">
          <inbox-outlined></inbox-outlined>
        </p>
        <p class="ant-upload-text">选择或者拖拽固件到此</p>
        <p class="ant-upload-hint">
            只支持地址为0x0的固件
        </p>
      </a-upload-dragger>
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
  