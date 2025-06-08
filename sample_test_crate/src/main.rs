use influxdb::{Client, ReadQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("http://localhost:8086", "website_ticks").with_token(
        "Vly8MdPQtaGaV3m9Q3H-6FL1HIbOC7Mj75c40tQqAto_nFUeSE0msZPgF6uIfZ5dwEs25UFKvBD5bJ3a9DjStw==",
    );

    // InfluxQL query for raw data
    let query = ReadQuery::new(
        r#"SELECT * FROM "website_tick"
           WHERE "website_id" = 'e6728a25-30f3-4c19-bf47-1b0147a86660'
           AND "region_id" = 'europe'
           AND "status" = 'Up'
           AND time > now() - 1d"#,
    );

    let result = client.query(query).await?;
    println!("{}", result);

    Ok(())
}

// use serde_json::Value;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();

//     let query = r#"
//         from(bucket: "website_ticks")
//         |> range(start: -24h)
//         |> filter(fn: (r) => r["_measurement"] == "website_tick")
//         |> filter(fn: (r) => r["_field"] == "response_time_ms")
//         |> filter(fn: (r) => r["region_id"] == "europe")
//         |> filter(fn: (r) => r["status"] == "Up")
//         |> filter(fn: (r) => r["website_id"] == "e6728a25-30f3-4c19-bf47-1b0147a86660")
//     "#;

//     let response = client
//         .post("http://localhost:8086/api/v2/query?org=website_ticks")
//         .header("Authorization", "Token Vly8MdPQtaGaV3m9Q3H-6FL1HIbOC7Mj75c40tQqAto_nFUeSE0msZPgF6uIfZ5dwEs25UFKvBD5bJ3a9DjStw==")
//         .header("Content-Type", "application/vnd.flux")
//         .body(query)
//         .send()
//         .await?;

//     let text = response.text().await?;
//     println!("{}", text);

//     Ok(())
// }

// without reqwstst
// use influxdb2::Client;
// use influxdb2::models::Query;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new(
//         "http://localhost:8086",
//         "website_ticks", // Replace with your actual organization name
//         "Vly8MdPQtaGaV3m9Q3H-6FL1HIbOC7Mj75c40tQqAto_nFUeSE0msZPgF6uIfZ5dwEs25UFKvBD5bJ3a9DjStw==",
//     );

//     // Raw data query without aggregation
//     let qs = format!(
//         r#"
//         from(bucket: "website_ticks")
//         |> range(start: -24h)
//         |> filter(fn: (r) => r["_measurement"] == "website_tick")
//         |> filter(fn: (r) => r["_field"] == "response_time_ms")
//         |> filter(fn: (r) => r["region_id"] == "europe")
//         |> filter(fn: (r) => r["status"] == "Up")
//         |> filter(fn: (r) => r["website_id"] == "e6728a25-30f3-4c19-bf47-1b0147a86660")
//     "#
//     );

//     let query = Query::new(qs);
//     let response = client.query_raw(Some(query)).await?;
//     println!("{:?}", response);

//     Ok(()) //return struct  [FluxRecord { table: 0, values: {"_field": String("response_time_ms"), "_measurement": String("website_tick"), "_start": TimeRFC(2025-06-07T09:34:51.678573384+00:00), "_stop": TimeRFC(2025-06-08T09:34:51.678573384+00:00), "_time": TimeRFC(2025-06-07T18:32:38.686+00:00), "_value": Long(3254), "region_id": String("europe"), "result": String("_result"), "status": String("Up"), "table": Long(0), "website_id": String("ed0870a7-9486-498a-83d2-6dff97b857b7")} } ]
// }
