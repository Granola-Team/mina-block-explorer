require "rspec"
require "socket"
require "timeout"
require_relative "manage-processes"

RSpec.describe "ManageProcesses Integration" do
  before do
    # Ensure the port we test is not occupied
    @port = 12345
    begin
      socket = Socket.tcp("127.0.0.1", @port, connect_timeout: 1)
      socket.close
      raise "Port #{@port} is already in use, choose another port"
    rescue Errno::ECONNREFUSED
      # Port is free, continue testing
    end
  end

  describe "Script integration with real processes" do
    it "starts and stops processes, waits for the port, and handles SIGINT" do
      # Start a Netcat process to listen on the specified port (simulating a service)
      socat_pid = Process.spawn("socat TCP-LISTEN:#{@port},reuseaddr,fork -", pgroup: true)

      # Use Timeout to avoid waiting forever in case something goes wrong
      Timeout.timeout(10) do
        # Step 1: Start the first process (a simple sleep command)
        first_cmd_pid = start_process("sleep 5")

        # Ensure the first process starts correctly
        expect(Process.getpgid(first_cmd_pid)).not_to be_nil

        # Step 2: Wait for the port (which is opened by Netcat) to be available
        wait_for_port(@port)

        # Step 3: Start the second process (another sleep command)
        second_cmd_pid = start_process("sleep 3")

        # Ensure the second process starts correctly
        expect(Process.getpgid(second_cmd_pid)).not_to be_nil

        # Step 4: Wait for the second process to finish
        _, status = Process.wait2(second_cmd_pid)
        expect(status.exitstatus).to eq(0)

        # Step 5: Kill the first process after the second finishes
        kill_process_group(first_cmd_pid)

        # Ensure the first process is terminated
        expect { Process.getpgid(first_cmd_pid) }.to raise_error(Errno::ESRCH)
      end
    ensure
      # Ensure the Netcat process is killed at the end of the test
      Process.kill("TERM", socat_pid) if socat_pid
      Process.wait(socat_pid)
    end
  end
end
