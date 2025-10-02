// Tauri API references
let invoke, appWindow, fs, dialog, notification, os;

const statusText = document.getElementById('status-text');
const textarea = document.getElementById('main-text');
const copyBtn = document.getElementById('copy-btn');

function setStatus(text, color = '#3182ce') {
    statusText.textContent = text;
    statusText.style.color = color;
    console.log(`Status: ${text}`);
}

// Copy button functionality
async function copyText() {
    if (!textarea || !textarea.value.trim()) {
        console.log('No text to copy');
        return;
    }
    
    try {
        await navigator.clipboard.writeText(textarea.value);
        
        // Visual feedback
        copyBtn.classList.add('copied');
        copyBtn.innerHTML = `
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20,6 9,17 4,12"></polyline>
            </svg>
        `;
        
        // Reset after 2 seconds
        setTimeout(() => {
            copyBtn.classList.remove('copied');
            copyBtn.innerHTML = `
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                </svg>
            `;
        }, 2000);
        
        console.log('Text copied to clipboard');
    } catch (error) {
        console.error('Failed to copy text:', error);
    }
}

// User-friendly status messages
function setUserStatus(results) {
    console.log('Debug info:', results);
    
    if (results.peerCount > 0) {
        setStatus(`Connected to ${results.peerCount} peer(s)`, '#38a169');
    } else {
        setStatus('Searching for peers...', '#3182ce');
    }
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
    
    // Log debug information to console
    console.log('Peer ID:', results.peerId || 'Not available');
    console.log('Peer Count:', results.peerCount.toString());
    
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
        
        console.log('Discovered Peers:', peerList);
    } else {
        console.log('No peers discovered');
    }
    
    // Update user-friendly status
    setUserStatus(results);
}

// Text area event handler
function setupTextAreaHandler() {
    if (!textarea) {
        console.error('Textarea not found');
        return;
    }
    
    let lastText = '';
    textarea.addEventListener('input', async () => {
        const currentText = textarea.value;
        
        // Only send if text has changed and we have Tauri APIs
        if (currentText !== lastText && invoke) {
            lastText = currentText;
            
            try {
                // Send text to all discovered peers
                await invoke('send_text_to_all_peers', { text: currentText });
                console.log('Sent text to all peers:', currentText);
                
                // Check if there are any peers to send to
                const peerCount = await invoke('get_peer_count');
                if (peerCount === 0) {
                    console.log('No peers available - text not sent');
                    setStatus('No peers available - text not sent', '#e53e3e');
                } else {
                    setStatus(`Text sent to ${peerCount} peer(s)`, '#38a169');
                }
            } catch (error) {
                console.error('Failed to send text:', error);
                const errorMessage = (error && (error.message || error.toString())) || 'Failed to send text';
                setStatus(errorMessage, '#e53e3e');
            }
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
        
        // Setup copy button
        if (copyBtn) {
            copyBtn.addEventListener('click', copyText);
        }

        // Simple event listener setup with longer delay
        setTimeout(() => {
            console.log('Setting up event listener...');
            console.log('Tauri object available:', !!window.__TAURI__);
            console.log('Event object available:', !!(window.__TAURI__ && window.__TAURI__.event));
            
            if (window.__TAURI__ && window.__TAURI__.event) {
                console.log('Available event methods:', Object.keys(window.__TAURI__.event));
                
                // Try the standard approach first
                if (window.__TAURI__.event.listen) {
                    try {
                        window.__TAURI__.event.listen('text-received', (event) => {
                            console.log('Received text from peer:', event.payload);
                            if (textarea) textarea.value = event.payload;
                        });
                        console.log('Event listener set up successfully');
                    } catch (error) {
                        console.error('Failed to set up event listener:', error);
                    }
                } else {
                    console.warn('event.listen method not available');
                }
            } else {
                console.warn('Tauri event API not available');
            }
        }, 5000); // Wait 5 seconds
        
        // Text sharing is now handled via UDP
        setStatus('Ready for text sharing...');
        
        // Test Tauri commands after initialization
        setTimeout(() => {
            console.log('Testing Tauri commands...');
            debugTauriCommands();
        }, 1000);
        
        // Set up periodic updates
        setInterval(debugTauriCommands, 2000);
        
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