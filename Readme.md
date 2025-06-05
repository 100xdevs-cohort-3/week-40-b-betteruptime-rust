# 🌐 Website Monitoring App

A **scalable**, **high-performance** website monitoring system inspired by [BetterStack](https://betterstack.com/) and [BetterUptime](https://betteruptime.com), designed to track the status and response times of millions of websites across multiple regions.

---

## 📖 Overview

This system continuously monitors website availability and performance, storing time-series data and sending notifications on downtime events. It is built using a modular architecture in **Rust**, providing efficiency, safety, and scalability.

---

## 🧱 Architecture (Crates)

- **api**: RESTful API for managing websites, regions, and monitoring data. *(Built with [Poem](https://github.com/poem-web/poem))*
- **store**: Interacts with PostgreSQL for metadata storage.
- **redis_lib**: Manages Redis streams for task distribution and notification queues.
- **worker**: Scalable workers that check website status every 3 minutes, store data in InfluxDB, and trigger notifications.
- **cron**: Syncs PostgreSQL and Redis daily at midnight.
- **notification**: Sends email alerts via SMTP for downtime events.

---

## ⚙️ Technologies Used

- **Rust**: High-performance systems programming.
- **Poem**: Web framework for API development.
- **PostgreSQL**: Stores metadata for websites and regions.
- **Redis**: Task and notification queue (streams + consumer groups).
- **InfluxDB**: Time-series storage for status and response times.
- **Reqwest**: HTTP client for worker checks.
- **Lettre**: SMTP email client.
- **Tokio**: Asynchronous runtime.
- **SQLx**: Asynchronous PostgreSQL client.
- **Chrono**: Time/date utilities.
- **Serde**: JSON serialization/deserialization.
- **UUID**: Unique ID generator.
- **Tokio-Cron-Scheduler**: Scheduled background jobs.

---

## 🚀 Features

- ✅ Add, update, delete websites and regions via REST API.
- 🔍 Monitor website uptime/downtime and response times every 3 minutes.
- 📈 Store time-series data for charting (1, 7, 30 days).
- 📧 Email notifications on downtime events.
- 🔁 Daily sync of websites between PostgreSQL and Redis.
- 📊 Scalable architecture using Redis streams and worker pools.

---

## 📦 Setup Instructions

### 1. Prerequisites

Install the following:

- Rust
- PostgreSQL
- Redis
- InfluxDB

### 2. Environment Variables

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://user:password@localhost:5432/dbname
REDIS_URL=redis://localhost:6379
INFLUXDB_URL=http://localhost:8086
INFLUXDB_TOKEN=your-influxdb-token
SMTP_HOST=smtp.example.com
SMTP_USER=user@example.com
SMTP_PASS=password
3. Initialize Services
Run PostgreSQL migrations in store/migrations

Create an InfluxDB bucket named website_ticks

4. Run Components
bash
Copy
Edit
cargo run --bin api           # Run the API server
cargo run --bin worker        # Run the background workers
cargo run --bin cron          # Run daily sync scheduler
cargo run --bin notification  # Run the notification sender
🔌 API Endpoints
🌍 Website Management
POST /websites — Create website

GET /websites — List all websites

GET /websites/:id — Get specific website

PATCH /websites/:id — Update website

DELETE /websites/:id — Delete website

🌐 Region Management
POST /regions — Create region

GET /regions — List all regions

📊 Monitoring
GET /monitor — Latest status of all websites

GET /monitor/:id?days=1&region=europe — Time-series data for a website

GET /monitor/:id/downtime — Downtime history

GET /monitor/:id/last_downtime — Last downtime event
