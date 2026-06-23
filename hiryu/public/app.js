const API_BASE = window.location.origin + '/api';
const WS_URL = (window.location.protocol === 'https:' ? 'wss:' : 'ws:')
  + '//' + window.location.host + '/api/ws';

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

function formatTime(timestamp) {
  if (!timestamp) return '';
  return new Date(timestamp * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

function buildMessageEl({ sender, content, timestamp }) {
  const div = document.createElement('div');
  div.className = 'message';
  div.innerHTML = `
    <div class="message-header">
      <strong>${escapeHtml(sender)}</strong>
      <span class="timestamp">${formatTime(timestamp)}</span>
    </div>
    <div class="message-content">${escapeHtml(content)}</div>`;
  return div;
}

function scrollToBottom() {
  messagesEl.scrollTop = messagesEl.scrollHeight;
}

function setNickname() {
  const value = nicknameInput.value.trim();
  if (!value) return;
  nickname = value;
  welcomeEl.hidden = true;
  chatEl.hidden = false;
  loadMessages();
  connectWebSocket();
}

async function loadMessages() {
  try {
    const res = await fetch(`${API_BASE}/messages`);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    const messages = await res.json();
    messagesEl.innerHTML = '';
    messages.forEach((msg) => messagesEl.appendChild(buildMessageEl(msg)));
    scrollToBottom();
  } catch (err) {
    console.error('Failed to load messages:', err);
  }
}

function connectWebSocket() {
  const ws = new WebSocket(WS_URL);

  ws.onmessage = (event) => {
    const msg = JSON.parse(event.data);
    messagesEl.appendChild(buildMessageEl(msg));
    scrollToBottom();
  };

  ws.onclose = () => {
    setTimeout(connectWebSocket, 3000);
  };

  ws.onerror = (err) => {
    console.error('WebSocket error:', err);
  };
}

async function handleSend() {
  const content = messageInput.value.trim();
  if (!content) return;
  messageInput.value = '';
  try {
    const res = await fetch(`${API_BASE}/message`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ sender: nickname, recipient: 'Everyone', content }),
    });
    if (!res.ok) throw new Error(`HTTP ${res.status}: ${await res.text()}`);
  } catch (err) {
    console.error('Failed to send message:', err);
    alert(`Failed to send message: ${err.message}`);
  }
}

enterBtn.addEventListener('click', setNickname);
nicknameInput.addEventListener('keydown', (e) => e.key === 'Enter' && setNickname());
sendBtn.addEventListener('click', handleSend);
messageInput.addEventListener('keydown', (e) => e.key === 'Enter' && handleSend());
