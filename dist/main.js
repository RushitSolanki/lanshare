const statusText = document.getElementById('status-text');
const textarea = document.getElementById('main-text');

let ws;

function setStatus(text, color = '#3182ce') {
  statusText.textContent = text;
  statusText.style.color = color;
}

function connectWebSocket() {
  // In production, replace ws://localhost:9001 with dynamic peer address
  ws = new WebSocket('ws://localhost:9001');

  ws.onopen = () => setStatus('Connected', '#38a169');
  ws.onclose = () => setStatus('Disconnected', '#e53e3e');
  ws.onerror = () => setStatus('Error', '#e53e3e');

  ws.onmessage = (event) => {
    if (event.data !== textarea.value) {
      textarea.value = event.data;
    }
  };
}

textarea.addEventListener('input', () => {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(textarea.value);
  }
});

window.addEventListener('DOMContentLoaded', () => {
  setStatus('Connecting...');
  connectWebSocket();
});
