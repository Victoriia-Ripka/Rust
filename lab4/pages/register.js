import React, { useState } from "react";
import Link from "next/link";

const handleRegister = () => {
    const [username, setUsername] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");

    const handleRegister = async (e) => {
        e.preventDefault();

        if (password!== confirmPassword) {
            alert("Passwords do not match");
            return;
        }

        const data = {
            name: username,
            email: email,
            password: password
        }

        console.log(data);
    };

    return (
        <section>
            <h2>Register</h2>

            <form onSubmit={handleRegister}>
                <label>
                    Name:
                <input type="text" placeholder="Name" value={username} onChange={(e) => setUsername(e.target.value)} required/>
                </label>
                <label>
                    Email:
                <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} required />
                </label>
                <label>
                    Password:
                <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} required />
                </label>
                <label>
                    Password:
                    <input type="password" placeholder="Confirm Password" value={confirmPassword} onChange={(e) => setConfirmPassword(e.target.value)} required />
                </label>

                <button type="submit">Register</button>
            </form>

            <Link href="/login" class="extra-nav">Already have an account? Login</Link>
        </section>
    )
}

export default handleRegister;