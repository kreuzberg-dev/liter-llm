```bash
curl http://localhost:4000/v1/images/generations \
  -H "Authorization: Bearer $LITER_LLM_MASTER_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "dall-e-3",
    "prompt": "A cross-section diagram of a Rust async runtime",
    "size": "1024x1024",
    "n": 1
  }'
```
