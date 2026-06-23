const API_BASE = window.location.origin + '/api';
let nickname = '';

const welcomeEl = document.getElementById('welcome');
const chatEl = document.getElementById('chat');
const nicknameInput = document.getElementById('nickname-input');
const enterBtn = document.getElementById('enter-btn');
const messagesEl = document.getElementById('messages');
const messageInput = document.getElementById('message-input');
const sendBtn = document.getElementById('send-btn');

function escapeHtml(str) {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function setNickname() {
  const value = nicknameInput.value.trim();
  if (!value) return;
  nickname = value;
  welcomeEl.hidden = true;
  chatEl.hidden = false;
  loadMessages();
  setInterval(loadMessages, 3000);
}

function renderMessages(messages) {
  const wasAtBottom =
    messagesEl.scrollHeight - messagesEl.scrollTop <= messagesEl.clientHeight + 1;

  messagesEl.innerHTML = messages
    .map(
      ({ sender, content }) => `
    <div class="message">
      <strong>${escapeHtml(sender)}:</strong>
      ${escapeHtml(content)}
    </div>`
    )
    .join('');

  if (wasAtBottom) {
    messagesEl.scrollTop = messagesEl.scrollHeight;
  }
}

async function loadMessages() {
  try {
    const res = await fetch(`${API_BASE}/messages`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    renderMessages(await res.json());
  } catch (err) {
    console.error('Failed to load messages:', err);
  }
}

async function handleSend() {
  const content = messageInput.value.trim();
  if (!content) return;
  try {
    const res = await fetch(`${API_BASE}/message`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ sender: nickname, recipient: 'Everyone', content }),
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}: ${await res.text()}`);
    messageInput.value = '';
    await loadMessages();
  } catch (err) {
    console.error('Failed to send message:', err);
    alert(`Failed to send message: ${err.message}`);
  }
}

enterBtn.addEventListener('click', setNickname);
nicknameInput.addEventListener('keydown', (e) => e.key === 'Enter' && setNickname());
sendBtn.addEventListener('click', handleSend);
messageInput.addEventListener('keydown', (e) => e.key === 'Enter' && handleSend());
