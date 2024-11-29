import React, { useState, useContext } from 'react';
import Link from "next/link";
import axios from "axios";
import { useRouter } from "next/router";
import { AuthContext } from "./layout";

const handleLogin = () => { 
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');

    const router = useRouter();
    const { setIsAuthenticated } = useContext(AuthContext);

    const handleLogin = async (e) => {
        e.preventDefault();
        
        const data = {
            email: email,
            password: password
        }

        console.log(data);

        try {
            const response = await axios.post('http://127.0.0.1:8080/login', {
                email: data.email,
                password: data.password
            });
            console.log("Login successful:", response);
            setIsAuthenticated(true);
            router.push("/chat");
            
        } catch (error) {
            alert("Maybe password is wrong");
            console.log("Error login :", error);
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

                <button type="submit">Login</button>
            </form>
            <Link href="/forgot-password" class="extra-nav">Forget password? Reset password</Link>
            
        </section>
    );
};

export default handleLogin;