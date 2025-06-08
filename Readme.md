Website Monitoring App

A scalable, high-performance website monitoring system inspired by BetterStack and BetterUptime. Designed to track uptime and response times of millions of websites across multiple regions with detailed monitoring data and notification capabilities.

✨ Features

Add, update, and delete websites and regions via REST API

Website status checks every 3 minutes (Up/Down)

Store time-series data (response times) in InfluxDB

Visualize performance metrics: response times vs. timestamps

Downtime detection with precise tracking

Email notifications for downtime events

Monitor websites across multiple regions

Scalable worker architecture using Redis streams

Daily sync between PostgreSQL and Redis queues

🧰 Technologies Used

Rust: High-performance systems language

Poem: Web framework for RESTful APIs

PostgreSQL: Persistent metadata storage

Redis: Stream-based job distribution

InfluxDB: Time-series storage of monitoring data

Reqwest: HTTP client for uptime checks

Lettre: SMTP client for email alerts

Tokio: Async runtime

SQLx: Async Postgres queries

Chrono: Date and time handling

Serde: JSON serialization

UUID: Unique ID generation

Tokio-Cron-Scheduler: Daily task scheduling

🚀 Setup Instructions

1. Install Dependencies

Rust

PostgreSQL

Redis

InfluxDB

2. Environment Variables (.env)

DATABASE_URL=postgres://user:password@localhost:5432/dbname
REDIS_URL=redis://localhost:6379
INFLUXDB_URL=http://localhost:8086
INFLUXDB_TOKEN=your-influxdb-token
SMTP_HOST=smtp.example.com
SMTP_USER=user@example.com
SMTP_PASS=password
INFLUXDB_ORG="website_ticks"

3. Initialize Databases

Run PostgreSQL migrations in store/migrations

Create InfluxDB bucket: website_ticks

4. Run Services

cargo run --bin api
cargo run --bin worker
cargo run --bin cron
cargo run --bin notification

🔗 API Endpoints

Websites

POST /websites: Add a websitePayload: { "url": "https://google.com", "name": "google" }

GET /websites: List all websites

GET /websites/:id: Get website details

PATCH /websites/:id: Update a website URL or name

DELETE /websites/:id: Remove a website

Regions

POST /regions: Add a regionPayload: { "name": "europe" }

GET /regions: List all regions

Monitoring

GET /monitor: List websites with latest monitoring status

GET /monitor/:id?days=1&region=europe: Time-series data (response times)

GET /monitor/:id/downtime?region=europe&days=1: Downtime data

GET /monitor/:id/last_downtime?region=europe&days=1: Last downtime event (or null if none)

📊 Dashboard UI Ideas

When a user clicks on a website:

Show Uptime Status:

Calculate time since last downtime (API: /monitor/:id/last_downtime)

Display duration in days, hours, minutes

Draw Time Series Chart:

X-axis: Timestamp

Y-axis: Response Time in ms

Two Data Lines:

Response Times (from /monitor/:id?days=n&region=...)

Downtime Events (from /monitor/:id/downtime?...)

Downtime Summary Panel:

Total downtimes

Time & region of each downtime event

Exact timestamps from downtime API response

📅 Example API Usage with Axios

// Add Website
axios.post('http://localhost:3002/websites', {
  url: 'https://google.com',
  name: 'google'
});

// Get All Websites
axios.get('http://localhost:3002/websites');

// Update Website
axios.patch('http://localhost:3002/websites/:id', {
  url: 'https://new-url.com'
});

// Delete Website
axios.delete('http://localhost:3002/websites/:id');

🚧 Final Notes

Backend is fully complete ✔

Monitor millions of sites across global regions ✔

Extendable for SMS/Slack/PagerDuty in future ✔

Perfect base to build a UI like BetterUptime ✔

Built with love in Rust ❤️ by Pankaj.
