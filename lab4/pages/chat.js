import React, { useState, useContext } from "react";
import { AuthContext } from "./layout";

const Chat = () => {
  const [message, setMessage] = useState("");
  const [messages, setMessages] = useState([]);

  const { isAuthenticated } = useContext(AuthContext);

  if (!isAuthenticated) {
    return <p>You must log in to access the chat.</p>;
  }

  const sendMessage = async (e) => {
    e.preventDefault();
    setMessages((prev) => [...prev, message]);
    setMessage("");
  };

  return (
    <section>
      <h2>Online Chat</h2>
      <div className="chat-box">
        {messages.map((msg, index) => (
          <div key={index}>{msg}</div>
        ))}
      </div>
      <form onSubmit={sendMessage}>
        <input
          type="text"
          placeholder="Type your message"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          required
        />
        <button type="submit">Send</button>
      </form>
    </section>
  );
};

export default Chat;
