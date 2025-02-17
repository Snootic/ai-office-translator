import { initializeKeys, apiKey, keys } from "./getApiKeys.js";
const { invoke } = window.__TAURI__.core;
const { getVersion } = window.__TAURI__.app;

const versionText = document.getElementById("app-version");
versionText.innerHTML = `v${await getVersion()}`;

const model_select = document.getElementById("model-selector")

await initializeKeys("gpt")

function fill_model_select(element, list){
    list.forEach(model => {
        let option = document.createElement("option");
        option.value = model
        option.innerHTML = model

        element.appendChild(option)
    });
}

async function get_models() {
    try {
        const result = await invoke('get_gpt_models', { apiKey: apiKey });
        const parsedResult = JSON.parse(result);

        if (parsedResult.success) {

            fill_model_select(model_select, parsedResult.output.gpt_models)
            fill_model_select(model_select,["deepl"])

        } else {
            console.error('Error:', parsedResult.output);
        }
    } catch (error) {
        console.error('Failed to get data:', error);
    }
}

// the hide parameter is boolean. It will trigger the class attribute
// "show" or remove it if necessary (only when the deepl model is selected)
function deepl_params(hide){
    const usage = document.querySelector(".quota-info")
    const glossary_div = document.querySelector("#glossary-div")
    if (hide) {
        usage.classList.remove("show");
        glossary_div.classList.remove("show");
    } else{
        usage.classList.add("show");
        glossary_div.classList.add("show");
    }
}

model_select.addEventListener("change", function(){
    if (model_select.value.includes("gpt")) {
        deepl_params(true)
    }else{
        deepl_params(false)
    }
})

get_models()
