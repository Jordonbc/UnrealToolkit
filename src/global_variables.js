export const { invoke } = window.__TAURI__.tauri;
export const listen = window.__TAURI__.event.listen;

export let ue_directory_input_element;
export let ue_directory_input_button;
export let project_directory_input_element;
export let project_directory_input_button;
export let is_source_warning_element;
export let client_section;
export let server_section;

export let ue_directory;
export let project_directory
export let is_source;

async function reload_variables() {
    console.log("Reloading Variables!");
    project_directory = await invoke("get_project_directory");
    ue_directory = await invoke("get_ue_directory");
    is_source = await invoke("get_is_source_directory");

    if (ue_directory_input_element !== null) {
        ue_directory_input_element.value = ue_directory;
    }

    if (project_directory_input_element !== null) {
        project_directory_input_element.value = project_directory;
    }

    if (is_source_warning_element !== null)
    {
        if (is_source)
        {
            is_source_warning_element.style.display = "none";
        }
        else {
            is_source_warning_element.style.display = "block";
        }
    }

    if (server_section !== null) {
        if (is_source)
        {
            server_section.style.display = "block";
        }
        else {
            server_section.style.display = "none";
        }
    }
  }

window.addEventListener("DOMContentLoaded", () => {
    console.log("Initializing global variables!");
    ue_directory_input_element = document.getElementById("ue_directory_input");
    ue_directory_input_button = document.getElementById("open_ue_directory_button");
    project_directory_input_element = document.getElementById("project_directory_input");
    project_directory_input_button = document.getElementById("open_project_directory_button");
    is_source_warning_element= document.getElementById("is_source_warning");
    client_section = document.getElementById("Client");
    server_section = document.getElementById("Server");

    reload_variables();
    listen("update_frontend", function (string) {
        reload_variables();
    });
});