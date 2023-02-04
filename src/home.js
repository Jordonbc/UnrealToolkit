import * as GLOBALS from '/global_variables.js';

function update_ui_elements() {
  GLOBALS.project_directory_input_element.value = GLOBALS.project_directory;
  GLOBALS.output_directory_input_element.value = GLOBALS.output_directory;

  if (GLOBALS.server_section !== null) {
    if (GLOBALS.is_source) {
      GLOBALS.server_section.style.display = "block";
    }
    else {
      GLOBALS.server_section.style.display = "none";
    }
  }

  if (GLOBALS.client_section !== null) {
    if (GLOBALS.client_configuration.configuration.includes("win64")) {
      GLOBALS.client_win64_element.checked = true;
    }

    if (GLOBALS.client_configuration.configuration.includes("linux")) {
      GLOBALS.client_linux_element.checked = true;
    }

    if (GLOBALS.client_configuration.configuration.includes("mac")) {
      GLOBALS.client_mac_element.checked = true;
    }

    switch (GLOBALS.client_configuration.build) {
      case "Shipping":
        GLOBALS.client_shipping_element.checked = true;
        break;
      case "Test":
        GLOBALS.client_test_element.checked = true;
        break;
      case "Development":
        GLOBALS.client_development_element.checked = true;
        break;
      default:
        break;
    }

    GLOBALS.client_no_crash_reporter_element.checked = GLOBALS.client_configuration.remove_crash_reporter;
  }

  if (GLOBALS.server_section !== null) {
    if (GLOBALS.server_configuration.configuration.includes("win64")) {
      GLOBALS.server_win64_element.checked = true;
    }

    if (GLOBALS.server_configuration.configuration.includes("linux")) {
      GLOBALS.server_linux_element.checked = true;
    }

    if (GLOBALS.server_configuration.configuration.includes("mac")) {
      GLOBALS.server_mac_element.checked = true;
    }

    switch (GLOBALS.server_configuration.build) {
      case "Shipping":
        GLOBALS.server_shipping_element.checked = true;
        break;
      case "Test":
        GLOBALS.server_test_element.checked = true;
        break;
      case "Development":
        GLOBALS.server_development_element.checked = true;
        break;
      default:
        break;
    }

    GLOBALS.server_no_crash_reporter_element.checked = GLOBALS.server_configuration.remove_crash_reporter;
  }

  if (GLOBALS.packaging_client === "Running")
  {
    package_client_button.style.background = "darkred";
    package_client_button.textContent = "⏳Cancel";
  }
  else {
    package_client_button.style.background = "#0f0f0f98";
    package_client_button.textContent = "Package Client";
  }
}

function update_backend_client() {
  let selected_client_build;
  let radios = document.getElementsByName('client_build');
  for (var i = 0, length = radios.length; i < length; i++) {
    if (radios[i].checked) {
      // do whatever you want with the checked radio
      selected_client_build = radios[i].value;
  
      // only one radio can be logically checked, don't check the rest
      break;
    }
  }

  const selected_client_configurations = [];
  radios = document.getElementsByName('client_configuration');
  for (var i = 0, length = radios.length; i < length; i++) {
    if (radios[i].checked) {
      // do whatever you want with the checked radio
      selected_client_configurations.push(radios[i].value);
    }
  }

  let config = {
      configuration: selected_client_configurations,
      build: selected_client_build,
      remove_crash_reporter: GLOBALS.client_no_crash_reporter_element.checked,
      is_server: false,
    };

  GLOBALS.invoke("set_client_configuration", { newClientConfig: config });
}

function update_backend_server() {
  let selected_server_build;
  let radios = document.getElementsByName('server_build');
  for (var i = 0, length = radios.length; i < length; i++) {
    if (radios[i].checked) {
      // do whatever you want with the checked radio
      selected_server_build = radios[i].value;
  
      // only one radio can be logically checked, don't check the rest
      break;
    }
  }

  const selected_server_configurations = [];
  radios = document.getElementsByName('server_configuration');
  for (var i = 0, length = radios.length; i < length; i++) {
    if (radios[i].checked) {
      // do whatever you want with the checked radio
      selected_server_configurations.push(radios[i].value);
    }
  }


  let config = {
      configuration: selected_server_configurations,
      build: selected_server_build,
      remove_crash_reporter: GLOBALS.server_no_crash_reporter_element.checked,
      is_server: true,
    };

  GLOBALS.invoke("set_server_configuration", { newServerConfig: config });
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("Running home.js");
  GLOBALS.project_directory_input_button.addEventListener("click", () => GLOBALS.invoke("open_project_directory_dialog"));

  GLOBALS.project_directory_input_element.addEventListener("keypress", function(event) {
    if (event.key === "Enter" && GLOBALS.project_directory_input_element.value.length > 0) {
      event.preventDefault();
      GLOBALS.invoke("set_project_directory", { newDirectory: GLOBALS.project_directory_input_element.value });
    }
  });

  GLOBALS.output_directory_input_button.addEventListener("click", () => GLOBALS.invoke("open_output_directory_dialog"));

  GLOBALS.output_directory_input_element.addEventListener("keypress", function(event) {
    if (event.key === "Enter" && GLOBALS.output_directory_input_element.value.length > 0) {
      event.preventDefault();
      GLOBALS.invoke("set_project_directory", { newDirectory: GLOBALS.output_directory_input_element.value });
    }
  });

  GLOBALS.client_win64_element.addEventListener("click", () => update_backend_client());
  GLOBALS.client_linux_element.addEventListener("click", () => update_backend_client());
  GLOBALS.client_mac_element.addEventListener("click", () => update_backend_client());
  GLOBALS.client_shipping_element.addEventListener("click", () => update_backend_client());
  GLOBALS.client_test_element.addEventListener("click", () => update_backend_client());
  GLOBALS.client_development_element.addEventListener("click", () => update_backend_client());
  GLOBALS.client_no_crash_reporter_element.addEventListener("click", () => update_backend_client());

  GLOBALS.server_win64_element.addEventListener("click", () => update_backend_server());
  GLOBALS.server_linux_element.addEventListener("click", () => update_backend_server());
  GLOBALS.server_mac_element.addEventListener("click", () => update_backend_server());
  GLOBALS.server_shipping_element.addEventListener("click", () => update_backend_server());
  GLOBALS.server_test_element.addEventListener("click", () => update_backend_server());
  GLOBALS.server_development_element.addEventListener("click", () => update_backend_server());
  GLOBALS.server_no_crash_reporter_element.addEventListener("click", () => update_backend_server());

  GLOBALS.package_client_button.addEventListener("click", () => GLOBALS.package_client());

  GLOBALS.package_server_button.addEventListener("click", () => GLOBALS.package_server());

  document.addEventListener("reload_ui", e => update_ui_elements());
});