import mitt from "mitt";
const emitter = mitt();
export default emitter;

export function terminalWrite(data) {
  emitter.emit("terminal", data);
}

export function refreshFirmwareList() {
  emitter.emit("refreshFirmwareList");
}