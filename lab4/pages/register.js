import React, { useState, useContext } from "react";
import Link from "next/link";
import axios from "axios";
import { useRouter } from "next/router";
import { AuthContext } from "./layout";


const Register = () => {
    const [username, setUsername] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");

    const router = useRouter();
    const { setIsAuthenticated, setUserName } = useContext(AuthContext);

    const handleRegister = async (e) => {
        e.preventDefault();

        if (password !== confirmPassword) {
            alert("Passwords do not match");
            return;
        }

        const data = {
            name: username,
            email: email,
            password: password
        }

        try {
            const response = await axios.post('http://127.0.0.1:8080/register', {
                name: data.name,
                email: data.email,
                password: data.password
            });
            

            setIsAuthenticated(true);
            setUserName(username);
            router.push("/chat");
            
        } catch (error) {
            alert("Maybe email do not match");
            console.log("Error register :", error);
        }
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

export default Register;