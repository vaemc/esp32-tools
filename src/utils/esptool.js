import { Command } from "@tauri-apps/api/shell";
import isHexPrefixed from "is-hex-prefixed";
import { exists, readTextFile } from "@tauri-apps/api/fs";
import { terminalWrite } from "./bus";

import kleur from "kleur";

export const buildDirectory = {
  flash_args: "\\flash_args",
  flash_app_args: "\\flash_app_args",
  sdkconfig: "\\config\\sdkconfig.json",
};

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
  cmd = cmd.filter((x) => x != "");
  const command = new Command("esptool", cmd);
  command.on("close", (data) => {});
  command.on("error", (error) => terminalWrite(error));
  command.stdout.on("data", (line) => terminalWrite(line));
  command.stderr.on("data", (line) => terminalWrite(line));
  const child = await command.spawn();
  //console.log("pid:", child.pid);
}
