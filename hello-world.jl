# println("hello world!!");
# f(x, y) = x + y
# println(f(1, 2));

using Sockets

function handle_request(sock::IOStream)
    request = readline(sock)
    println("Received request: $request")
    response = "Hello, client!\n"
    write(sock, response)
    # close(sock)
end

function start_server(port::Int)
    server = listen(port)
    println("Server started on port $port...")
    while true
        sock = accept(server)
        # handle_request(sock)
        request = readline(sock)
        println("Received request: $request")
        response = "Hello, client!\n"
        write(sock, response)
    end
end


start_server(2000)
#= errormonitor(@async begin
    server = listen(2000)
    while true
        sock = accept(server)
        println("Hello World\n")
    end
end) =#
