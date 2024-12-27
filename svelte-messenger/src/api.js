const API_BASE = __API_URL__;

export async function fetchMessages() {
    console.log(`From local: ${API_BASE}`);
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
