const { listen } = window.__TAURI__.event;

const loadingSpinner = document.getElementById("loading-spinner");
const statusText = document.getElementById("status");
const latestVersionText = document.getElementById("latest-version");
const progressBar = document.getElementById("progress-bar");
const downloadProgress = document.getElementById("download-progress")

statusText.innerHTML = "Checking for updates";

listen('update-progress', (event) => {
    loadingSpinner.classList.remove("show");
    
    downloadProgress.classList.add("show");

    latestVersionText.innerHTML = `Newest version: ${event.payload.version}`;

    const { total_size, downloaded_size } = event.payload;
    const progressPercentage = (downloaded_size / total_size) * 100;
    progressBar.style.width = `${progressPercentage.toFixed(2)}%`;
    statusText.innerHTML = `Downloading update: ${progressPercentage.toFixed(2)}%`;
}
)