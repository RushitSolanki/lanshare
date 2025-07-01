const statusText = document.getElementById('status-text');
const textarea = document.getElementById('main-text');

let ws;

function setStatus(text, color = '#3182ce') {
  statusText.textContent = text;
  statusText.style.color = color;
  console.log(`Status: ${text}`); // Debug logging
}

function connectWebSocket() {
  // In production, replace ws://localhost:9001 with dynamic peer address
  ws = new WebSocket('ws://localhost:9001');

  ws.onopen = () => {
    setStatus('Connected', '#38a169');
    console.log('WebSocket connected');
  };
  ws.onclose = () => {
    setStatus('Disconnected', '#e53e3e');
    console.log('WebSocket disconnected');
  };
  ws.onerror = (error) => {
    setStatus('Error', '#e53e3e');
    console.error('WebSocket error:', error);
  };

  ws.onmessage = (event) => {
    console.log('Received message:', event.data);
    if (event.data !== textarea.value) {
      textarea.value = event.data;
    }
  };
}

// Debug Tauri commands
async function debugTauriCommands() {
  try {
    console.log('Testing Tauri commands...');
    
    // Test get_peers
    const peers = await window.__TAURI__.invoke('get_peers');
    console.log('Discovered peers:', peers);
    
    // Test get_peer_count
    const peerCount = await window.__TAURI__.invoke('get_peer_count');
    console.log('Peer count:', peerCount);
    
    // Test get_peer_id
    const peerId = await window.__TAURI__.invoke('get_peer_id');
    console.log('Current peer ID:', peerId);
    
    // Update debug panel
    document.getElementById('peer-id').textContent = peerId || 'Not available';
    document.getElementById('peer-count').textContent = peerCount;
    document.getElementById('peers-list').textContent = peers.length > 0 
      ? peers.map(p => `${p.id} (${p.hostname})`).join(', ')
      : 'No peers discovered';
    
  } catch (error) {
    console.error('Error testing Tauri commands:', error);
    document.getElementById('peer-id').textContent = 'Error';
    document.getElementById('peer-count').textContent = 'Error';
    document.getElementById('peers-list').textContent = 'Error: ' + error.message;
  }
}

textarea.addEventListener('input', () => {
  if (ws && ws.readyState === WebSocket.OPEN) {
    ws.send(textarea.value);
    console.log('Sent message:', textarea.value);
  }
});

window.addEventListener('DOMContentLoaded', () => {
  setStatus('Connecting...');
  console.log('DOM loaded, starting application...');
  
  // Test Tauri commands after a short delay
  setTimeout(debugTauriCommands, 1000);
  
  // Update debug info every 5 seconds
  setInterval(debugTauriCommands, 5000);
  
  connectWebSocket();
});

// Add global debug function
window.debugLanShare = debugTauriCommands;
