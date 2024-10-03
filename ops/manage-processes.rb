require "optparse"
require "socket"

# Function to check if the port is available using Ruby's Socket class
def port_open?(port, host = "127.0.0.1")
  Socket.tcp(host, port, connect_timeout: 1).close
  true
rescue Errno::ECONNREFUSED, Errno::EHOSTUNREACH, SocketError
  false
end

# Function to wait for a port to become available
def wait_for_port(port, interval = 5)
  puts "Waiting for port #{port} to become available..."
  until port_open?(port)
    puts "Port #{port} is not available, retrying in #{interval} seconds..."
    sleep interval
  end
  puts "Port #{port} is now available."
end

# Start a process and return its PID, using a new process group to handle signals
def start_process(command)
  puts "Starting process: #{command}"
  Process.spawn(command, pgroup: true) # Start in a new process group
end

def kill_process_group(pid)
  puts "Attempting to kill process group with PID: #{pid}"
  begin
    # Send the TERM signal to the process group (-pid means the group)
    Process.kill("TERM", -pid)
  rescue Errno::EPERM => e
    # Ignore permission errors when killing the process group
    puts "Error: #{e.message}. Some processes in the group could not be killed."
  rescue Errno::ESRCH => e
    # Ignore errors if the process or group does not exist
    puts "Error: #{e.message}. Process group not found."
  end

  # Ensure that we wait for the main process itself to finish
  begin
    Process.wait(pid)
  rescue Errno::ECHILD
    puts "No child processes left to wait for."
  end
end

if __FILE__ == $0
  # Set up OptionParser for handling command-line arguments
  options = {}
  OptionParser.new do |opts|
    opts.banner = "Usage: manage_processes.rb [options]"

    opts.on("--port PORT", Integer, "Port to wait for") do |v|
      options[:port] = v
    end

    opts.on("--first-cmd COMMAND", "First command to run") do |v|
      options[:first_cmd] = v
    end

    opts.on("--second-cmd COMMAND", "Second command to run after port is available") do |v|
      options[:second_cmd] = v
    end

    opts.on("-h", "--help", "Show this help message") do
      puts opts
      exit
    end
  end.parse!

  # Validate input
  if options[:port].nil? || options[:port] < 0 || options[:port] > 65535
    puts "Invalid or missing port number"
    exit 1
  end

  if options[:first_cmd].nil? || options[:second_cmd].nil?
    puts "Both first and second commands must be provided"
    exit 1
  end

  # Track child process PIDs to ensure they can be killed later
  first_process_pid = nil
  second_process_pid = nil

  # Set up signal handling to ensure children are killed on SIGINT
  trap("INT") do
    puts "\nReceived SIGINT, terminating child processes..."
    kill_process_group(first_process_pid) if first_process_pid
    kill_process_group(second_process_pid) if second_process_pid
    exit 1
  end

  # Step 1: Start the first process
  first_process_pid = start_process(options[:first_cmd])

  # Step 2: Wait for the port to be available
  wait_for_port(options[:port])

  # Step 3: Start the second process
  second_process_pid = start_process(options[:second_cmd])

  # Step 4: Wait for the second process to finish
  _, second_status = Process.wait2(second_process_pid)
  puts "Second process finished with status: #{second_status.exitstatus}"

  # Step 5: Kill the first process after the second finishes
  kill_process_group(first_process_pid)

  # Step 6: Propagate the exit status of the second process
  exit second_status.exitstatus
end
