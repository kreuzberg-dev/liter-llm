<!-- snippet:compile-only -->

```go
package main

import (
 "context"
 "fmt"
 "os"

 llm "github.com/kreuzberg-dev/liter-llm/packages/go"
)

func main() {
 client := llm.NewClient(llm.WithAPIKey(os.Getenv("MISTRAL_API_KEY")))
 resp, err := client.OCR(context.Background(), &llm.OCRRequest{
  Model: "mistral/mistral-ocr-latest",
  Document: llm.DocumentInput{
   Type: "document_url",
   URL:  "https://example.com/invoice.pdf",
  },
 })
 if err != nil {
  panic(err)
 }
 for _, page := range resp.Pages {
  fmt.Printf("Page %d: %.100s...\n", page.Index, page.Markdown)
 }
}
```
