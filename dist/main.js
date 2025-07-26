// Tauri API references
let invoke, appWindow, fs, dialog, notification, os;
let ws;

const statusText = document.getElementById('status-text');
const textarea = document.getElementById('main-text');

function setStatus(text, color = '#3182ce') {
    statusText.textContent = text;
    statusText.style.color = color;
    console.log(`Status: ${text}`);
}

// Wait for Tauri APIs to be available (corrected for static setup)
async function waitForTauri() {
    console.log('Waiting for Tauri APIs...');
    
    return new Promise((resolve, reject) => {
        let attempts = 0;
        const maxAttempts = 50;
        
        const checkTauri = () => {
            attempts++;
            console.log(`Checking for Tauri APIs... attempt ${attempts}`);
            
            // Check multiple possible locations for Tauri APIs
            if (window.__TAURI_API__) {
                console.log('Found __TAURI_API__');
                invoke = window.__TAURI_API__.invoke;
                resolve();
            } else if (window.__TAURI__) {
                console.log('Found __TAURI__');
                // Try different ways to access invoke in Tauri 2.x
                if (window.__TAURI__.invoke) {
                    invoke = window.__TAURI__.invoke;
                } else if (window.__TAURI__.core && window.__TAURI__.core.invoke) {
                    invoke = window.__TAURI__.core.invoke;
                } else if (window.__TAURI__.api && window.__TAURI__.api.invoke) {
                    invoke = window.__TAURI__.api.invoke;
                } else {
                    console.log('Available __TAURI__ properties:', Object.keys(window.__TAURI__));
                    // Try to find invoke in the object structure
                    for (const key of Object.keys(window.__TAURI__)) {
                        const obj = window.__TAURI__[key];
                        if (obj && typeof obj.invoke === 'function') {
                            invoke = obj.invoke;
                            console.log('Found invoke in:', key);
                            break;
                        }
                    }
                }
                
                appWindow = window.__TAURI__.window?.getCurrent?.();
                fs = window.__TAURI__.fs;
                dialog = window.__TAURI__.dialog;
                notification = window.__TAURI__.notification;
                os = window.__TAURI__.os;
                resolve();
            } else if (window.invoke) {
                console.log('Found global invoke');
                invoke = window.invoke;
                resolve();
            } else if (attempts < maxAttempts) {
                setTimeout(checkTauri, 100);
            } else {
                console.error('Tauri APIs not found after', maxAttempts, 'attempts');
                console.log('Available window properties:', Object.keys(window).filter(key => key.includes('TAURI') || key === 'invoke'));
                reject(new Error('Tauri APIs not available'));
            }
        };
        
        checkTauri();
    });
}

// Debug function to check what's available
function debugWindowAPIs() {
    console.log('=== Debugging Window APIs ===');
    console.log('window.__TAURI__:', !!window.__TAURI__);
    console.log('window.__TAURI_API__:', !!window.__TAURI_API__);
    console.log('window.invoke:', !!window.invoke);
    console.log('window.__TAURI_INVOKE__:', !!window.__TAURI_INVOKE__);
    
    const tauriKeys = Object.keys(window).filter(key => 
        key.includes('TAURI') || key.includes('tauri') || key === 'invoke'
    );
    console.log('Tauri-related keys:', tauriKeys);
    
    if (window.__TAURI__) {
        console.log('__TAURI__ contents:', Object.keys(window.__TAURI__));
        
        // Deep inspect the __TAURI__ object to find invoke
        console.log('=== Deep __TAURI__ Inspection ===');
        for (const key of Object.keys(window.__TAURI__)) {
            const obj = window.__TAURI__[key];
            console.log(`${key}:`, typeof obj);
            if (obj && typeof obj === 'object') {
                console.log(`  ${key} properties:`, Object.keys(obj));
                if (obj.invoke) {
                    console.log(`  Found invoke in ${key}!`);
                }
            }
        }
    }
}

// WebSocket connection with retry logic (unchanged)
function connectWebSocket(retryCount = 0) {
    const maxRetries = 5;
    const retryDelay = Math.min(1000 * Math.pow(2, retryCount), 10000);
    
    try {
        ws = new WebSocket('ws://localhost:9001');
        
        ws.onopen = () => {
            setStatus('Connected', '#38a169');
            console.log('WebSocket connected');
        };
        
        ws.onclose = (event) => {
            setStatus('Disconnected', '#e53e3e');
            console.log('WebSocket disconnected:', event.code, event.reason);
            
            if (event.code !== 1000 && retryCount < maxRetries) {
                console.log(`Retrying connection in ${retryDelay}ms... (attempt ${retryCount + 1}/${maxRetries})`);
                setTimeout(() => connectWebSocket(retryCount + 1), retryDelay);
            } else if (retryCount >= maxRetries) {
                setStatus('Connection failed - Max retries reached', '#e53e3e');
            }
        };
        
        ws.onerror = (error) => {
            setStatus('Connection Error', '#e53e3e');
            console.error('WebSocket error:', error);
        };
        
        ws.onmessage = (event) => {
            console.log('Received message:', event.data);
            if (event.data !== textarea.value) {
                textarea.value = event.data;
            }
        };
        
    } catch (error) {
        console.error('Failed to create WebSocket:', error);
        setStatus('WebSocket Creation Failed', '#e53e3e');
    }
}

