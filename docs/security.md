# AjoChain — Security Considerations

## Threat Model

### Economic Attacks

#### 1. Rational Default (Post-Payout Disappearance)
**Risk:** A member receives their payout early in the cycle and stops contributing.

**Mitigation:**
- **Collateral Vault:** Members lock 150% of total obligations as collateral on join.
- **Auto-Slashing:** Missed contributions trigger automatic collateral slashing.
- **Reputation Penalty:** -25 reputation points per dispute, affecting future pool access.

#### 2. Sybil Attacks (Multiple Identities)
**Risk:** An attacker creates multiple identities to dominate pool payouts.

**Mitigation:**
- **Collateral Requirement:** Each identity must lock real capital.
- **Reputation Gating:** Higher-tier pools require proven track records.
- **Admin Allowlisting:** Pool creators can restrict membership.

#### 3. Front-Running (Auction Mode)
**Risk:** MEV bots front-run auction bids.

**Mitigation:**
- Stellar's consensus model has no public mempool, significantly reducing front-running risk.
- Bids are recorded per-round and resolved at deadline.

### Smart Contract Risks

#### 4. Reentrancy
**Mitigation:** Soroban's execution model prevents reentrancy by design — each invocation runs to completion before another can begin.

#### 5. Storage Exhaustion
**Mitigation:**
- Bounded member lists (`max_members` cap).
- Temporary storage for per-round data (auto-expired).
- Cursor-based pagination for all list queries.

#### 6. Arithmetic Overflow
**Mitigation:** All arithmetic uses `checked_add`, `checked_sub`, `checked_mul` with explicit `Overflow` error returns.

#### 7. Unauthorised Access
**Mitigation:**
- `require_auth()` on all state-mutating functions.
- Admin-only guards with explicit address comparison.
- Governance timelock on protocol-level changes.

### Governance Risks

#### 8. Hostile Admin
**Mitigation:**
- 48-hour timelock on all governance actions.
- Security Council veto mechanism.
- Transparent on-chain proposal history.

#### 9. Emergency Response
**Mitigation:**
- Circuit breaker for immediate pause.
- Emergency pause bypasses timelock (admin-only).
- Post-pause unpause also requires admin auth.

## Audit Checklist

- [ ] All public functions validate caller authorisation
- [ ] No unbounded loops in production code
- [ ] All arithmetic is overflow-safe
- [ ] Storage keys use appropriate TTL categories
- [ ] Events are emitted for all state changes
- [ ] Error codes are unique and deterministic
- [ ] Collateral ratios are sufficient to prevent rational defaults
- [ ] Timelock delays are enforced before governance execution
- [ ] Council veto can block malicious proposals
