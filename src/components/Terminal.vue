<template>
    <div style="width: 100%;">
        <div id="terminal" style="height:160px;" class="xterm"></div>
    </div>
</template>
<script setup>
import emitter from "../utils/bus"
import "xterm/css/xterm.css";
import "xterm/lib/xterm.js";
import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import { onMounted } from "vue";

const terminal = new Terminal({
    fontSize: 14,
    allowProposedApi: true,
    cursorStyle: "bar",
    theme: {
        background: "#202020",
        magenta: "#e39ef7",
    },
});

emitter.on('terminal', data => {
    terminal.writeln(data);
})

onMounted(() => {
    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.open(document.getElementById("terminal"));
    fitAddon.fit();
})
</script>
<style>
/* .xterm .xterm-viewport {
  background-color: transparent;
  overflow-y: scroll;
  cursor: default;
  position: absolute;
  right: 0;
  left: 0;
  top: 0;
  bottom: 0;
  scrollbar-color: var(--highlight) var(--dark);
  scrollbar-width: thin;
}

.xterm-viewport::-webkit-scrollbar {
  background-color: var(--dark);
  width: 5px;
}

.xterm-viewport::-webkit-scrollbar-thumb {
  background: var(--highlight);
} */


.xterm-viewport::-webkit-scrollbar-track {
  -webkit-box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.3);
  background-color: #141414;
}

.xterm-viewport::-webkit-scrollbar {
  width: 10px;
  background-color: #141414;
}

.xterm-viewport::-webkit-scrollbar-thumb {
  background-color: #000000;
  border: 2px solid #555555;
}
</style>