// Enhanced debug function matching your Rust command signatures
async function debugTauriCommands() {
    if (!invoke) {
        console.warn('Tauri invoke not available yet');
        return;
    }
    
    try {
        console.log('Testing Tauri commands...');
        
        const results = {
            peers: [],
            peerCount: 0,
            peerId: null
        };
        
        // Test get_peers
        try {
            const peersResult = await invoke('get_peers');
            console.log('Raw peers result:', peersResult);
            results.peers = Array.isArray(peersResult) ? peersResult : [];
            console.log('Discovered peers:', results.peers);
        } catch (error) {
            console.error('Error getting peers:', error);
            results.peers = [];
        }
        
        // Test get_peer_count
        try {
            const peerCountResult = await invoke('get_peer_count');
            console.log('Raw peer count result:', peerCountResult);
            results.peerCount = typeof peerCountResult === 'number' ? peerCountResult : 0;
            console.log('Peer count:', results.peerCount);
        } catch (error) {
            console.error('Error getting peer count:', error);
            results.peerCount = 0;
        }
        
        // Test get_peer_id
        try {
            const peerIdResult = await invoke('get_peer_id');
            console.log('Raw peer ID result:', peerIdResult);
            results.peerId = peerIdResult;
            console.log('Current peer ID:', results.peerId);
        } catch (error) {
            console.error('Error getting peer ID:', error);
            results.peerId = null;
        }

        try {
            const peerStructure = await invoke('debug_peer_structure');
            console.log('Peer structure debug:', peerStructure);
        } catch (error) {
            console.log('Debug peer structure not available:', error);
        }
        
        // Update UI with results
        console.log('Updating debug panel with results:', results);
        updateDebugPanel(results);
        
    } catch (error) {
        console.error('Error in debugTauriCommands:', error);
        updateDebugPanel({
            peerId: null,
            peerCount: 0,
            peers: []
        });
    }
}

function updateDebugPanel(results) {
    console.log('updateDebugPanel called with:', results);
    
    const peerIdElement = document.getElementById('peer-id');
    const peerCountElement = document.getElementById('peer-count');
    const peersListElement = document.getElementById('peers-list');
    
    console.log('Found elements:', {
        peerId: !!peerIdElement,
        peerCount: !!peerCountElement,
        peersList: !!peersListElement
    });
    
    if (peerIdElement) {
        peerIdElement.textContent = results.peerId || 'Not available';
        console.log('Updated peer ID to:', results.peerId || 'Not available');
    }
    
    if (peerCountElement) {
        peerCountElement.textContent = results.peerCount.toString();
        console.log('Updated peer count to:', results.peerCount.toString());
    }
    
    if (peersListElement) {
        if (results.peers && results.peers.length > 0) {
            const peerList = results.peers.map(peer => {
                if (typeof peer === 'string') {
                    return peer;
                } else if (peer.id && peer.hostname) {
                    return `${peer.id} (${peer.hostname})`;
                } else if (peer.id) {
                    return peer.id;
                } else {
                    return JSON.stringify(peer);
                }
            }).join(', ');
            
            peersListElement.textContent = peerList;
            console.log('Updated peers list to:', peerList);
        } else {
            peersListElement.textContent = 'No peers discovered';
            console.log('Updated peers list to: No peers discovered');
        }
    }
}

// Text area event handler
function setupTextAreaHandler() {
    if (!textarea) {
        console.error('Textarea not found');
        return;
    }
    
    textarea.addEventListener('input', () => {
        if (ws && ws.readyState === WebSocket.OPEN) {
            ws.send(textarea.value);
            console.log('Sent message:', textarea.value);
        } else {
            console.warn('WebSocket not connected, cannot send message');
        }
    });
}

// Main initialization function
async function initializeApp() {
    try {
        console.log('DOM loaded, starting application...');
        setStatus('Initializing...');
        
        // Debug what's available first
        debugWindowAPIs();
        
        // Wait for Tauri APIs
        await waitForTauri();
        console.log('Tauri APIs loaded successfully');
        
        // Setup event handlers
        setupTextAreaHandler();
        
        // Connect WebSocket
        //setStatus('Connecting to WebSocket...');
        //connectWebSocket();
        
        // Test Tauri commands after initialization
        setTimeout(() => {
            setStatus('Testing Tauri commands...');
            debugTauriCommands();
        }, 1000);
        
        // Set up periodic updates
        setInterval(debugTauriCommands, 5000);
        
    } catch (error) {
        console.error('Failed to initialize app:', error);
        setStatus('Initialization failed', '#e53e3e');
    }
}

// Event listeners
document.addEventListener('DOMContentLoaded', initializeApp);

if (document.readyState !== 'loading') {
    initializeApp();
}

// Global functions
window.debugLanShare = debugTauriCommands;
window.debugWindowAPIs = debugWindowAPIs;
window.reconnectWebSocket = () => {
    if (ws) ws.close();
    setStatus('Reconnecting...');
    connectWebSocket();
};

// Add a function to test invoke directly
window.testInvoke = async () => {
    console.log('Testing invoke directly...');
    if (invoke) {
        try {
            const result = await invoke('get_peer_count');
            console.log('Invoke test result:', result);
            return result;
        } catch (error) {
            console.error('Invoke test error:', error);
            return error;
        }
    } else {
        console.error('Invoke not available');
        return 'Invoke not available';
    }
};

window.getConnectionStatus = () => ({
    webSocket: ws ? ws.readyState : 'Not initialized',
    tauri: !!invoke,
    status: statusText.textContent
});