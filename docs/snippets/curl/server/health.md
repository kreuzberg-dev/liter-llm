```bash
# Liveness probe: returns 200 as long as the process is running.
curl -fsS http://localhost:4000/health/liveness

# Readiness probe: returns 200 once service pool and file store are initialised.
curl -fsS http://localhost:4000/health/readiness

# Full status: includes configured model list.
curl -fsS http://localhost:4000/health
```
