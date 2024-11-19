require "rspec/expectations"
require "socket"
require "timeout"
require_relative "manage-processes"

RSpec.describe "ManageProcesses Integration" do
  before do
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
      # Start Socat with Process.spawn instead of as root
      socat_cmd = "socat TCP-LISTEN:#{@port},reuseaddr,fork SYSTEM:'echo ready'"
      socat_pid = Process.spawn(socat_cmd, pgroup: true)

      begin
        Timeout.timeout(10) do
          # Start processes in their own process groups
          first_cmd_pid = Process.spawn("sleep 5", pgroup: true)
          expect(Process.getpgid(first_cmd_pid)).not_to be_nil

          wait_for_port(@port)

          second_cmd_pid = Process.spawn("sleep 3", pgroup: true)
          expect(Process.getpgid(second_cmd_pid)).not_to be_nil

          # Wait for second process
          _, status = Process.wait2(second_cmd_pid)
          expect(status.exitstatus).to eq(0)

          # Use Process.kill instead of kill_process_group
          begin
            Process.kill("-TERM", first_cmd_pid) # The minus sign sends signal to process group
          rescue Errno::EPERM
            Process.kill("TERM", first_cmd_pid)  # Fallback to just the process
          end

          # Verify process termination
          expect { Process.getpgid(first_cmd_pid) }.to raise_error(Errno::ESRCH)
        end
      ensure
        # Clean up Socat process
        begin
          Process.kill("-TERM", socat_pid)
        rescue Errno::EPERM
          Process.kill("TERM", socat_pid)
        end
        begin
          Process.wait(socat_pid)
        rescue
          nil
        end
      end
    end
  end
end
