import React, { useState } from 'react';

const handleLogin = () => { 
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');

    const handleLogin = async (e) => {
        e.preventDefault();
        if (!email || !password) {
            alert('Please enter both email and password.');
            return;
        }

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
                <input type="submit" value="Login" />
            </form>
            
        </section>
    );
};

export default handleLogin;