const API_URL = import.meta.env.VITE_API_URL;
const API_BASE = 'http://0.0.0.0:8080';

export async function fetchMessages() {
    const response = await fetch(`${API_BASE}/messages`);
    if (!response.ok) throw new Error('Failed to fetch messages');
    console.log(response);
    return response.json();
}

export async function sendMessage(sender, content) {
    console.log('Sending request to API:', {
        sender,
        recipient: 'Everyone',
        content,
    });
    const response = await fetch(`${API_BASE}/message`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            sender,
            recipient: 'Everyone',
            content,
        }),
    });

    if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`HTTP ${response.status}: ${errorText}`);
    }
}
