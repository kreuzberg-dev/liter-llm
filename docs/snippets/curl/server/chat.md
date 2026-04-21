```bash
curl http://localhost:4000/v1/chat/completions \
  -H "Authorization: Bearer $LITER_LLM_MASTER_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4o",
    "messages": [
      {"role": "user", "content": "Summarize the CAP theorem in one sentence."}
    ]
  }'
```
