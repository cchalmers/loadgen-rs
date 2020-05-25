#include "ccc.h"
#include "src/lib.rs.h"
#include <iostream>
#include <loadgen/c_api.h>

namespace mlperf {
    void assign_str(std::string& los, rust::Str str) {
      los.assign(str.data(), str.size());
    }

    LogSettingsOpaque::LogSettingsOpaque() {}

    std::unique_ptr<LogSettingsOpaque> mk_log_settings() {
        return std::unique_ptr<LogSettingsOpaque>(new LogSettingsOpaque());
    }

    const LogSettings& get_log_settings(const LogSettingsOpaque& ls) {
        return ls.inner;
    }

    LogSettings& get_log_settings_mut(LogSettingsOpaque& ls) {
        return ls.inner;
    }

    /* const LogOutputSettings& new_log(const int64_t& x) { */
    /* /1* void new_log() { *1/ */
    /*   LogOutputSettings* los = new LogOutputSettings; */
    /*   LogOutputSettings& los_ref = *los; */
    /*   return los_ref; */
    /*   /1* std::cerr << lg_output.prefix << "\n"; *1/ */
    /* } */

    /* /1* void print_log(const LogOutputSettings& lg_output) { *1/ */
    /* /1*   std::cerr << lg_output.prefix << "\n"; *1/ */
    /* /1* } *1/ */

    /* /1* void print_log(const LogOutputSettings& lg_output) { *1/ */
    /* /1*   std::cerr << lg_output.prefix << "\n"; *1/ */
    /* /1* } *1/ */

    /* LOS::LOS() {} */
    /* LOS::~LOS() { std::cerr << "done with LOS\n"; } */
    /* std::unique_ptr<LOS> mk_los() { */
    /*     return std::unique_ptr<LOS>(new LOS()); */
    /* } */
    /* const LogOutputSettings& get_los(const LOS& thing) { return thing.los; } */
    /* LogOutputSettings& get_los_mut(LOS& thing) { return thing.los; } */

    /*     /1* type LOS; *1/ */

    /*     /1* fn new_los() -> UniquePtr<LOS>; *1/ */
    /*     /1* fn get_nm(thing: &LOS) -> &CxxString; *1/ */


void start_test(SystemUnderTest &sut, QuerySampleLibrary &qsl, const TestSettings &requested_settings, const LogSettings &log_settings) {
  StartTest(&sut, &qsl, requested_settings, log_settings);
}

/* void start_log_test(size_t sut, size_t qsl, const TestSettings& settings, const LogSettings& log_settings) { */
/*     SystemUnderTestTrampoline* sut_cast = */
/*         reinterpret_cast<SystemUnderTestTrampoline*>(sut); */
/*     QuerySampleLibraryTrampoline* qsl_cast = */
/*         reinterpret_cast<QuerySampleLibraryTrampoline*>(qsl); */
/*     mlperf::StartTest(sut_cast, qsl_cast, settings, log_settings); */
/* } */

// mlperf::c::StartTest just forwards to mlperf::StartTest after doing the
// proper cast.
/* void StartTest2(void* sut, void* qsl, const TestSettings& settings) { */
/*   c::SystemUnderTestTrampoline* sut_cast = */
/*       reinterpret_cast<c::SystemUnderTestTrampoline*>(sut); */
/*   c::QuerySampleLibraryTrampoline* qsl_cast = */
/*       reinterpret_cast<c::QuerySampleLibraryTrampoline*>(qsl); */
/*   LogSettings default_log_settings; */
/*   mlperf::StartTest(sut_cast, qsl_cast, settings, default_log_settings); */
/* } */


/* /1* std::unique_ptr<LogSettings> new_log() { *1/ */
/* void new_log() { */
/*   /1* return std::unique_ptr<LogSettings> (new LogSettings()) *1/ */
/*   LogSettings settings; */
/*   /1* return std::unique_ptr<LogSettings>(new settings); *1/ */
/* } */

} // namespace org
