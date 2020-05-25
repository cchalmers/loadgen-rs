#include "ccc.h"
#include "src/lib.rs.h"
#include <loadgen/c_api.h>

namespace mlperf {
  void assign_str(std::string& los, rust::Str str) {
    los.assign(str.data(), str.size());
  }

  LogSettingsOpaque::LogSettingsOpaque() {}

  std::unique_ptr<LogSettingsOpaque> mk_log_settings() {
    return std::unique_ptr<LogSettingsOpaque>(new LogSettingsOpaque());
  }

  const LogSettings& get_log_settings(const LogSettingsOpaque& ls) { return ls.inner; }

  LogSettings& get_log_settings_mut(LogSettingsOpaque& ls) { return ls.inner; }

  int test_settings_from_file(TestSettings& settings, rust::Str path, rust::Str model, rust::Str scenario) {
    return settings.FromConfig(std::string(path), std::string(model), std::string(scenario));
  }

} // namespace mlperf
