import { initializeKeys, apiKey, keys } from "./getApiKeys.js";
import { message } from "./message.js";
const { invoke } = window.__TAURI__.tauri;
const { open } = window.__TAURI__.dialog;
const { appDir } = window.__TAURI__.path;

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
const cancelarTraducaoBotao = document.getElementById("cancel-translation");

function updateConfirmFields(formData) {
    confirmarSrcLang.value = formData.get("source-language") || "Nenhum idioma selecionado";
    confirmarTargLang.value = formData.get("target-language") || "Nenhum idioma selecionado";
    confirmarGlossario.value = formData.get("glossary-select") || "Nenhum glossário selecionado (Não disponível para GPT)";

    const originalFile = formData.get("original-file-input");
    confirmarArquivoOriginal.value = originalFile ? originalFile.name : "Nenhum arquivo selecionado";
    confirmarArquivoDestino.value = formData.get("target-file-input") || "Nenhum caminho selecionado";
}

async function handleTranslation(formData) {
    const originalFile = formData.get("original-file-input");
    if (!originalFile) {
        console.error("Nenhum arquivo foi selecionado.");
        return;
    }

    const reader = new FileReader();
    reader.readAsArrayBuffer(originalFile);
    reader.onload = async function () {
        const fileData = new Uint8Array(reader.result);
        try {
            const result = await invoke("translate_doc_alt", {
                key: apiKey,
                fileData: Array.from(fileData),
                fileName: originalFile.name,
                fileOutput: formData.get("target-file-input"),
                sourceLanguage: formData.get("source-language"),
                targetLanguage: formData.get("target-language"),
                glossaryId: formData.get("glossary-select"),
            });

            const parsedResult = JSON.parse(result);
            message(parsedResult.success ? "Arquivo traduzido com sucesso!" : "Falha na tradução :(");
        } catch (error) {
            console.error("Erro:", error);
            message("Ocorreu um erro!");
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
            defaultPath: await appDir(),
        });

        if (selected) {
            const originalFile = document.getElementById("original-file-input").files[0];
            if (originalFile) {
                const targetLanguage = document.getElementById("target-language").options[document.getElementById("target-language").selectedIndex].innerHTML;
                targetFileInput.value = `${selected}/${targetLanguage} - ${originalFile.name}`;
            } else {
                message("Nenhum arquivo original selecionado.");
            }
        }
    } catch (error) {
        console.error("Erro ao selecionar diretório:", error);
    }
});