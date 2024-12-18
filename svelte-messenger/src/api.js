const API_BASE = 'http://127.0.0.1:8080';

export async function fetchMessages() {
    const response = await fetch(`${API_BASE}/messages`);
    if (!response.ok) throw new Error('Failed to fetch messages');
    return response.json();
}

export async function sendMessage(sender, content) {
    console.log('Sending request to API:', {
        sender,
        recipient: 'Everyone',
        content,
    });
    const response = await fetch('http://127.0.0.1:8080/message', {
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
        const errorText = await response.text(); // Читаем тело ответа с ошибкой
        throw new Error(`HTTP ${response.status}: ${errorText}`);
    }
}
