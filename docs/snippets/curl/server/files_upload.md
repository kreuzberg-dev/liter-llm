```bash
curl http://localhost:4000/v1/files \
  -H "Authorization: Bearer $LITER_LLM_MASTER_KEY" \
  -F purpose=batch \
  -F file=@requests.jsonl
```
