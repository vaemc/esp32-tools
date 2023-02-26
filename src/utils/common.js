import { invoke } from "@tauri-apps/api/tauri";
import { readTextFile, readDir, removeFile as rf } from "@tauri-apps/api/fs";
import { save } from "@tauri-apps/api/dialog";

export async function saveFileDialog() {
  const filePath = await save({
    filters: [
      {
        name: "Bin",
        extensions: ["bin"],
      },
    ],
  });

  return filePath;
}
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

export async function openFileInExplorer(path) {
  invoke("open_file_in_explorer", { path: path });
}

export async function removeFile(path) {
  await rf(path);
}
