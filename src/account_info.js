const { invoke } = window.__TAURI__.core;
import { initializeKeys, apiKey, keys } from "./getApiKeys.js";

await initializeKeys("deepl")


function fill_glossary_select(element, list){
    list.forEach(glossary => {
        let option = document.createElement("option");
        option.value = glossary._glossary_id
        option.innerHTML = glossary._name

        element.appendChild(option)
    });
}

async function checkUsage() {
    try {
        const result = await invoke('check_usage', {apiKey: apiKey});
        const parsedResult = JSON.parse(result);

        if (parsedResult.success) {

            const quotaText = document.getElementById("quota-usage")

            quotaText.textContent = `${parsedResult.output.used_characters} caracteres usados de ${parsedResult.output.characters_limit}`

            if (parsedResult.output.characters_limit_reached){
                const limitText = document.getElementById("limit-reached")

                limitText.innerHTML = `Cota limite atingida! Não é mais possível traduzir nesse período`
            }
        } else {
            console.error('Error:', parsedResult.output);
        }
    } catch (error) {
        console.error('Failed to get usage data:', error);
    }
}

async function get_glossaries() {
    try {
        const result = await invoke('get_glossaries', { apiKey: apiKey });
        const parsedResult = JSON.parse(result);


        if (parsedResult.success) {
            const select = document.getElementById("glossary-select")

            fill_glossary_select(select, parsedResult.output)

        } else {
            console.error('Error:', parsedResult.output);
        }
    } catch (error) {
        console.error('Failed to get data:', error);
    }
}


get_glossaries()

checkUsage();