[![Build and Push Docker Images](https://github.com/Tempestmon/couperose/actions/workflows/build-and-push.yml/badge.svg?branch=master)](https://github.com/Tempestmon/couperose/actions/workflows/build-and-push.yml)
# couperose
An experimental messenger
Available by https://messenger.tempestmon.ru

# SPEC-1: Messenger Service with Diverse Databases and Observability

## Background

The messenger service is designed to allow users to communicate seamlessly while also serving as a platform to practice and enhance architectural and development skills. The project emphasizes diversity in database usage, observability, and scalability while leveraging the performance and safety of Rust for most components.

The system will support functionalities such as user messaging, file sharing, and system observability (metrics, logging, and tracing). The use of diverse databases enables exploration of various data storage paradigms and their use cases.

## Requirements

**Functional Requirements**:
- Must-Have:
  - Allow users to send text messages to other users.
  - Support group chats.
  - Persist user profiles and chat histories.
  - Support file/image sharing.
  - Provide metrics, logs, and tracing for system observability.

* Should-Have:
  * Support search across chat histories.
  * Include typing indicators and read receipts.
  * Real-time communication via gRPC or WebSocket.
  * Scale horizontally to handle increasing user loads.
* Could-Have:
  * Real-time notifications for new messages.
  * Support rich media like videos or GIFs.
  * Integration with other services (e.g., bot frameworks).

**Non-Functional Requirements**:
- Ensure low latency for messaging (&lt;200ms for message delivery).
- Support millions of users with a scalable architecture.
- Ensure message delivery reliability (eventual consistency acceptable).
- Use diverse databases effectively:
  - Redis for caching and ephemeral data.
  - Postgres for structured data (e.g., user profiles).
  - Cassandra for chat history.
  - MongoDB for JSON-like or flexible structures.
  - Neo4j for social graph modeling (e.g., contacts, group memberships).
  - Prometheus/VictoriaMetrics for metrics storage and querying.
  - S3-compatible storage for files/images.
- Advanced observability with:
  - Distributed tracing (OpenTelemetry).
  - Detailed metrics.
  - Centralized structured logging.

## Method

### Architecture Overview

The system will adopt a ***modular microservices architecture*** to support scalability and flexibility. Each component will handle specific responsibilities, enabling future enhancements without disrupting existing functionality. The architecture will include:

```plantuml
@startuml
package "Frontend" {
  component "Web/Mobile Client" as Client
}

package "Backend" {
  component "API Gateway" as Gateway
  component "Messaging Service" as Messaging
  component "User Management Service" as UserService
  component "Observability Service" as Observability
}

package "Data Storage" {
  database "Postgres (User Data)" as Postgres
  database "Cassandra (Chat History)" as Cassandra
  database "Redis (Cache)" as Redis
  database "Prometheus/VictoriaMetrics (Metrics)" as MetricsDB
  storage "S3-Compatible (File Storage)" as S3
}

Client --> Gateway : HTTP
Gateway --> Messaging : gRPC
Gateway --> UserService : gRPC
Messaging --> Cassandra : Store Chat History
UserService --> Postgres : Manage User Data
Messaging --> Redis : Cache Active Sessions
Observability --> MetricsDB : Store Metrics
Observability --> Logs : Centralized Logging
Observability --> Traces : Distributed Tracing
@enduml
```

### Components

1. ***Frontend***:
   * A thin client (e.g., web or mobile app) communicates with the backend via ***HTTP***.
   * Supports features like authentication, sending/receiving messages, and real-time updates.
2. ***gRPC API Gateway***:
   * Serves as the single entry point for all client requests.
   * Handles routing to specific services and manages authentication tokens.
   * Written in Rust with libraries like `tonic` (gRPC framework for Rust).
3. ***Messaging Service***:
   * Core service handling message delivery, storage, and real-time communication.
   * Ensures low-latency messaging with session management using Redis.
   * Stores chat history in Cassandra for scalability and fast retrieval.
4. ***User Management Service***:
   * Handles user authentication, profile management, and contact lists.
   * Uses Postgres for structured user data.
5. ***Observability Service***:
   * Aggregates metrics, logs, and traces for the entire system.
   * Integrates with Prometheus/VictoriaMetrics for metrics and OpenTelemetry for distributed tracing.
   * Provides dashboards and alerts.
6. ***Data Storage***:
   * ***Postgres***: Structured user data.
   * ***Cassandra***: Scalable storage for chat history.
   * ***Redis***: Low-latency cache for ephemeral data.
   * ***S3-compatible storage***: File/image storage (future feature).
   * ***Prometheus/VictoriaMetrics***: Metrics storage and querying.
