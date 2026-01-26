import { useState, useRef, useEffect } from 'react';
import { useChat } from './hooks/useChat';
import './Chat.css';

function App() {
  const [username, setUsername] = useState<string | null>(null);
  const [room, setRoom] = useState<string | null>(null);
  const [loginUser, setLoginUser] = useState('');
  const [loginRoom, setLoginRoom] = useState('Lobby');
  const [inputText, setInputText] = useState('');
  
  const { messages, isConnected, sendMessage } = useChat(username, room);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    if (loginUser.trim()) {
      setUsername(loginUser);
      setRoom(loginRoom);
    }
  };

  const handleSend = (e: React.FormEvent) => {
    e.preventDefault();
    if (inputText.trim()) {
      sendMessage(inputText);
      setInputText('');
    }
  };

  if (!username) {
    return (
      <div className="login-form">
        <h1>Rust 聊天室</h1>
        <form onSubmit={handleLogin} style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          <input
            type="text"
            placeholder="输入用户名..."
            value={loginUser}
            onChange={(e) => setLoginUser(e.target.value)}
            required
            autoFocus
          />
          <select value={loginRoom} onChange={(e) => setLoginRoom(e.target.value)}>
            <option value="Lobby">大厅 (Lobby)</option>
            <option value="Rust">Rust 频道</option>
            <option value="Axum">Axum 频道</option>
          </select>
          <button type="submit">加入聊天</button>
        </form>
      </div>
    );
  }

  return (
    <div className="chat-container">
      <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h2>
          房间: <span className="room-badge">{room}</span>
        </h2>
        <div>
          <span>用户: {username}</span> | 
          <span style={{ color: isConnected ? 'green' : 'red', marginLeft: '10px' }}>
            {isConnected ? '● 已连接' : '○ 未连接'}
          </span>
        </div>
      </header>

      <div className="messages-list">
        {messages.map((msg, index) => {
          const isSystem = msg.message === 'join' || msg.message === 'leave';
          const isOwn = msg.username === username;
          
          if (isSystem) {
            return (
              <div key={index} className="message-item message-system">
                {msg.username} {msg.message === 'join' ? '加入了聊天室' : '离开了聊天室'}
              </div>
            );
          }

          const messageContent = typeof msg.message === 'object' ? msg.message.message : '';

          return (
            <div 
              key={index} 
              className={`message-item ${isOwn ? 'message-own' : 'message-user'}`}
            >
              {!isOwn && <div className="message-header">{msg.username}</div>}
              <div className="message-body">{messageContent}</div>
              <div style={{ fontSize: '0.7em', color: '#999', marginTop: '4px' }}>
                {new Date(msg.timestamp * 1000).toLocaleTimeString()}
              </div>
            </div>
          );
        })}
        <div ref={messagesEndRef} />
      </div>

      <form className="input-area" onSubmit={handleSend}>
        <input
          type="text"
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          placeholder="输入消息..."
          disabled={!isConnected}
        />
        <button type="submit" disabled={!isConnected || !inputText.trim()}>
          发送
        </button>
      </form>
      <button 
        onClick={() => { setUsername(null); setRoom(null); }} 
        style={{ marginTop: '10px', background: 'none', border: 'none', color: '#666', cursor: 'pointer', fontSize: '0.8em' }}
      >
        退出聊天室
      </button>
    </div>
  );
}

export default App;
