import Link from "next/link";

const RootLayout = ({ children }) => {
  return (
    <>
      <header>
        <div>
          <h1>Welcome to My Chat App</h1>
          <p>Connect with others and enjoy real-time messaging.</p>
        </div>
        <nav className="navigation">
          <ul>
            <li>
              <Link href="/register">Register</Link>
            </li>
            <li>
              <Link href="/login">Login</Link>
            </li>
            <li>
              <Link href="/forgot-password">Forgot Password</Link>
            </li>
            <li>
              <Link href="/chat">Online Chat</Link>
            </li>
          </ul>
        </nav>
      </header>

      <main className="main-body">
        <h1>Online chat web site</h1>
        {children}
      </main>

      <footer>
        <p>This site and its contents are the property of Viktoriia Nowotka</p>
      </footer>
    </>
  );
};

export default RootLayout;
