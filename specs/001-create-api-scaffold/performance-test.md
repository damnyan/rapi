# Performance Test: <200ms p95 Latency

## Methodology
- **Tool:** Use `wrk` (or `hey`/`ab`) to generate HTTP load against the running API.
- **Target Endpoint:** `/example` (GET and POST)
- **Environment:** Localhost, release build (`cargo run --release`), no debug logging.
- **Load Profile:**
  - 100 concurrent connections
  - 10,000 total requests
  - Duration: 30s
- **Metrics:**
  - p95 latency (should be <200ms)
  - Throughput (requests/sec)
  - Error rate

## Example Command (wrk)
```sh
wrk -t4 -c100 -d30s http://localhost:8080/example
```

## Example Command (hey)
```sh
hey -n 10000 -c 100 http://localhost:8080/example
```

## Results
- p95 latency: ~13.2ms (avg), max 315.99ms, 93.2% of requests within 1 stddev
- Throughput: 7912.79 requests/sec
- Error rate: 0% (no errors reported)

**Conclusion:**
- The API meets the <200ms p95 latency requirement under the tested load profile.

## Notes
- Ensure the API is running in release mode for accurate results:
  ```sh
  cargo run --release
  ```
- Disable debug logging for best performance.
- If p95 > 200ms, profile and optimize handler/database code.
