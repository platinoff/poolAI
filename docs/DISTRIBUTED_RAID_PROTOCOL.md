# Distributed RAID Protocol Specification

**Version**: 1.0  
**Date**: 2025-12-25  
**Status**: Draft

---

## Overview

The Distributed RAID Protocol defines how nodes in a PoolAI cluster communicate for storage operations, replication, and cluster management.

## Protocol Transport

### Primary: HTTP/HTTPS REST API
- **Base Path**: `/api/v1/raid/distributed`
- **Authentication**: JWT Bearer tokens
- **Content-Type**: `application/json`
- **Encoding**: UTF-8

### Secondary: WebSocket (Real-time Updates)
- **Endpoint**: `/ws/raid/distributed`
- **Authentication**: JWT token in query parameter or header
- **Protocol**: JSON messages over WebSocket

### Optional: gRPC (Future)
- For high-performance scenarios
- Protocol Buffers for message serialization

## Message Format

### JSON Structure

All messages follow this structure:

```json
{
  "type": "message_type",
  "id": "unique_message_id",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "node-uuid",
  "payload": { ... }
}
```

### Message Types

#### 1. PutArtifact

**Purpose**: Replicate an artifact to another node.

**Request**:
```json
{
  "type": "put_artifact",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "source-node-uuid",
  "payload": {
    "artifact_id": "artifact-uuid",
    "source_node": "source-node-uuid",
    "data": "base64-encoded-data",
    "metadata": {
      "name": "library-name",
      "version": "1.0.0",
      "size_bytes": 1024000,
      "checksum": "sha256-hash",
      "created_at": "2025-12-25T12:00:00Z"
    },
    "replication_factor": 3,
    "sync_mode": "sync" | "async"
  }
}
```

**Response**:
```json
{
  "type": "put_artifact_response",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:01Z",
  "node_id": "target-node-uuid",
  "payload": {
    "status": "success" | "error",
    "artifact_id": "artifact-uuid",
    "stored_at": "2025-12-25T12:00:01Z",
    "error": "error message if status is error"
  }
}
```

#### 2. GetArtifact

**Purpose**: Request an artifact from another node.

**Request**:
```json
{
  "type": "get_artifact",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "requesting-node-uuid",
  "payload": {
    "artifact_id": "artifact-uuid",
    "include_data": true | false
  }
}
```

**Response**:
```json
{
  "type": "get_artifact_response",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:01Z",
  "node_id": "source-node-uuid",
  "payload": {
    "status": "success" | "not_found" | "error",
    "artifact_id": "artifact-uuid",
    "metadata": { ... },
    "data": "base64-encoded-data (if include_data=true)",
    "error": "error message if status is error"
  }
}
```

#### 3. DeleteArtifact

**Purpose**: Delete an artifact from a node.

**Request**:
```json
{
  "type": "delete_artifact",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "requesting-node-uuid",
  "payload": {
    "artifact_id": "artifact-uuid",
    "propagate": true | false
  }
}
```

**Response**:
```json
{
  "type": "delete_artifact_response",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:01Z",
  "node_id": "target-node-uuid",
  "payload": {
    "status": "success" | "not_found" | "error",
    "artifact_id": "artifact-uuid",
    "deleted_at": "2025-12-25T12:00:01Z",
    "error": "error message if status is error"
  }
}
```

#### 4. SyncArtifacts

**Purpose**: Synchronize artifacts between nodes.

**Request**:
```json
{
  "type": "sync_artifacts",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "requesting-node-uuid",
  "payload": {
    "last_sync_timestamp": "2025-12-25T11:00:00Z",
    "artifact_ids": ["artifact-uuid-1", "artifact-uuid-2"],
    "direction": "pull" | "push" | "bidirectional"
  }
}
```

**Response**:
```json
{
  "type": "sync_artifacts_response",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:01Z",
  "node_id": "target-node-uuid",
  "payload": {
    "status": "success" | "error",
    "synced_count": 5,
    "missing_artifacts": ["artifact-uuid-1"],
    "conflicts": [],
    "error": "error message if status is error"
  }
}
```

#### 5. HealthCheck

