# Check if arguments are provided
if ARGV.empty?
  puts "Usage: ruby ops/validate-env.rb ENV_VAR1 ENV_VAR2 ..."
  exit 1
end

# Get the list of required environment variables from script arguments
required_env_vars = ARGV

# Check if each required environment variable is set
missing_vars = required_env_vars.select { |var| ENV[var].nil? || ENV[var].strip.empty? }

# If there are any missing variables, output an error and exit
unless missing_vars.empty?
  puts "Error: The following environment variables are missing or empty: #{missing_vars.join(', ')}"
  exit 1
end

puts "All required environment variables are set."
