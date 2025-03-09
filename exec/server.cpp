/*
 *****************************************************
 *
 * Standard Library Includes
 *
 *****************************************************
 */

#include <errno.h>
#include <format>
#include <iostream>
#include <netinet/in.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>
#include <vector>

/*
 *****************************************************
 *
 * Squeef Library Includes
 *
 *****************************************************
 */

#include "log.hpp"

/*
 *****************************************************
 *
 * Struct Type Definitions
 *
 *****************************************************
 */

struct Server {
  std::vector<Logger> loggers;
  int port;
  int server_fd;

  Server(int port, std::vector<Logger> loggers) : loggers{loggers}, port{port} { server_fd = -1; }

  void log_info(std::string msg) {
    for (const Logger &l : loggers) {
      l.log_info(msg);
    }
  }

  void log_error(std::string msg) {
    for (const Logger &l : loggers) {
      l.log_error(msg);
    }
  }

  void start() {
    log_info("Starting server...");

    server_fd = socket(AF_INET, SOCK_STREAM, 0);

    if (server_fd == -1) {
      log_error(std::format("Failed to create socket. Errno {}", strerror(errno)));
      return;
    }

    struct sockaddr_in address{.sin_family = AF_INET,
                               .sin_port = htons(static_cast<in_port_t>(port)),
                               .sin_addr = {.s_addr = INADDR_ANY},
                               .sin_zero = {0}};

    if (bind(server_fd, reinterpret_cast<struct sockaddr *>(&address), sizeof(address)) == -1) {
      log_error(std::format("Failed to bind socket. Errno {}", strerror(errno)));
      return;
    }

    if (listen(server_fd, 0) == -1) {
      log_error(std::format("Failed to start listening. Errno {}", strerror(errno)));
      return;
    }

    log_info(std::format("Server started. Listening on :{}", port));
  }

  void run() {
    int client_fd;
    struct sockaddr_in client_addr;
    int client_addr_len;

    while (true) {
      client_fd = accept(server_fd,
                         reinterpret_cast<struct sockaddr *>(&client_addr),
                         reinterpret_cast<socklen_t *>(&client_addr_len));

      if (client_fd == -1) {
        log_error(std::format("Failed to accept incoming connection. Errno {}", strerror(errno)));
        return;
      }

      log_info(std::format("Accepted connection from {}", client_addr.sin_addr.s_addr));

      run_connection(client_fd);
    }
  }

  // @TODO
  void run_connection(int client_fd) {
    char buf[128];
    int read_len = read(client_fd, buf, sizeof(buf));
    buf[read_len] = 0;
    std::cout << buf << std::endl;

    close(client_fd);
  }

  void stop() {
    log_info("Stopping server...");
    close(server_fd);
    log_info("Server stopped");
  }
};

/*
 *****************************************************
 *
 * Functions
 *
 *****************************************************
 */

int main(void) {
  Logger default_logger{"default", std::cout};

  Server s(6870, {default_logger});

  s.start();
  s.run();
  s.stop();

  return 0;
}
