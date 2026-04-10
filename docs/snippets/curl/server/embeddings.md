```bash
curl http://localhost:4000/v1/embeddings \
  -H "Authorization: Bearer $LITER_LLM_MASTER_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "text-embedding-3-small",
    "input": ["the quick brown fox", "the lazy dog"]
  }'
```
