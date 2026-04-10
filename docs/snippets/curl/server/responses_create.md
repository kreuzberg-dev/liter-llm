```bash
curl http://localhost:4000/v1/responses \
  -H "Authorization: Bearer $LITER_LLM_MASTER_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4o",
    "input": "Explain eventual consistency to a backend engineer."
  }'
```
