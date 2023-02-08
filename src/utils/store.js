import { defineStore } from "pinia";
export const portStore = defineStore("portStore", {
  state: () => ({
    port: "",
  }),
});

export const historyPathStore = defineStore("HistoryPath", {
  state: () => ({
    pathList: [],
  }),
  persist: true,
});
