import * as GLOBALS from '/global_variables.js';

function open_Project_Directory_Dialog() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  GLOBALS.invoke("open_project_directory_dialog");
  //greetMsgEl.textContent = await invoke("nameOfRustFunctionHere", { paramNameHere: paramater});
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("Running home.js");
  GLOBALS.project_directory_input_button.addEventListener("click", () => open_Project_Directory_Dialog());

  GLOBALS.project_directory_input_element.addEventListener("keypress", function(event) {
    if (event.key === "Enter" && GLOBALS.project_directory_input_element.value.length > 0) {
      event.preventDefault();
      GLOBALS.invoke("set_project_directory", { newDirectory: GLOBALS.project_directory_input_element.value });
    }
  })
});