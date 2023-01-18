const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function openUEDirectory_Dialog() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("open_ue_directory_dialog");
  //greetMsgEl.textContent = await invoke("nameOfRustFunctionHere", { paramNameHere: paramater});
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#project_directory_input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#open_ue_directory_button")
    .addEventListener("click", () => openUEDirectory_Dialog());
});
