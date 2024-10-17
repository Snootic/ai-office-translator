import { message } from "./message.js";

const { invoke } = window.__TAURI__.core;

const source_file_input = document.getElementById("original-file-input")
let doc_div = document.getElementById("document-info-div");

source_file_input.addEventListener("change", function(){
    const source_file = source_file_input.files[0]
    const reader = new FileReader();
    reader.readAsArrayBuffer(source_file);
    reader.onload = async function () {
        doc_div.innerHTML = ""

        const loadingSpinner = document.getElementById("loading-spinner");
        const shadow = document.getElementById("background-shadow");
    
        shadow.classList.add("show");
        loadingSpinner.classList.add("show");
    
        document.body.style.pointerEvents = "none";
        document.body.style.overflow = 'hidden';

        const fileData = new Uint8Array(reader.result);
        try {
            // need this await to make sure the loading spinner will spin before the tauri invoke
            // if the file is too large it will still freeze though, probably due a memory leak idk
            await new Promise(resolve => setTimeout(resolve, 50));

            const result = await invoke("load_document",{
                fileData: Array.from(fileData),
                fileName: source_file.name
            })

            const parsedResult = JSON.parse(result);
            console.log(parsedResult.output)
            let output = parsedResult.output

            for (let key in output) {
                if (output.hasOwnProperty(key)) {
                    const value = output[key];
            
                    if (value === "" || value === null || value === "<not serializable>") {
                        continue;
                    }
            
                    const p = document.createElement("p");
                    p.innerHTML = `<strong>${key}</strong>: ${value}`;
                    doc_div.appendChild(p);
                }
            }

            message(parsedResult.success ? "Arquivo carregado!" : "Falha ao carregar arquivo :(");
        } catch (error) {
            console.error("erro:", error);
            message("Ocorreu um erro!")
        }finally{
            loadingSpinner.classList.remove("show");
            shadow.classList.remove("show");
            document.body.style.pointerEvents = "auto";
            document.body.style.overflow = '';
        }
    }
})

