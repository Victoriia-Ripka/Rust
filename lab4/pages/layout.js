import Link from "next/link";

const RootLayout = ({ children }) => {
  return (
    <div className="layout">
      <header>
        <h1>Online chat web site</h1>
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
        {children}
      </main>

      <footer>
        <p>This site and its contents are the property of Viktoriia Nowotka</p>
      </footer>
    </div>
  );
};

export default RootLayout;
