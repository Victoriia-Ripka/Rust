import '../styles/global.css';
import RootLayout from './layout';

export default function App({ Component, pageProps }) {
    return (
        <RootLayout>
            <Component {...pageProps} />
        </RootLayout>
    );
}