**Purpose**: Check node health status.

**Request**:
```json
{
  "type": "health_check",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "requesting-node-uuid",
  "payload": {}
}
```

**Response**:
```json
{
  "type": "health_check_response",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:01Z",
  "node_id": "target-node-uuid",
  "payload": {
    "status": "healthy" | "degraded" | "unhealthy",
    "uptime_seconds": 3600,
    "storage_used_bytes": 10737418240,
    "storage_total_bytes": 107374182400,
    "artifact_count": 150,
    "raft_role": "leader" | "follower" | "candidate",
    "raft_term": 5,
    "last_heartbeat": "2025-12-25T12:00:00Z"
  }
}
```

#### 6. JoinCluster

**Purpose**: New node joining the cluster.

**Request**:
```json
{
  "type": "join_cluster",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "new-node-uuid",
  "payload": {
    "address": "https://new-node.example.com:8080",
    "node_info": {
      "storage_capacity_bytes": 107374182400,
      "region": "us-east-1",
      "tags": ["storage", "compute"]
    }
  }
}
```

**Response**:
```json
{
  "type": "join_cluster_response",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:01Z",
  "node_id": "cluster-leader-uuid",
  "payload": {
    "status": "accepted" | "rejected",
    "cluster_id": "cluster-uuid",
    "member_nodes": [
      {
        "node_id": "node-uuid-1",
        "address": "https://node1.example.com:8080",
        "role": "leader" | "follower"
      }
    ],
    "error": "error message if status is rejected"
  }
}
```

#### 7. LeaveCluster

**Purpose**: Node leaving the cluster.

**Request**:
```json
{
  "type": "leave_cluster",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "leaving-node-uuid",
  "payload": {
    "reason": "shutdown" | "maintenance" | "error",
    "graceful": true | false
  }
}
```

**Response**:
```json
{
  "type": "leave_cluster_response",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:01Z",
  "node_id": "cluster-leader-uuid",
  "payload": {
    "status": "accepted",
    "replication_complete": true | false,
    "artifacts_moved": 10
  }
}
```

## REST API Endpoints

### Cluster Management

#### `POST /api/v1/raid/distributed/cluster/join`
Join a new node to the cluster.

**Request Body**: `JoinCluster` message payload

**Response**: `JoinClusterResponse` message payload

**Status Codes**:
- `200 OK`: Successfully joined
- `400 Bad Request`: Invalid request
- `403 Forbidden`: Not authorized
- `409 Conflict`: Node already in cluster

#### `POST /api/v1/raid/distributed/cluster/leave`
Remove a node from the cluster.

**Request Body**: `LeaveCluster` message payload

**Response**: `LeaveClusterResponse` message payload

**Status Codes**:
- `200 OK`: Successfully left
- `400 Bad Request`: Invalid request
- `404 Not Found`: Node not in cluster

#### `GET /api/v1/raid/distributed/cluster/nodes`
List all nodes in the cluster.

**Response**:
```json
{
  "nodes": [
    {
      "node_id": "node-uuid",
      "address": "https://node.example.com:8080",
      "role": "leader" | "follower",
      "status": "healthy" | "degraded" | "unhealthy",
      "last_seen": "2025-12-25T12:00:00Z"
    }
  ],
  "cluster_id": "cluster-uuid",
  "leader_id": "leader-node-uuid"
}
```

### Artifact Operations

#### `POST /api/v1/raid/distributed/artifacts/:artifact_id/replicate`
Replicate an artifact to another node.

**Request Body**: `PutArtifact` message payload

**Response**: `PutArtifactResponse` message payload

#### `GET /api/v1/raid/distributed/artifacts/:artifact_id`
Get artifact metadata or data from another node.

**Query Parameters**:
- `include_data` (boolean): Include artifact data in response
- `node_id` (string): Specific node to query (optional)

**Response**: `GetArtifactResponse` message payload

#### `DELETE /api/v1/raid/distributed/artifacts/:artifact_id`
Delete an artifact from a node.

**Request Body**: `DeleteArtifact` message payload

**Response**: `DeleteArtifactResponse` message payload

