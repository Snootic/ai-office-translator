import { message } from "./message.js";

const { invoke } = window.__TAURI__.core;

const source_file_input = document.getElementById("original-file-input")

source_file_input.addEventListener("change", function(){
    const source_file = source_file_input.files[0]
    const reader = new FileReader();
    reader.readAsArrayBuffer(source_file);
    reader.onload = async function () {
        const fileData = new Uint8Array(reader.result);
        try {
            const result = await invoke("load_document",{
                fileData: Array.from(fileData),
                fileName: source_file.name
            })

            const parsedResult = JSON.parse(result);
            console.log(parsedResult.output)
            let output = parsedResult.output

            let doc_div = document.getElementById("document-info-div");
            
            doc_div.innerHTML = ""

            for (let key in output) {
                if (output.hasOwnProperty(key)) {
                    const value = output[key];
            
                    if (value === "" || value === null || value === "<not serializable>") {
                        continue;
                    }
            
                    const p = document.createElement("p");
                    p.textContent = `${key}: ${value}`;
                    doc_div.appendChild(p);
                }
            }

            message(parsedResult.success ? "Arquivo carregado!" : "Falha ao carregar arquivo :(");
        } catch (error) {
            console.error("erro:", error);
            message("Ocorreu um erro!")
        }
    }
})

