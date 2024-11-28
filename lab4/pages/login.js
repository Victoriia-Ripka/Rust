import React, { useState } from 'react';
import Link from "next/link";

const handleLogin = () => { 
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');

    const handleLogin = async (e) => {
        e.preventDefault();
        
        const data = {
            email: email,
            password: password
        }

        console.log(data);

    };

    return (
        <section>
            <h2>Login</h2>
        
            <form onSubmit={handleLogin}>
                <label>
                    Email:
                    <input type="email" placeholder="Email" value={email} onChange={(e) => setEmail(e.target.value)} required />
                </label>
                <label>
                    Password:
                    <input type="password" placeholder="Password" value={password} onChange={(e) => setPassword(e.target.value)} required />
                </label>

                <button type="submit">Login</button>
            </form>
            <Link href="/forgot-password" class="extra-nav">Forget password? Login</Link>
            
        </section>
    );
};

export default handleLogin;