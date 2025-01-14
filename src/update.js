const { listen } = window.__TAURI__.event;

const loadingSpinner = document.getElementById("loading-spinner");
const statusText = document.getElementById("status");
const progressBar = document.getElementById("progress-bar");
const downloadProgress = document.getElementById("download-progress")

statusText.innerHTML = "Checking for updates";

listen('update-progress', (event) => {
    loadingSpinner.classList.remove("show");
    
    downloadProgress.classList.add("show");

    if (event.payload == "no-update") {
        window.location.replace("index.html");
    }

    const { total_size, downloaded_size } = event.payload;
    const progressPercentage = (downloaded_size / total_size) * 100;
    progressBar.style.width = `${progressPercentage.toFixed(2)}%`;
    statusText.innerHTML = `Downloading update: ${progressPercentage.toFixed(2)}%`;
}
)