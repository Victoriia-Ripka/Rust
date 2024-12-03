import React, { useState, createContext, useContext  } from "react";
import Link from "next/link";
import { useRouter } from "next/router";

export const AuthContext = createContext();

const RootLayout = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [userName, setUserName] = useState('');

    return (
    <AuthContext.Provider value={{ isAuthenticated, setIsAuthenticated, userName, setUserName }}>
      <div className="layout">
        <header>
          <h1>Online chat web site</h1>
          <nav className="navigation">
            <ul>
              {!isAuthenticated && (
                <>
                  <li>
                    <Link href="/register">Register</Link>
                  </li>
                  <li>
                    <Link href="/login">Login</Link>
                  </li>
                  <li>
                    <Link href="/forgot-password">Forgot Password</Link>
                  </li>
                </>
              )}
              {isAuthenticated && (
                <li>
                  <Link href="/chat">Online Chat</Link>
                </li>
              )}
              
            </ul>
          </nav>
        </header>

        <main className="main-body">{children}</main>

        <footer>
          <p>This site and its contents are the property of Viktoriia Nowotka</p>
        </footer>
      </div>
    </AuthContext.Provider>
  );
};

export default RootLayout;

export const useAuth = () => useContext(AuthContext);