#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>
#include <loadgen/loadgen.h>

namespace mlperf {


class ThingC {
public:
  ThingC(std::string appname);
  ~ThingC();

  std::string appname;
};

struct SharedThing;

std::unique_ptr<ThingC> make_demo(rust::Str appname);
const std::string &get_name(const ThingC &thing);
void do_thing(SharedThing state);

void query_samples_complete(QuerySampleResponse &responses, size_t response_count);

void start_test(SystemUnderTest &sut, QuerySampleLibrary &qsl, const TestSettings &requested_settings, const LogSettings &log_settings);

} // namespace org
