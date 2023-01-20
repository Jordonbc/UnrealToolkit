import * as GLOBALS from '/global_variables.js';

function openUEDirectory_Dialog() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  GLOBALS.invoke("open_ue_directory_dialog");
  //greetMsgEl.textContent = await invoke("nameOfRustFunctionHere", { paramNameHere: paramater});
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("Running Settings.js");
  GLOBALS.ue_directory_input_button.addEventListener("click", () => openUEDirectory_Dialog());

  GLOBALS.ue_directory_input_element.addEventListener("keypress", function(event) {
    if (event.key === "Enter" && GLOBALS.ue_directory_input_element.value.length > 0) {
      event.preventDefault();
      GLOBALS.invoke("set_ue_directory", { newDirectory: GLOBALS.ue_directory_input_element.value });
    }
  })
});