require 'optparse'
require 'socket'

# Function to check if the port is available using Ruby's Socket class
def port_open?(port, host = '127.0.0.1')
  begin
    Socket.tcp(host, port, connect_timeout: 1).close
    true
  rescue Errno::ECONNREFUSED, Errno::EHOSTUNREACH, SocketError
    false
  end
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

# Start a process and return its PID
def start_process(command)
  puts "Starting process: #{command}"
  pid = Process.spawn(command)
  pid
end

# Kill a process by its PID
def kill_process(pid)
  puts "Killing process with PID: #{pid}"
  Process.kill("TERM", pid)
  Process.wait(pid)
end

# Option parser for named arguments
options = {}
OptionParser.new do |opts|
  opts.banner = "Usage: manage_processes.rb [options]"

  opts.on("--port PORT", Integer, "Port to wait for") do |v|
    options[:port] = v
  end

  opts.on("--first-cmd COMMAND", "First command to run") do |v|
    options[:first_cmd] = v
  end

  opts.on("--second-cmd COMMAND", "Second command to run") do |v|
    options[:second_cmd] = v
  end

  opts.on("-h", "--help", "Show this help message") do
    puts opts
    exit
  end
end.parse!

# Validate input
unless options[:port] && options[:port].between?(0, 65535)
  puts "Invalid or missing port"
  exit 1
end

if options[:first_cmd].nil? || options[:second_cmd].nil?
  puts "Both first and second commands must be provided"
  exit 1
end

# Start the first process
first_pid = start_process(options[:first_cmd])

# Wait for the port to open
wait_for_port(options[:port])

# Start the second process
second_pid = start_process(options[:second_cmd])

# Wait for the second process to finish
_, second_status = Process.wait2(second_pid)

# Kill the first process
kill_process(first_pid)

# Exit with the second process's status code
exit second_status.exitstatus
