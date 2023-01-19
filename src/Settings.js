const { invoke } = window.__TAURI__.tauri;
const listen = window.__TAURI__.event.listen;

let ue_directory_input_element;
let ue_directory_input_button;


function openUEDirectory_Dialog() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("open_ue_directory_dialog");
  //greetMsgEl.textContent = await invoke("nameOfRustFunctionHere", { paramNameHere: paramater});
}

async function reload_variables() {
  let ue_directory = await invoke("get_ue_directory");
    ue_directory_input_element.value = ue_directory
}

window.addEventListener("DOMContentLoaded", () => {
  ue_directory_input_element = document.getElementById("ue_directory_input");
  ue_directory_input_button = document.getElementById("open_ue_directory_button");

  reload_variables();
  

  listen("ue_directory_changed", function (string) {
    ue_directory_input_element.value = string.payload;
  });
  
  ue_directory_input_button.addEventListener("click", () => openUEDirectory_Dialog());

  ue_directory_input_element.addEventListener("keypress", function(event) {
    if (event.key === "Enter" && ue_directory_input_element.value.length > 0) {
      event.preventDefault();
      invoke("set_ue_directory", { newDirectory: ue_directory_input_element.value });
    }
  })
});