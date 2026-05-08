import type { Metadata } from 'next';
import './globals.css';

export const metadata: Metadata = {
  title: 'AjoChain | On-Chain Cooperative Savings Protocol',
  description: 'AjoChain is a decentralized protocol digitizing the traditional African rotating savings and credit association (ROSCA) model using Stellar and Soroban smart contracts.',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <nav className="nav-header">
          <a href="/" className="nav-logo">AjoChain</a>
          <div className="nav-links">
            <a href="#about">Protocol</a>
            <a href="#features">Features</a>
            <a href="#ecosystem">Ecosystem</a>
            <a href="/app" className="btn-primary">Launch App</a>
          </div>
        </nav>
        {children}
      </body>
    </html>
  );
}
