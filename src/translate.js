import { initializeKeys, apiKey, keys } from "./getApiKeys.js";
import { message } from "./message.js";
const { invoke } = window.__TAURI__.core;
console.log(window)
console.log(window.__TAURI__)
console.log(window.__TAURI_PLUGIN_DIALOG__)
const { open } = window.__TAURI__.dialog;
// const { appDir } = window.__TAURI__.Manager;

await initializeKeys("deepl");

const divConfirmar = document.querySelector("#confirm-submit");
const formTraduzir = document.getElementById("file-form");
const targetFileInput = document.getElementById("target-file-input");
const confirmarGlossario = document.getElementById("glossary_confirm");
const confirmarTargLang = document.getElementById("target_language_confirm");
const confirmarSrcLang = document.getElementById("source_language_confirm");
const confirmarArquivoOriginal = document.getElementById("original_file_confirm");
const confirmarArquivoDestino = document.getElementById("target_file_confirm");
const confirmarTraducaoBotao = document.getElementById("confirm-translation");
const confirmarModelo = document.getElementById("model_confirm");
const cancelarTraducaoBotao = document.getElementById("cancel-translation");

function updateConfirmFields(formData) {
    confirmarSrcLang.value = formData.get("source-language") || "Nenhum idioma selecionado";
    confirmarTargLang.value = formData.get("target-language") || "Nenhum idioma selecionado";
    confirmarGlossario.value = formData.get("glossary-select") || "Nenhum glossário selecionado (Não disponível para GPT)";

    const originalFile = formData.get("original-file-input");
    confirmarArquivoOriginal.value = originalFile ? originalFile.name : "Nenhum arquivo selecionado";
    confirmarArquivoDestino.value = formData.get("target-file-input") || "Nenhum caminho selecionado";
    confirmarModelo.value = document.getElementById("model-selector").value
}

async function handleTranslation(formData) {
    if (document.getElementById("model-selector").value.includes("gpt")){
        await initializeKeys("gpt");
    }

    const loadingSpinner = document.getElementById("loading-spinner");
    const shadow = document.getElementById("background-shadow");

    shadow.classList.add("show");
    loadingSpinner.classList.add("show");

    document.body.style.pointerEvents = "none";
    document.body.style.overflow = 'hidden';              

    const originalFile = formData.get("original-file-input");
    if (!originalFile) {
        console.error("Nenhum arquivo foi selecionado.");
        return;
    }

    const reader = new FileReader();
    reader.readAsArrayBuffer(originalFile);
    reader.onload = async function () {
        const fileData = new Uint8Array(reader.result);
        let args = [formData.get("target-file-input"),formData.get("target-language"),formData.get("source-language")]
        let model = document.getElementById("model-selector").value
            try {
                const result = await invoke("translate_document", {
                    apiKey: apiKey,
                    model: model,
                    fileData: Array.from(fileData),
                    fileName: originalFile.name,
                    args: args,
                    kwargs: null
                });

                const parsedResult = JSON.parse(result);
                console.log(parsedResult.output);
                message(parsedResult.success ? "Arquivo traduzido com sucesso!" : "Falha na tradução :(");
            } catch (error) {
                console.error("Erro:", error);
                message("Ocorreu um erro!");
            } finally {
                loadingSpinner.classList.remove("show");
                    shadow.classList.remove("show");
                    document.body.style.pointerEvents = "auto";
                    document.body.style.overflow = '';
            }
    };
}

formTraduzir.addEventListener("submit", (event) => {
    event.preventDefault();
    const formData = new FormData(formTraduzir);
    updateConfirmFields(formData);
    divConfirmar.classList.add("show");
});

confirmarTraducaoBotao.addEventListener("click", async (event) => {
    event.preventDefault();
    const formData = new FormData(formTraduzir);
    await handleTranslation(formData);
});

cancelarTraducaoBotao.addEventListener("click", () => {
    divConfirmar.classList.remove("show");
});

targetFileInput.addEventListener("click", async (event) => {
    event.preventDefault();
    try {
        const selected = await open({
            directory: true,
        });

        if (selected) {
            const originalFile = document.getElementById("original-file-input").files[0];
            if (originalFile) {
                const targetLanguage = document.getElementById("target-language").options[document.getElementById("target-language").selectedIndex].innerHTML;
                
                const originalFileName = originalFile.name;
                const extensionIndex = originalFileName.lastIndexOf('.');

                const fileNameWithoutExt = originalFileName.slice(0, extensionIndex);
                const fileExtension = originalFileName.slice(extensionIndex);
                
                targetFileInput.value = `${selected}/${fileNameWithoutExt} - ${targetLanguage}${fileExtension}`;
            } else {
                message("Nenhum arquivo original selecionado.");
            }
        }
    } catch (error) {
        console.error("Erro ao selecionar diretório:", error);
    }
});


