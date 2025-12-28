# Raft Consensus Library Evaluation

**Date**: 2025-12-25  
**Purpose**: Evaluate Rust Raft libraries for Distributed RAID implementation  
**Status**: In Progress

---

## Requirements

### Functional Requirements
- ✅ Async/await support (Tokio runtime)
- ✅ Leader election
- ✅ Log replication
- ✅ Membership changes (add/remove nodes)
- ✅ Snapshot support (for state recovery)
- ✅ Network transport abstraction (HTTP/HTTPS)

### Non-Functional Requirements
- ✅ Windows-gnu compatibility (no native toolchain surprises)
- ✅ Active maintenance
- ✅ Good documentation
- ✅ Production-ready stability
- ✅ Reasonable performance

---

## Candidate Libraries

### 1. async-raft

**Repository**: https://github.com/async-raft/async-raft  
**Version**: Latest stable  
**License**: Apache-2.0 / MIT

#### Pros
- ✅ **Modern async/await** - Built for Tokio
- ✅ **Active maintenance** - Regular updates
- ✅ **Well-documented** - Comprehensive docs
- ✅ **Transport abstraction** - Can use HTTP/HTTPS
- ✅ **Production-ready** - Used in real projects
- ✅ **Snapshot support** - For state recovery
- ✅ **Membership changes** - Dynamic cluster management

#### Cons
- ⚠️ **Learning curve** - More complex API
- ⚠️ **Dependencies** - May have transitive deps

#### Evaluation
**Rating**: ⭐⭐⭐⭐⭐ (5/5)  
**Recommendation**: **PRIMARY CHOICE**

**Rationale**: 
- Perfect fit for our async/await architecture
- Actively maintained and production-ready
- Good documentation and examples
- Transport abstraction allows HTTP/HTTPS integration

---

### 2. raft-rs

**Repository**: https://github.com/tikv/raft-rs  
**Version**: Latest stable  
**License**: Apache-2.0

#### Pros
- ✅ **Mature** - Used in TiKV (production system)
- ✅ **Battle-tested** - Large-scale deployments
- ✅ **Performance** - Highly optimized

#### Cons
- ❌ **Synchronous API** - Not async/await native
- ❌ **TiKV-specific** - Tightly coupled to TiKV
- ❌ **Complex integration** - Requires more work for HTTP transport
- ⚠️ **Less flexible** - Harder to adapt to our use case

#### Evaluation
**Rating**: ⭐⭐⭐ (3/5)  
**Recommendation**: **NOT RECOMMENDED**

**Rationale**:
- Designed for TiKV, not general-purpose
- Synchronous API doesn't fit our async architecture
- Would require significant adaptation work

---

### 3. Other Options

#### raft (by pingcap)
- Similar to raft-rs, TiKV-specific
- Not recommended for our use case

#### Custom Implementation
- Too much work
- Risk of bugs
- Not recommended

---

## Decision

### Selected Library: **async-raft**

**Version**: Latest stable (to be determined)  
**Integration Strategy**:
1. Add `async-raft` as optional dependency (feature flag `raft`)
2. Create Raft network transport using HTTP/HTTPS
3. Integrate with existing RaidManager
4. Implement leader election and log replication
5. Add membership change support

### Implementation Plan

#### Phase 1: Setup (Week 11, Day 1)
- [x] Add `async-raft` dependency to `Cargo.toml`
- [x] Create Raft network transport module (`raft_transport.rs`)
- [x] Define Raft state machine for RAID operations (`raft.rs`)
- [x] Basic Raft node setup

#### Phase 2: Integration (Week 11, Day 2-3)
- [x] Integrate Raft with RaidManager (basic structure created)
- [x] Create RaidRaftStorage structure
- [x] Create RaidRaftStateMachine with apply_operation method
- [x] Update RaidRaftNode with storage, state_machine, and transport
- [x] Add apply_operation method to RaidRaftNode
- [x] Add transport() method for node management
- [x] Integration tests (5 tests passing)
- [x] Implement RaftStorage trait
- [x] Implement RaftNetwork trait
- [x] Initialize Raft instance in RaidRaftNode
- [x] Basic leader election support (automatic for single-node clusters)
- [x] Wait for leader election method
- [x] Integration tests виправлено (5 tests passing)
- [ ] Multi-node leader election testing
- [ ] Basic log replication testing
- [ ] Multi-node cluster integration tests

#### Phase 3: Testing (Week 11, Day 4-5)
- [ ] Integration tests
- [ ] Multi-node cluster tests
- [ ] Failure scenario tests
- [ ] Performance benchmarks

---

## Dependencies

### Required
```toml
async-raft = { version = "0.7", optional = true }
```

### Optional (for advanced features)
```toml
# May be needed for snapshot compression
bincode = { version = "1.3", optional = true }
```

---

## Next Steps

1. ✅ **Evaluation Complete** - async-raft selected
2. 🔄 **Add Dependency** - Add async-raft to Cargo.toml
3. 🔄 **Create Transport** - HTTP/HTTPS transport for Raft
4. 🔄 **Integrate** - Connect Raft to RaidManager
5. 🔄 **Test** - Comprehensive testing

---

**Prepared by**: Rust Architect  
**Date**: 2025-12-25  
**Status**: Evaluation Complete - Ready for Implementation

