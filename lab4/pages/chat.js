import React, { useState, useEffect, useCallback, useContext, useRef } from "react";
import axios from "axios";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { useAuth } from "./layout";

const Chat = () => {
  const { isAuthenticated, userName } = useAuth();

  const [message, setMessage] = useState("");
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

  const handleSubmit = useCallback(
    (e) => {
      e.preventDefault();
      if (message.trim() && readyState === ReadyState.OPEN) {
        const payload = {
          sender: userName,
          text: message.trim(),
        };
        sendMessage(JSON.stringify(payload));
        setMessage(""); 
      }
    },
    [message, sendMessage, readyState]
  );

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
            </p>
          ))}
      </div>

      {/* <p>WebSocket connection status: {connectionStatus}</p> */}
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Type your message"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          required
        />
        <button type="submit" disabled={readyState !== ReadyState.OPEN} className="chat-btn">Send</button>
      </form>
    </section>
  );
};

export default Chat;