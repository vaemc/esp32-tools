import { defineStore } from "pinia";
export const portStore = defineStore("port", {
  state: () => ({
    port: "",
  })
});
