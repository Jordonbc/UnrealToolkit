import * as GLOBALS from '/global_variables.js';

function update_ui_elements() {
  GLOBALS.ue_directory_input_element.value = GLOBALS.ue_directory;

  if (GLOBALS.is_source) {
    GLOBALS.is_source_warning_element.style.display = "none";
  }
  else {
    GLOBALS.is_source_warning_element.style.display = "block";
  }
}

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
  });

  document.addEventListener("reload_ui", e => update_ui_elements());
});