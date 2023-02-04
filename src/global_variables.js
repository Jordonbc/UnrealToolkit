export const { invoke } = window.__TAURI__.tauri;
export const listen = window.__TAURI__.event.listen;

// elements
export let ue_directory_input_element;
export let ue_directory_input_button;
export let project_directory_input_element;
export let project_directory_input_button;
export let output_directory_input_element;
export let output_directory_input_button;
export let is_source_warning_element;
export let client_section;
export let server_section;
export let package_client_button;
export let package_server_button;

// client configuration
export let client_win64_element;
export let client_linux_element;
export let client_mac_element;

export let client_shipping_element;
export let client_test_element;
export let client_development_element;

export let client_no_crash_reporter_element;

// server configuration
export let server_win64_element;
export let server_linux_element;
export let server_mac_element;

export let server_shipping_element;
export let server_test_element;
export let server_development_element;

export let server_no_crash_reporter_element;

// global variables
export let ue_directory;
export let project_directory
export let output_directory
export let is_source;
export let client_configuration;
export let server_configuration;
export let packaging_client;
export let packaging_server;

const reload_ui_event = new Event('reload_ui');

async function reload_variables() {
    console.log("Reloading Variables!");
    project_directory = await invoke("get_project_directory");
    output_directory = await invoke("get_compiled_output_directory");
    ue_directory = await invoke("get_ue_directory");
    is_source = await invoke("get_is_source_directory");
    client_configuration = await invoke("get_client_configuration");
    server_configuration = await invoke("get_server_configuration");
    packaging_client = await invoke("get_client_packaging_status");
    packaging_server = await invoke("get_server_packaging_status");

    console.log(packaging_client);
    console.log(packaging_server);

    document.dispatchEvent(reload_ui_event);
  }

function init_buttons() {
    project_directory_input_button = document.getElementById("open_project_directory_button");
    ue_directory_input_button = document.getElementById("open_ue_directory_button");
    output_directory_input_button = document.getElementById("open_output_directory_button");
    package_client_button = document.getElementById("package_client_button");
    package_server_button = document.getElementById("package_server_button");
}

function init_input_elements() {
    ue_directory_input_element = document.getElementById("ue_directory_input");
    project_directory_input_element = document.getElementById("project_directory_input");
    output_directory_input_element = document.getElementById("output_directory_input");
}

function init_warnings() {
    is_source_warning_element= document.getElementById("is_source_warning");
}

function init_misc() {
    client_section = document.getElementById("Client");
    server_section = document.getElementById("Server");
}

function init_forms() {
    // Client
    client_win64_element = document.getElementById("client_win64");
    client_linux_element = document.getElementById("client_linux");
    client_mac_element = document.getElementById("client_mac");

    client_shipping_element = document.getElementById("client_shipping");
    client_test_element = document.getElementById("client_test");
    client_development_element = document.getElementById("client_development");

    client_no_crash_reporter_element = document.getElementById("client_no_crashreporter");

    // Server
    server_win64_element = document.getElementById("server_win64");
    server_linux_element = document.getElementById("server_linux");
    server_mac_element = document.getElementById("server_mac");

    server_shipping_element = document.getElementById("server_shipping");
    server_test_element = document.getElementById("server_test");
    server_development_element = document.getElementById("server_development");

    server_no_crash_reporter_element = document.getElementById("server_no_crashreporter");
}

window.addEventListener("DOMContentLoaded", () => {
    console.log("Initializing global variables!");

    init_buttons();
    init_input_elements();
    init_warnings();
    init_misc();
    init_forms();

    reload_variables();
    listen("update_frontend", function (string) {
        reload_variables();
    });
});

export async function package_client() {
    let result = await invoke("package_client");
}

export async function package_server() {
    let result = invoke("package_server");
}