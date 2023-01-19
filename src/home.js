const { invoke } = window.__TAURI__.tauri;
const listen = window.__TAURI__.event.listen;

let project_directory_input_element;
let project_directory_input_button;


function open_Project_Directory_Dialog() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("open_project_directory_dialog");
  //greetMsgEl.textContent = await invoke("nameOfRustFunctionHere", { paramNameHere: paramater});
}

async function reload_variables() {
  let project_directory = await invoke("get_project_directory");
    project_directory_input_element.value = project_directory;
}

window.addEventListener("DOMContentLoaded", () => {
  project_directory_input_element = document.getElementById("project_directory_input");
  project_directory_input_button = document.getElementById("open_project_directory_button");

  reload_variables();
  

  listen("project_directory_changed", function (string) {
    project_directory_input_element.value = string.payload;
  });
  
  project_directory_input_button.addEventListener("click", () => open_Project_Directory_Dialog());

  project_directory_input_element.addEventListener("keypress", function(event) {
    if (event.key === "Enter" && project_directory_input_element.value.length > 0) {
      event.preventDefault();
      invoke("set_project_directory", { newDirectory: project_directory_input_element.value });
    }
  })
});