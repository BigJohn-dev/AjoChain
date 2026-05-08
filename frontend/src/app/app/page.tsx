export default function AppDashboard() {
  return (
    <div style={{ padding: '100px 10vw', textAlign: 'center' }}>
      <h1 className="hero-title" style={{ fontSize: '3rem' }}>
        App <span className="gradient-text">Dashboard</span>
      </h1>
      <p className="hero-subtitle">
        Connect your wallet to get started. Integration coming soon!
      </p>
      <a href="/" className="btn-secondary">Back to Home</a>
    </div>
  );
}
