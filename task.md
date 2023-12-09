# Mini Project: Cloud Instance Resource Monitoring 
## Project Description: 
- In this mini project, you will create a TCP server (C/C++) that listens on a fixed port number (e.g., 9090) for incoming requests. 
- The server will use different opcodes to handle various actions, including gathering statistics (CPU, memory, disk, network, and cloud platform). 
- Additionally, you will develop a client application(C/C++) that can send requests to the server and display the server’s responses. 
- The server will be packaged as Debian packages for easy installation, removal, start, and stop. 
- The server will also support both loopback and outside IP requests, log all requests with log rotation, and handle multiple requests concurrently using multiple threads. 

 
## Server Requirements: 
- Debian Package for Server: Create a Debian package (.deb) for the server, enabling easy installation, removal, start, and stop using standard package management tools like dpkg.
- TCP Server: Develop a TCP server in C/C++ that listens on port 9090 for incoming requests. 
- Support for Loopback and Outside IP: Ensure that the server can accept requests from both loopback (localhost) and outside IP addresses. 
- Different Opcodes: Implement different opcodes to handle various actions or requests. For example: 
- Opcode 1: Request for cloud platform statistics 
- Opcode 2: Request for CPU statistics 
- Opcode 3: Request for Memory statistics 
- Opcode 4: Request for disk statistics 
- Opcode 5: Request for network statistics 
- Concurrency with Multiple Threads: Use multithreading to handle multiple requests concurrently. Each incoming request should be handled in its own thread to prevent blocking other clients.
- Logging with Log Rotation: Implement logging for all incoming requests, including the request type, timestamp, and relevant details. Implement log rotation to manage log files and prevent them from growing indefinitely. 
- Client Requirements: 
    - TCP Client: Client in C/C++ that can send requests to the server. 
    - User Interaction: The client should run in a loop, displaying a menu that asks the user to choose which type of request (opcode) to send to the server. After receiving a response from the server, the client should display the result. 
 
## Sample Output: 
1.  ### `Get CPU Statistics` 
    - Number of Cores: 4    
    - CPU Model: Intel Core i7   
    - CPU Usage: 25% 
2.  ### `Get Memory Statistics` 
    - Total Memory: 8192 MB    
    - Used Memory: 4096 MB    
    - Free Memory: 4096 MB   
    - Memory Type: DDR4     
3.  ### `Get Disk Statistics` 
    - Total Disk Space: 100 GB    
    - Used Disk Space: 60 GB    
    - Free Disk Space: 40 GB    
    - Disk Type: SSD    
4. ### `Get Network Statistics`:    
    - List of Network Interfaces:    
        1.  - Interface Name: ens3    
            - IP Address: 192.168.1.100     
            - TX Speed: 1 Gbps     
            - RX Speed: 1 Gbps
                
        2.  - Interface Name: ens4     
            - IP Address: 192.168.1.101     
            - TX Speed: 1 Gbps    
            - RX Speed: 1 Gbps 
5.  ### `Get Platform Details` 
    - Cloud Platform: OpenStack    
    - Kernel Version: 5.4.0    
    - Architecture: x86_64    
    - Hypervisor: KVM    
    - Instance Type: Virtual Machine 

 
## Skills exercised: 
1. TCP/IP client server in C/C++ 
2. Server like UCT Agent 
3. Cloud platform understanding 
4. Logging functionality 
5. Statistics monitoring
