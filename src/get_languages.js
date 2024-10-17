const { invoke } = window.__TAURI__.core;
import { initializeKeys, apiKey, keys } from "./getApiKeys.js";

await initializeKeys("deepl")
// it uses the deepl key to fetch the languages because it easier this way
// i must find a source for languages though to not depend on it
// if there is no deepl key the app does not work currently
// probably just listing all languages on a json file and loading it will be better
// can use less resource this way, since there is a memory leak in the rust side (pyo3?)

function fill_select(element, list) {
    list.forEach(language => {
        let option = document.createElement("option");
        option.value = language.code
        option.innerHTML = language.name

        element.appendChild(option)
    });
}

async function get_source_languages() {
    try {
        const result = await invoke('get_source_languages', { apiKey: apiKey });
        const parsedResult = JSON.parse(result);

        if (parsedResult.success) {
            const select = document.getElementById("source-language")

            fill_select(select, parsedResult.output)

        } else {
            console.error('Error:', parsedResult.output);
        }
    } catch (error) {
        console.error('Failed to get data:', error);
    }
}

async function get_target_languages() {
    try {
        const result = await invoke('get_target_languages', { apiKey: apiKey });
        const parsedResult = JSON.parse(result);

        if (parsedResult.success) {
            const select = document.getElementById("target-language")

            fill_select(select, parsedResult.output)

        } else {
            console.error('Error:', parsedResult.output);
        }
    } catch (error) {
        console.error('Failed to get data:', error);
    }
}

// will trigger when the app starts
get_source_languages();
get_target_languages();

export {fill_select};
