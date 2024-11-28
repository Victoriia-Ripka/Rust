import React, { useState } from "react";

const handleRegister = () => {
    const [username, setUsername] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");

    const handleRegister = async (e) => {
        e.preventDefault();
        // Logic to register via your Rust API
    };

    return (
        <section>
            <h2>Register</h2>

            <form onSubmit={handleSubmit}>
                <input type="text" placeholder="Name" value={userName} onChange={(e) => setUsername(e.target.value)} required/>
                <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} required/>
                <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} required/>
                <input type="password" placeholder="Confirm Password" value={confirmPassword} onChange={(e) => setConfirmPassword(e.target.value)} required/>

                <button type="submit">Register</button>
            </form>

            <Link to="/login">Already have an account? Login</Link>
        </section>
    )
}

export default handleRegister;