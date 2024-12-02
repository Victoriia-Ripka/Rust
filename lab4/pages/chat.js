import React, { useState, useEffect, useCallback, useContext, useRef } from "react";
import { AuthContext } from "./layout";
import axios from "axios";
import { useRouter } from "next/router";

import useWebSocket, { ReadyState } from "react-use-websocket";

const Chat = () => {
  // const { isAuthenticated } = useContext(AuthContext);
  const [message, setMessage] = useState("");
  const [socketUrl, setSocketUrl] = useState('ws://127.0.0.1:8080/ws');
  const [messageHistory, setMessageHistory] = useState([]);
  const { sendMessage, lastMessage, readyState } = useWebSocket(socketUrl, {
    shouldReconnect: () => true,
  });

  // if (!isAuthenticated) {
  //   return <p>You must log in to access the chat.</p>;
  // }

  useEffect(() => {
    if (lastMessage !== null) {
      setMessageHistory((prev) => prev.concat(lastMessage));
    }
  }, [lastMessage]);

  const handleSubmit = useCallback(
    (e) => {
      e.preventDefault();
      if (message.trim() && readyState === ReadyState.OPEN) {
        const payload = {
          sender_id: '1746d3ea-829b-42af-b9ab-5e995c4e816e',
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

      <div className="chat-box">
        {messageHistory.map((msg, index) => (
          <div key={index}>{msg.data}</div>
        ))}
      </div>
      <p>WebSocket connection status: {connectionStatus}</p>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Type your message"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          required
        />
        <button type="submit" disabled={readyState !== ReadyState.OPEN}>Send</button>
      </form>
    </section>
  );
};

export default Chat;