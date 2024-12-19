import React, { useState, useEffect, useCallback, useContext, useRef } from "react";
import axios from "axios";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { useAuth } from "./layout";

const Chat = () => {
  const { isAuthenticated, userName } = useAuth();

  const [message, setMessage] = useState("");
  const [file, setFile] = useState(null);
  const [socketUrl, setSocketUrl] = useState('ws://127.0.0.1:8080/ws');
  const [messageHistory, setMessageHistory] = useState([]);
  const { sendMessage, lastMessage, readyState } = useWebSocket(socketUrl, {
    shouldReconnect: () => true,
  });

  if (!isAuthenticated) {
    return <p>You must log in to access the chat.</p>;
  }

  useEffect(() => {
    const fetchMessages = async () => {
      try {
        const response = await axios.get('http://127.0.0.1:8080/messages');
        setMessageHistory(response.data);
      } catch (error) {
        console.error("Error fetching message history:", error);
      }
    };

    fetchMessages();
  }, []);

  useEffect(() => {
    if (lastMessage !== null) {
      try {
        const parsedMessage = JSON.parse(lastMessage.data); 
        setMessageHistory((prev) => [...prev, parsedMessage]);
      } catch (error) {
        console.error('Error parsing WebSocket message:', error);
      }
    }
  }, [lastMessage]);

  const handleSubmit = async (e) => {
    e.preventDefault();

    if (!message.trim() && !file) return;

    let fileUrl = null;

    if (file) {
      const formData = new FormData();
      formData.append("file", file);

      try {
        const response = await axios.post("http://127.0.0.1:8080/upload", formData);
        console.log(response.data);
        fileUrl = response.data.fileUrl;
      } catch (error) {
        console.error("File upload error:", error);
        return;
      }
    }

    console.log(fileUrl)
    const timestamp = new Date().toISOString();

    const payload = {
      sender: userName,
      text: message.trim(),
      fileUrl,
      timestamp 
    };

    if (readyState === ReadyState.OPEN) {
      sendMessage(JSON.stringify(payload));
      setMessage(""); 
      setFile(null); 
    }
  };

  const connectionStatus = {
    [ReadyState.CONNECTING]: 'Connecting',
    [ReadyState.OPEN]: 'Open',
    [ReadyState.CLOSING]: 'Closing',
    [ReadyState.CLOSED]: 'Closed',
    [ReadyState.UNINSTANTIATED]: 'Uninstantiated',
  }[readyState];

  return (
    <section>
      <h2>Online Chat</h2>

      <div className="msgs-box">
        {messageHistory.map((msg) => (
          <p key={msg.id}>
            <strong>{msg.sender}</strong>: {msg.text}
            {msg.file_url && (
              <a href={msg.file_url} target="_blank" rel="noopener noreferrer">
                View File
              </a>
            )}
          </p>
        ))}
      </div>

      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Type your message"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
        />
        <input
          type="file"
          onChange={(e) => setFile(e.target.files[0])}
        />
        <button type="submit" disabled={readyState !== ReadyState.OPEN}>
          Send
        </button>
      </form>

    </section>
  );
};

export default Chat;