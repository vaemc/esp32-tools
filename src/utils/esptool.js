import { Command } from "@tauri-apps/api/shell";
import isHexPrefixed from "is-hex-prefixed";
import { exists, readTextFile } from "@tauri-apps/api/fs";
import { terminalWrite, refreshFirmwareList } from "./bus";
import { message } from "ant-design-vue";
import { portStore } from "../utils/store";
import {
  getCurrentDir,
  isEspToolExists,
  openFileInExplorer,
} from "../utils/native";
import { notification, Button } from "ant-design-vue";
import { h } from "vue";

import kleur from "kleur";

const currentDir = await getCurrentDir();
export const buildDirectory = {
  flash_args: "\\flash_args",
  flash_app_args: "\\flash_app_args",
  sdkconfig: "\\config\\sdkconfig.json",
};

export async function generateCmd(data, path = "") {
  const port = portStore().port;

  let cmd = data;
  if (cmd.find((x) => x === "${port}") != null) {
    if (port === "") {
      message.warning("请选择端口！");
      return;
    }
  }
  let isIncludeFlashArgs = cmd.find((x) => x === "${flashArgs}") != null;
  let appInfo;
  if (isIncludeFlashArgs) {
    appInfo = await getFlashArgs(path);
  }
  cmd = cmd.map((item) => {
    if (item === "${chip}") {
      return appInfo.chip;
    }
    switch (item) {
      case "${chip}":
        return appInfo.chip;
      case "${appName}":
        return currentDir + "\\firmware\\" + appInfo.appName;
      case "${port}":
        return port;
      case "${path}":
        return path;
    }
    return item;
  });
  if (isIncludeFlashArgs) {
    cmd.splice(cmd.indexOf("${flashArgs}"), 0, ...appInfo.flashArgs);
    cmd = cmd.filter((x) => x != "${flashArgs}");
  }

  return cmd;
}

export async function getFlashArgs(path) {
  const filePathList = Object.keys(buildDirectory).map(
    (key) => buildDirectory[key]
  );
  let filePathListCheckResult = await Promise.all(
    filePathList.map(async (item) => {
      let result = await exists(path + item);
      if (!result) {
        terminalWrite(kleur.blue().bold(`"${path}${item}" file not found`));
      }
      return result;
    })
  );
  if (filePathListCheckResult.filter((x) => x).length != filePathList.length) {
    return false;
  }

  let sdkconfig = await readTextFile(path + buildDirectory.sdkconfig);
  let chip = JSON.parse(sdkconfig).IDF_TARGET;
  let flashArgsText = await readTextFile(path + buildDirectory.flash_args);
  let flashArgs = [];
  flashArgsText.split("\n").map((item) => {
    let line = item.split(" ");
    if (!isHexPrefixed(line[0])) {
      flashArgs.push(...line);
    } else {
      flashArgs.push(line[0]);
      flashArgs.push(path + "\\" + line[1]);
    }
  });

  let flashAppArgs = await readTextFile(path + buildDirectory.flash_app_args);
  let appName = flashAppArgs.split("\n")[1].split(" ")[1];

  const data = {
    appName: appName,
    chip: chip,
    flashArgs: flashArgs,
  };
  return data;
}

export async function runCmd(cmd) {
  let result = await isEspToolExists();
  if (!result) {
    notification.open({
      message: "未检测到esptool工具",
      description: "请将esptool工具放在软件根目录esptool文件夹！",
      btn: () =>
        h(
          Button,
          {
            type: "primary",
            size: "small",
            onClick: () => {
              openFileInExplorer(currentDir + "\\esptool");
            },
          },
          {
            default: () => "打开文件夹",
          }
        ),
    });

    return;
  }

  if (cmd.find((x) => x === "merge_bin") != null) {
    openFileInExplorer(currentDir + "\\firmware");
  }

  cmd = cmd.filter((x) => x != "");
  const command = new Command("esptool", cmd);
  command.on("close", (data) => {});
  command.on("error", (error) => terminalWrite(error));
  command.stdout.on("data", (line) => terminalWrite(line));
  command.stderr.on("data", (line) => terminalWrite(line));
  const child = await command.spawn();

  await new Promise((r) => setTimeout(r, 2500));
  await refreshFirmwareList();
  //console.log("pid:", child.pid);
}