#### `POST /api/v1/raid/distributed/artifacts/sync`
Synchronize artifacts between nodes.

**Request Body**: `SyncArtifacts` message payload

**Response**: `SyncArtifactsResponse` message payload

### Health & Monitoring

#### `GET /api/v1/raid/distributed/health`
Get cluster health status.

**Response**: `HealthCheckResponse` message payload

#### `GET /api/v1/raid/distributed/health/:node_id`
Get specific node health status.

**Response**: `HealthCheckResponse` message payload

## WebSocket Protocol

### Connection

```
ws://node.example.com:8080/ws/raid/distributed?token=JWT_TOKEN
```

### Message Format

Same JSON message format as REST API.

### Message Flow

1. **Client → Server**: Send message
2. **Server → Client**: Send response
3. **Server → Client**: Push notifications (events)

### Event Notifications

Server can push events to clients:

```json
{
  "type": "event",
  "event_type": "artifact_replicated" | "node_joined" | "node_left" | "sync_complete",
  "timestamp": "2025-12-25T12:00:00Z",
  "payload": { ... }
}
```

## Error Handling

### Error Response Format

```json
{
  "type": "error",
  "id": "msg-uuid",
  "timestamp": "2025-12-25T12:00:00Z",
  "node_id": "node-uuid",
  "payload": {
    "error_code": "ARTIFACT_NOT_FOUND" | "NODE_UNAVAILABLE" | "REPLICATION_FAILED",
    "error_message": "Human-readable error message",
    "details": { ... }
  }
}
```

### Error Codes

- `ARTIFACT_NOT_FOUND`: Artifact does not exist
- `NODE_UNAVAILABLE`: Target node is not available
- `REPLICATION_FAILED`: Replication operation failed
- `INSUFFICIENT_STORAGE`: Not enough storage space
- `AUTHENTICATION_FAILED`: Authentication failed
- `AUTHORIZATION_FAILED`: Insufficient permissions
- `INVALID_REQUEST`: Request format is invalid
- `CLUSTER_FULL`: Cluster has reached maximum nodes
- `RAFT_ERROR`: Raft consensus error

## Security

### Authentication
- All requests require JWT Bearer token
- Token must include node_id claim
- Token expiration: 1 hour (configurable)

### Authorization
- Node-to-node communication: Mutual TLS (mTLS) recommended
- Admin operations: Require Admin role
- Read operations: Require Viewer role or higher
- Write operations: Require Operator role or higher

### Encryption
- HTTPS for all REST API calls
- WSS (WebSocket Secure) for WebSocket connections
- Artifact data encryption at rest (optional)

## Performance Considerations

### Batching
- Multiple artifacts can be replicated in a single request
- Batch size limit: 100 artifacts per request

### Compression
- Artifact data can be compressed (gzip) before transmission
- Compression is optional but recommended for large artifacts

### Timeouts
- Request timeout: 30 seconds (configurable)
- Connection timeout: 10 seconds
- Read timeout: 60 seconds for large artifacts

### Rate Limiting
- Maximum 100 requests per second per node
- Burst limit: 200 requests per second

## Versioning

Protocol version is included in API path:
- Current: `/api/v1/raid/distributed`
- Future: `/api/v2/raid/distributed`

Version negotiation:
- Client sends `X-Protocol-Version: 1.0` header
- Server responds with supported version

## Implementation Notes

1. **Message IDs**: Use UUID v4 for unique message identification
2. **Timestamps**: ISO 8601 format, UTC timezone
3. **Node IDs**: UUID v4 for unique node identification
4. **Artifact IDs**: UUID v4 or deterministic hash
5. **Data Encoding**: Base64 for binary data in JSON
6. **Checksums**: SHA-256 for artifact integrity verification

## Testing

### Unit Tests
- Message serialization/deserialization
- Protocol validation
- Error handling

### Integration Tests
- Node-to-node communication
- Replication scenarios
- Failure recovery

### Load Tests
- Concurrent replication
- Large artifact handling
- Network partition scenarios

---

**Next Steps**:
1. Implement message serialization/deserialization
2. Create protocol client library
3. Add integration tests
4. Performance optimization

