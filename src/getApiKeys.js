let apiKey;
let keys;

async function initializeKeys(model) {
    
    const { invoke } = window.__TAURI__.core;
    if (model.includes("gpt")){
        keys = await invoke('get_chatgpt_keys');
    }else{
        keys = await invoke('get_deep_keys');
    }

    // for now just returning the first key found
    // will still, in the future, make it handle multiple keys wisely
    apiKey = keys[0].key; 
}

export { apiKey, keys, initializeKeys };
