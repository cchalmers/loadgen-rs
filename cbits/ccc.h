#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>
#include <loadgen/loadgen.h>
#include <loadgen/c_api.h>

namespace mlperf {

void assign_str(std::string& los, rust::Str str);

class LogSettingsOpaque {
public:
  LogSettingsOpaque();
  LogSettings inner;
};

std::unique_ptr<LogOutputSettings> mk_logout_settings();
std::unique_ptr<LogSettingsOpaque> mk_log_settings();
const LogSettings& get_log_settings(const LogSettingsOpaque& ls);
LogSettings& get_log_settings_mut(LogSettingsOpaque& ls);

int test_settings_from_file(TestSettings& settings, rust::Str path, rust::Str model, rust::Str scenario);

/* void query_samples_complete(QuerySampleResponse &responses, size_t response_count); */

/* void start_test(SystemUnderTest &sut, QuerySampleLibrary &qsl, const TestSettings &requested_settings, const LogSettings &log_settings); */

} // namespace mlperf
