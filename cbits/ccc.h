#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>
#include <loadgen/loadgen.h>
#include <loadgen/c_api.h>

namespace mlperf {

void assign_str(std::string& los, rust::Slice<uint8_t> str);

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
} // namespace mlperf
