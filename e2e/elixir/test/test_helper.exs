ExUnit.start()

# Spawn mock-server binary and set MOCK_SERVER_URL for all tests.
mock_server_bin = Path.expand("../../rust/target/release/mock-server", __DIR__)
fixtures_dir = Path.expand("../../../fixtures", __DIR__)

if File.exists?(mock_server_bin) do
  port = Port.open({:spawn_executable, mock_server_bin}, [
    :binary,
    :line,
    args: [fixtures_dir]
  ])
  receive do
    {^port, {:data, {:eol, "MOCK_SERVER_URL=" <> url}}} ->
      System.put_env("MOCK_SERVER_URL", url)
  after
    30_000 ->
      raise "mock-server startup timeout"
  end
end
