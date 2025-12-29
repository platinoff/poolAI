# ADR-001: Distributed RAID (BurstRAID/SmallWorld) Architecture

**Status**: Proposed  
**Date**: 2025-12-25  
**Deciders**: PoolAI Architecture Team  
**Context**: Need for distributed storage with fault tolerance and replication

---

## Context

PoolAI currently has a local RAID module that provides:
- Local artifact storage
- Garbage collection
- Quota management
- Basic node registry

However, for production deployment and scalability, we need:
- **Distributed storage** across multiple nodes
- **Fault tolerance** (survive node failures)
- **Data replication** for reliability
- **Consistency** across distributed nodes
- **Auditability** for compliance

## Decision

We will implement a **Distributed RAID** system (internally called "BurstRAID/SmallWorld") that provides:

1. **Distributed Storage Protocol** - Communication between nodes
2. **Raft Consensus** - For consistency and leader election
3. **Event Sourcing** - For auditability and state reconstruction
4. **Circuit Breaker Pattern** - For fault tolerance
5. **Replication Strategy** - Multi-node data replication

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Distributed RAID Layer                    │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Node A     │  │   Node B     │  │   Node C     │      │
│  │              │  │              │  │              │      │
│  │ ┌──────────┐ │  │ ┌──────────┐ │  │ ┌──────────┐ │      │
│  │ │  Raft    │ │  │ │  Raft    │ │  │ │  Raft    │ │      │
│  │ │ Consensus│ │  │ │ Consensus│ │  │ │ Consensus│ │      │
│  │ └──────────┘ │  │ └──────────┘ │  │ └──────────┘ │      │
│  │              │  │              │  │              │      │
│  │ ┌──────────┐ │  │ ┌──────────┐ │  │ ┌──────────┐ │      │
│  │ │  Event   │ │  │ │  Event   │ │  │ │  Event   │ │      │
│  │ │ Sourcing │ │  │ │ Sourcing │ │  │ │ Sourcing │ │      │
│  │ └──────────┘ │  │ └──────────┘ │  │ └──────────┘ │      │
│  │              │  │              │  │              │      │
│  │ ┌──────────┐ │  │ ┌──────────┐ │  │ ┌──────────┐ │      │
│  │ │ Circuit  │ │  │ │ Circuit  │ │  │ │ Circuit  │ │      │
│  │ │ Breaker  │ │  │ │ Breaker  │ │  │ │ Breaker  │ │      │
│  │ └──────────┘ │  │ └──────────┘ │  │ └──────────┘ │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│         │                  │                  │              │
│         └──────────────────┼──────────────────┘              │
│                            │                                 │
│                    ┌───────▼───────┐                        │
│                    │  Local RAID   │                        │
│                    │   (Storage)   │                        │
│                    └───────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

## Components

### 1. Distributed Storage Protocol

**Purpose**: Define how nodes communicate for storage operations.

**Protocol Messages**:
- `PutArtifact(node_id, artifact_id, data, metadata)` - Replicate artifact
- `GetArtifact(node_id, artifact_id)` - Request artifact from node
- `DeleteArtifact(node_id, artifact_id)` - Delete artifact from node
- `SyncArtifacts(node_id, last_sync_timestamp)` - Synchronize artifacts
- `HealthCheck(node_id)` - Node health status
- `JoinCluster(node_id, address)` - New node joining cluster
- `LeaveCluster(node_id)` - Node leaving cluster

**Transport**: 
- HTTP/HTTPS for REST API
- WebSocket for real-time updates
- gRPC (optional, for high-performance scenarios)

### 2. Raft Consensus

**Purpose**: Ensure consistency across distributed nodes.

**Implementation**:
- Use existing Rust Raft library (e.g., `async-raft` or `raft-rs`)
- Leader election for write operations
- Log replication for consistency
- Membership changes (add/remove nodes)

**Raft Roles**:
- **Leader**: Handles all write operations, replicates to followers
- **Follower**: Receives log entries from leader, votes in elections
- **Candidate**: Temporary state during leader election

**Configuration**:
- Minimum 3 nodes for fault tolerance (survive 1 node failure)
- Recommended 5 nodes for production (survive 2 node failures)
- Quorum = (N/2) + 1 nodes

### 3. Event Sourcing

**Purpose**: Provide auditability and state reconstruction.

**Event Types**:
- `ArtifactCreated(artifact_id, node_id, timestamp, metadata)`
- `ArtifactUpdated(artifact_id, node_id, timestamp, changes)`
- `ArtifactDeleted(artifact_id, node_id, timestamp)`
- `NodeJoined(node_id, address, timestamp)`
- `NodeLeft(node_id, timestamp)`
- `ReplicationStarted(artifact_id, source_node, target_node)`
- `ReplicationCompleted(artifact_id, source_node, target_node)`

