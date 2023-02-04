import { Command } from "@tauri-apps/api/shell";
import isHexPrefixed from "is-hex-prefixed";
import { exists, readTextFile } from "@tauri-apps/api/fs";
import { terminalWrite } from "./bus";
import chalk from "chalk";

export async function getFlashArgs(path) {
  if (!(await exists(path + "/flash_args"))) {
    terminalWrite(chalk.magenta.bold(`"${path}\\flash_args" file not found`));
    return;
  }
  if (!(await exists(path + "/config/sdkconfig.json"))) {
    terminalWrite(
      chalk.magenta.bold(`"${path}\\config\\sdkconfig.json" file not found`)
    );
    return;
  }
  let sdkconfig = await readTextFile(path + "/config/sdkconfig.json");
  let chip = JSON.parse(sdkconfig).IDF_TARGET;
  let flashArgsText = await readTextFile(path + "/flash_args");
  let flashArgs = [];
  flashArgsText.split("\n").map((item) => {
    let line = item.split(" ");
    if (!isHexPrefixed(line[0])) {
      flashArgs.push(...line);
    } else {
      flashArgs.push(line[0]);
      flashArgs.push(path + "/" + line[1]);
    }
  });

  const data = {
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
