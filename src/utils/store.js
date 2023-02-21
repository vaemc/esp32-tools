import { defineStore } from "pinia";
export const portStore = defineStore("portStore", {
  state: () => ({
    port: "",
  }),
});

export const historyPathStore = defineStore("historyPath", {
  state: () => ({
    pathList: [],
  }),
  persist: true,
});