**Storage**:
- Append-only event log
- Snapshot for fast recovery
- Event replay for state reconstruction

### 4. Circuit Breaker Pattern

**Purpose**: Prevent cascading failures and improve resilience.

**States**:
- **Closed**: Normal operation, requests pass through
- **Open**: Node is failing, requests are rejected immediately
- **Half-Open**: Testing if node has recovered, limited requests allowed

**Configuration**:
- Failure threshold: 5 consecutive failures
- Timeout: 60 seconds before attempting recovery
- Success threshold: 2 successful requests to close circuit

### 5. Replication Strategy

**Purpose**: Ensure data availability and durability.

**Strategies**:
1. **Synchronous Replication** (for critical data)
   - Wait for quorum confirmation before returning success
   - Higher latency, stronger consistency

2. **Asynchronous Replication** (for non-critical data)
   - Return success immediately, replicate in background
   - Lower latency, eventual consistency

3. **Read Replicas**
   - Distribute read load across nodes
   - Improve read performance

**Replication Factor**: Configurable (default: 3 copies)

## Implementation Plan

### Phase 1: Protocol Design (Week 10)
- [ ] Define message formats (JSON/Protobuf)
- [ ] Design API endpoints
- [ ] Create protocol documentation
- [ ] Unit tests for message serialization

### Phase 2: Raft Integration (Week 11-12)
- [x] Choose Raft library (async-raft 0.6.1)
- [x] Create Raft transport module (HTTP/HTTPS)
- [x] Define Raft state machine structures
- [x] Basic Raft node setup
- [x] Create RaidRaftStorage structure
- [x] Create RaidRaftStateMachine with apply_operation
- [x] Update RaidRaftNode with full integration (storage, state_machine, transport)
- [x] Integrate state machine with RaidManager
- [x] Implement RaftStorage trait
- [x] Implement RaftNetwork trait
- [x] Initialize Raft instance in RaidRaftNode
- [x] Basic leader election support (automatic for single-node clusters)
- [x] Wait for leader election method
- [x] Integration tests (5 tests passing)
- [x] Multi-node leader election testing — ✅ базові тести реалізовано (Week 12)
- [x] Log replication testing — ✅ базові тести реалізовано (Week 12)
- [x] Multi-node cluster integration tests — ✅ базові тести реалізовано (Week 12)
- ℹ️ Примітка: Тести потребують feature flag `--features raft` для запуску

### Phase 3: Event Sourcing (Week 13) — ✅ ЗАВЕРШЕНО
- [x] Event store implementation (`EventStore`, `RaidEvent`, `EventRecord`, `Snapshot`)
- [x] Event replay mechanism (`replay_events`, `replay_events_since_snapshot`)
- [x] Snapshot creation (`create_snapshot`, `load_snapshot`)
- [x] Audit log API (5 REST endpoints: `/raid/events`, `/raid/events/:artifact_id`, `/raid/events/range`, `/raid/snapshot`, `/raid/snapshot/create`)
- [x] Integration tests (8 tests passing)

### Phase 4: Circuit Breaker (Week 14) — ✅ ЗАВЕРШЕНО
- [x] Circuit breaker implementation (`CircuitBreaker`, `CircuitBreakerManager`)
- [x] Integration with ProtocolClient (automatic failure detection)
- [x] Failure detection (consecutive failure counting)
- [x] Recovery mechanism (half-open state with success threshold)
- [x] Integration tests (8 tests passing)

### Phase 5: Replication (Week 15-16) — ✅ ЗАВЕРШЕНО 🎉
- [x] Replication Engine Core (Week 15.1) — ✅ ЗАВЕРШЕНО
  - [x] ReplicationEngine structure
  - [x] Node selection algorithm
  - [x] Replication metadata tracking
  - [x] Unit tests (7 tests passing)
- [x] Synchronous replication (Week 15.2) — ✅ ЗАВЕРШЕНО
  - [x] replicate_sync() method with quorum-based confirmation
  - [x] Protocol client management
  - [x] Timeout handling
  - [x] Error recovery and partial success handling
- [x] Replication events integration (Week 15.3) — ✅ ЗАВЕРШЕНО
  - [x] Emit ReplicationStarted events for each target node
  - [x] Emit ReplicationCompleted events on successful replication
  - [x] Integration with EventStore in replicate_sync()
- [x] Asynchronous replication (Week 16.1) — ✅ ЗАВЕРШЕНО
  - [x] Background replication queue (mpsc channel)
  - [x] Async replication workers (configurable count)
  - [x] Retry mechanism (configurable attempts)
  - [x] Status tracking (Queued status)
