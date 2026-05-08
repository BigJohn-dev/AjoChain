"use client";

import { Canvas, useFrame } from '@react-three/fiber';
import { ScrollControls, Scroll, useScroll, Float, Stars, Environment, Lightformer } from '@react-three/drei';
import { useRef } from 'react';
import * as THREE from 'three';

// A dynamic shape that rotates and changes based on scroll
function FloatingCrystal() {
  const meshRef = useRef<THREE.Mesh>(null);
  const scroll = useScroll();

  useFrame((state) => {
    if (!meshRef.current) return;
    const offset = scroll.offset; // 0 to 1
    const time = performance.now() / 1000;
    
    // Rotate object based on scroll and performance timer
    meshRef.current.rotation.x = time * 0.2 + offset * Math.PI * 2;
    meshRef.current.rotation.y = time * 0.3 + offset * Math.PI * 4;
    
    // Move object based on scroll
    // Start at center right
    // Scroll 0.5: Move to center left
    // Scroll 1.0: Move to center
    const targetX = THREE.MathUtils.lerp(
      THREE.MathUtils.lerp(3, -3, Math.min(offset * 2, 1)),
      0,
      Math.max((offset - 0.5) * 2, 0)
    );
    
    meshRef.current.position.x = THREE.MathUtils.lerp(meshRef.current.position.x, targetX, 0.1);
  });

  return (
    <Float speed={2} rotationIntensity={1} floatIntensity={2}>
      <mesh ref={meshRef} position={[3, 0, 0]} scale={1.5}>
        <octahedronGeometry args={[1, 0]} />
        <meshPhysicalMaterial
          color="#fbbf24"
          emissive="#d97706"
          emissiveIntensity={0.5}
          clearcoat={1}
          clearcoatRoughness={0.1}
          metalness={0.8}
          roughness={0.2}
        />
      </mesh>
    </Float>
  );
}

// Background elements
function BackgroundParticles() {
  const groupRef = useRef<THREE.Group>(null);
  const scroll = useScroll();

  useFrame(() => {
    if (!groupRef.current) return;
    groupRef.current.position.y = scroll.offset * 10;
  });

  return (
    <group ref={groupRef}>
      <Stars radius={100} depth={50} count={5000} factor={4} saturation={0} fade speed={1} />
    </group>
  );
}

export default function Scene() {
  return (
    <div id="canvas-container">
      <Canvas camera={{ position: [0, 0, 8], fov: 45 }} dpr={[1, 2]}>
        <color attach="background" args={['#030508']} />
        
        <ambientLight intensity={0.2} />
        <spotLight position={[10, 10, 10]} angle={0.15} penumbra={1} intensity={1} color="#10b981" />
        <pointLight position={[-10, -10, -10]} intensity={0.5} color="#fbbf24" />
        
        <Environment resolution={256}>
          <group rotation={[-Math.PI / 4, -0.3, 0]}>
            <Lightformer intensity={4} rotation-x={Math.PI / 2} position={[0, 5, -9]} scale={[10, 10, 1]} />
            <Lightformer intensity={2} rotation-y={Math.PI / 2} position={[-5, 1, -1]} scale={[10, 2, 1]} />
            <Lightformer intensity={2} rotation-y={-Math.PI / 2} position={[10, 1, 0]} scale={[20, 2, 1]} color="#10b981" />
          </group>
        </Environment>

        <ScrollControls pages={3} damping={0.2}>
          <BackgroundParticles />
          <FloatingCrystal />
          
          <Scroll html style={{ width: '100%' }}>
            <div className="scroll-html-container">
              
              <section className="section" id="hero">
                <div className="section-content glass" style={{ padding: '48px' }}>
                  <h1 className="hero-title">
                    Trustless <br />
                    <span className="gradient-text">Cooperative</span><br />
                    Savings.
                  </h1>
                  <p className="hero-subtitle">
                    AjoChain modernizes the traditional African ROSCA (Esusu) model using Stellar and Soroban. No middlemen. Just code and community.
                  </p>
                  <div className="action-group">
                    <a href="/app" className="btn-primary">Start Saving</a>
                    <a href="#about" className="btn-secondary">Explore Protocol</a>
                  </div>
                </div>
              </section>

              <section className="section section-right" id="about">
                <div className="section-content glass" style={{ padding: '48px' }}>
                  <h2 className="hero-title" style={{ fontSize: '3rem' }}>
                    Built on <br/>
                    <span className="gradient-text">Soroban</span>
                  </h2>
                  <p className="hero-subtitle" style={{ marginBottom: '24px' }}>
                    Replacing the human coordinator with immutable smart contracts.
                  </p>
                  <ul style={{ listStyle: 'none', display: 'flex', flexDirection: 'column', gap: '16px' }}>
                    <li style={{ fontSize: '1.1rem', display: 'flex', alignItems: 'center', gap: '12px' }}>
                      <span style={{ color: '#10b981' }}>✔</span> 150% Collateral Vault to prevent defaults
                    </li>
                    <li style={{ fontSize: '1.1rem', display: 'flex', alignItems: 'center', gap: '12px' }}>
                      <span style={{ color: '#10b981' }}>✔</span> Automated payout distributions
                    </li>
                    <li style={{ fontSize: '1.1rem', display: 'flex', alignItems: 'center', gap: '12px' }}>
                      <span style={{ color: '#10b981' }}>✔</span> Near-zero transaction fees on Stellar
                    </li>
                  </ul>
                </div>
              </section>

              <section className="section section-center" id="features">
                <div className="section-content" style={{ maxWidth: '1200px' }}>
                  <h2 className="hero-title" style={{ fontSize: '3.5rem', textAlign: 'center' }}>
                    Protocol <span className="gradient-text">Features</span>
                  </h2>
                  
                  <div className="feature-grid">
                    <div className="feature-card glass">
                      <div className="feature-icon">🛡️</div>
                      <h3 className="feature-title">Secure Vaults</h3>
                      <p className="feature-desc">Collateral deposits ensure members remain committed to the cycle, eliminating rational defaults.</p>
                    </div>
                    <div className="feature-card glass">
                      <div className="feature-icon">🎲</div>
                      <h3 className="feature-title">Fair Payouts</h3>
                      <p className="feature-desc">Choose from fixed rotation, deterministic randomness, or auction-based priority for payouts.</p>
                    </div>
                    <div className="feature-card glass">
                      <div className="feature-icon">⭐</div>
                      <h3 className="feature-title">On-Chain Reputation</h3>
                      <p className="feature-desc">Earn trust scores based on consistent participation, unlocking premium Diamond pools.</p>
                    </div>
                  </div>
                </div>
              </section>

            </div>
          </Scroll>
        </ScrollControls>
      </Canvas>
    </div>
  );
}
