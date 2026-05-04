package e2e_test

import (
	"bufio"
	"io"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"testing"
)

func TestMain(m *testing.M) {
	_, filename, _, _ := runtime.Caller(0)
	dir := filepath.Dir(filename)
	mockServerBin := filepath.Join(dir, "..", "rust", "target", "release", "mock-server")
	fixturesDir := filepath.Join(dir, "..", "..", "fixtures")
	cmd := exec.Command(mockServerBin, fixturesDir)
	cmd.Stderr = os.Stderr
	stdout, err := cmd.StdoutPipe()
	if err != nil {
		panic(err)
	}
	if err := cmd.Start(); err != nil {
		panic(err)
	}
	scanner := bufio.NewScanner(stdout)
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "MOCK_SERVER_URL=") {
			_ = os.Setenv("MOCK_SERVER_URL", strings.TrimPrefix(line, "MOCK_SERVER_URL="))
			break
		}
	}
	go func() { _, _ = io.Copy(io.Discard, stdout) }()
	code := m.Run()
	_ = cmd.Process.Signal(os.Interrupt)
	_ = cmd.Wait()
	os.Exit(code)
}
