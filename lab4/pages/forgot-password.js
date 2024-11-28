import React, { useState } from "react";

const ForgotPassword = () => {
  const [email, setEmail] = useState("");

  const handleForgotPassword = async (e) => {
      e.preventDefault();
      
    const data = {
      email: email
    }
    
    console.log(data);
      // TODO: Send email to the provided email address with a reset link
    
  };

  return (
    <section>
      <h2>Forgot Password</h2>

      <form onSubmit={handleForgotPassword}>
          <label>
            Email:
            <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} required />
          </label>
            
        <button type="submit">Send Reset Link</button>
      </form>
    </section>
  );
};

export default ForgotPassword;
