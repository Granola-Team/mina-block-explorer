require "sinatra"
require "launchy"

# Set the public folder to the parent directory
set :public_folder, File.dirname(__FILE__) + "/../.build/docs/doc"

puts File.dirname(__FILE__) + "/../.build/docs/doc"

# Serve mina_block_explorer/index.html for the root URL
get "/" do
  send_file File.join(settings.public_folder, "mina_block_explorer/index.html")
end

# Rewrite requests for *.html to mina_block_explorer/
get "/*.html" do
  file_path = File.join(settings.public_folder, "mina_block_explorer", params[:splat].first + ".html")
  if File.exist?(file_path)
    send_file file_path
  else
    halt 404
  end
end

# Handle subdirectories (e.g., /broadcast/index.html)
get "/*/index.html" do
  file_path = File.join(settings.public_folder, "mina_block_explorer", params[:splat].first, "index.html")
  if File.exist?(file_path)
    send_file file_path
  else
    halt 404
  end
end

# Open browser automatically
Thread.new do
  sleep 1
  Launchy.open("http://localhost:4567")
end
