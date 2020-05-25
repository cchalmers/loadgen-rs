#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>
#include <loadgen/loadgen.h>
#include <loadgen/c_api.h>

namespace mlperf {


class ThingC {
public:
  ThingC(std::string appname);
  ~ThingC();

  std::string appname;
};

class LogSettingsOpaque {
public:
  LogSettingsOpaque();
  LogSettings inner;
};

class LOS {
public:
  LOS();
  ~LOS();
  LogOutputSettings los;
};
std::unique_ptr<LOS> mk_los();
const LogOutputSettings& get_los(const LOS& thing);
LogOutputSettings& get_los_mut(LOS& thing);
void assign_outdir(LogOutputSettings& los, rust::Str outdir);
void assign_prefix(LogOutputSettings& los, rust::Str prefix);
void assign_suffix(LogOutputSettings& los, rust::Str suffix);
void assign_str(std::string& los, rust::Str str);
const std::string& prefix(const LogOutputSettings& los);


void print_log(const LogOutputSettings& lg_output);

std::unique_ptr<LogOutputSettings> mk_logout_settings();
std::unique_ptr<LogSettingsOpaque> mk_log_settings();
const LogSettings& get_log_settings(const LogSettingsOpaque& ls);
LogSettings& get_log_settings_mut(LogSettingsOpaque& ls);

struct SharedThing;

std::unique_ptr<ThingC> make_demo(rust::Str appname);
const std::string &get_name(const ThingC &thing);
void do_thing(SharedThing state);

void query_samples_complete(QuerySampleResponse &responses, size_t response_count);

void start_test(SystemUnderTest &sut, QuerySampleLibrary &qsl, const TestSettings &requested_settings, const LogSettings &log_settings);

const LogOutputSettings& new_log(const int64_t& x);
/* void new_log(mlperf::LogSettings& log_settings); */
void start_log_test(size_t sut, size_t qsl, const TestSettings& settings, const LogSettings& log_settings);

} // namespace org
