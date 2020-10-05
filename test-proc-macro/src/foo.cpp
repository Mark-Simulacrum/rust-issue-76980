#include <string>
#include <sstream>

extern "C" int custom_to_string(unsigned int id) {
	std::stringstream os;
	os << id;
	return os.str().length();
}
