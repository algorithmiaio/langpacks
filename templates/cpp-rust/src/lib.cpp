#include <string>

std::string * buffer;

// Allocates string and returns c_str pointer. Deallocate via cleanup()
extern "C" const char *
hello(const char * name) {
  buffer = new std::string("Hello from C++, ");
  buffer->append(name);
  return buffer->c_str();
}

extern "C" void
cleanup() {
  delete buffer;
}

