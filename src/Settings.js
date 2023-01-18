const { invoke } = window.__TAURI__.tauri;
const listen = window.__TAURI__.event.listen;

let ue_directory_input_element;
let ue_directory_input_button;


function openUEDirectory_Dialog() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("open_ue_directory_dialog");
  //greetMsgEl.textContent = await invoke("nameOfRustFunctionHere", { paramNameHere: paramater});
}

window.addEventListener("DOMContentLoaded", () => {
  ue_directory_input_element = document.getElementById("ue_directory_input");
  ue_directory_input_button = document.getElementById("open_ue_directory_button");

  listen("ue_directory_changed", function (s) {
    console.log("Recieved: " + s.payload);
    ue_directory_input_element.value = s.payload;
  });
  
  ue_directory_input_button.addEventListener("click", () => openUEDirectory_Dialog());

  ue_directory_input_element.addEventListener("keypress", function(event) {
    // If the user presses the "Enter" key on the keyboard
    if (event.key === "Enter") {
      console.log("pressed enter");
      // Cancel the default action, if needed
      event.preventDefault();
      // Trigger the button element with a click
      invoke("set_ue_directory", { newDirectory: ue_directory_input_element.value });
    }
  })
});