- [x] Read replica support (Week 16.2) — ✅ ЗАВЕРШЕНО
  - [x] Read replica selection (health-aware)
  - [x] Load balancing for reads (round-robin)
  - [x] Read consistency levels (Eventual, Quorum, Strong)
  - [x] Health checks for replicas (circuit breaker integration)
- [x] Conflict resolution (Week 16.3) — ✅ ЗАВЕРШЕНО
  - [x] Conflict detection (checksum mismatch, concurrent writes)
  - [x] Last-write-wins strategy
  - [x] First-write-wins strategy
  - [x] Vector clock structure (basic implementation)
  - [x] Manual resolution support
  - [x] sync_with_conflict_resolution() API
- [ ] Integration tests

### Phase 6: Testing & Optimization (Week 17-18) — 🔄 В ПРОЦЕСІ
- [x] Distributed system tests (Phase 6.1) — ✅ ЗАВЕРШЕНО
  - [x] Multi-node synchronous replication tests
  - [x] Quorum-based replication tests
  - [x] Replication metadata tracking tests
  - [x] Node selection algorithm tests
  - [x] Read consistency levels tests
  - [x] Conflict resolution strategies tests
  - [x] 10 tests passing
- [x] Failure scenario tests (Phase 6.2) — ✅ ЗАВЕРШЕНО
  - [x] Quorum availability during failures
  - [x] Circuit breaker failure detection and recovery
  - [x] Replication status on failure
  - [x] Node selection with failed nodes
  - [x] Partial failure handling
  - [x] Read consistency with failures
  - [x] Replication retry on failure
  - [x] Network partition scenarios
  - [x] 9 tests passing
- [x] Performance benchmarks (Phase 6.3) — ✅ ЗАВЕРШЕНО
  - [x] Node selection performance benchmark
  - [x] Quorum calculation performance benchmark
  - [x] Replication metadata operations benchmark
  - [x] Metadata retrieval performance benchmark
  - [x] Node registration performance benchmark
  - [x] Configuration access performance benchmark
  - [x] Consistency level comparison benchmark
  - [x] Conflict resolution strategy comparison benchmark
  - [x] 8 benchmark tests
- [ ] Load testing (Phase 6.4)
- [ ] Documentation

## Dependencies

### External Libraries
- **Raft**: `async-raft` or `raft-rs` (to be evaluated)
- **Event Store**: Custom implementation or `eventstore-rs`
- **Circuit Breaker**: `tower` middleware or custom implementation
- **Serialization**: `serde`, `serde_json`, `prost` (for Protobuf)

### Internal Dependencies
- ✅ Local RAID module (existing)
- ✅ Network module (existing)
- ✅ Core error handling (existing)

## Trade-offs

### Consistency vs Availability
- **Choice**: Strong consistency with Raft
- **Trade-off**: Higher latency, but data integrity is critical
- **Mitigation**: Use async replication for non-critical operations

### Complexity vs Simplicity
- **Choice**: Full distributed system with Raft
- **Trade-off**: More complex, but provides strong guarantees
- **Mitigation**: Phased implementation, comprehensive testing

### Performance vs Safety
- **Choice**: Synchronous replication for critical data
- **Trade-off**: Slower writes, but guaranteed durability
- **Mitigation**: Configurable replication strategy

## Risks

1. **Network Partitions**: Raft handles this, but may cause temporary unavailability
2. **Split-Brain**: Prevented by Raft quorum requirements
3. **Data Loss**: Mitigated by replication and event sourcing
4. **Performance**: May be slower than local storage, but acceptable for distributed system

## Success Criteria

- [ ] Survive single node failure without data loss
- [ ] Maintain consistency across all nodes
- [ ] Provide audit trail for all operations
- [ ] Handle network partitions gracefully
- [ ] Support dynamic node addition/removal
- [ ] Achieve < 100ms latency for local operations
- [ ] Achieve < 500ms latency for distributed operations

## Alternatives Considered

### 1. Simple Master-Slave Replication
- **Rejected**: No automatic failover, single point of failure

### 2. Eventual Consistency (Dynamo-style)
- **Rejected**: Too complex for our use case, consistency is important

### 3. External Distributed Storage (Ceph, GlusterFS)
- **Rejected**: Want to maintain control and integrate with our system

## References

- [Raft Consensus Algorithm](https://raft.github.io/)
- [Event Sourcing Pattern](https://martinfowler.com/eaaDev/EventSourcing.html)
- [Circuit Breaker Pattern](https://martinfowler.com/bliki/CircuitBreaker.html)
- [Distributed Systems Principles](https://www.allthingsdistributed.com/)

---

**Next Steps**: 
1. Review and approve this ADR
2. Begin Phase 1: Protocol Design
3. Create detailed technical specifications

