const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#project_directory_input");
  greetMsgEl = document.querySelector("#greet-msg");
  document
    .querySelector("#open_project_button")
    .addEventListener("click", () => greet());
});
