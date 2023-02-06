import { invoke } from "@tauri-apps/api/tauri";
import { readTextFile, readDir, BaseDirectory } from "@tauri-apps/api/fs";

export async function getSerialPortList() {
  let data = await invoke("get_serial_port_list");
  return JSON.parse(data);
}

export async function getCurrentDir() {
  let data = await invoke("get_current_dir");
  return data;
}

export async function getToolListConfig() {
  let jsonData = JSON.parse(
    await readTextFile((await getCurrentDir()) + "\\tools.config.json")
  );
  return jsonData.toolListConfig;
}

export async function getCustomToolList() {
  let jsonData = JSON.parse(
    await readTextFile((await getCurrentDir()) + "\\tools.config.json")
  );
  return jsonData.customToolList;
}

export async function getFirmwareList() {
  let list = await readDir((await getCurrentDir()) + "\\firmware");
  list = list.map((item) => {
    return item.name;
  });
  return list;
}

export async function isEspToolExists() {
  let list = await readDir((await getCurrentDir()) + "\\esptool");
  let result = list.find((x) => x.name.includes("esptool")) != null;
  if (result) {
    return true;
  }
  return false;
}

export async function openDirInExplorer(path) {
  invoke("open_dir_in_explorer", { path: path });
}
