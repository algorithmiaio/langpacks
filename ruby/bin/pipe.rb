require 'json'
require 'base64'
require_relative "#{Dir.pwd}/src/algorithm.rb" # TODO: would be nice to know algoname

def pipe_loop
  STDIN.each_line do |line|
    request = JSON.parse(line)
    response = begin
      result = call_algorithm(request)
      content_type = if result.is_a? String then
        result.encoding == 'ASCII-8BIT' ? 'binary' : 'text'
      else
        'json'
      end
      { :result => result, :metadata => { :content_type => content_type}}
    rescue Exception => e
      { :error => { :message => e.message, :stacktrace => e.backtrace.join("\n") }}
    end

    # Add final newline delimeter and flush stdout before writing back response
    puts "\n"
    STDOUT.flush
    File.open('/tmp/algoout', 'w') do |algoout|
      algoout.puts response.to_json
    end
  end
end

def call_algorithm(request)
  input = case request['content_type']
    when 'text', 'json'
      request['data']
    when 'binary'
      Base64.decode64(request['data'])
    else
      raise "Invalid content_type: #{request['content_type']}"
  end
  apply(input)
end

pipe_loop
