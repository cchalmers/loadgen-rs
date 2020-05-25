#include "ccc.h"
#include "src/lib.rs.h"
#include <iostream>
#include <loadgen/c_api.h>

namespace mlperf {

    ThingC::ThingC(std::string appname) : appname(std::move(appname)) {}

    ThingC::~ThingC() { std::cout << "done with ThingC" << std::endl; }

    std::unique_ptr<ThingC> make_demo(rust::Str appname) {
        return std::unique_ptr<ThingC>(new ThingC(std::string(appname)));
    }

    std::unique_ptr<LogOutputSettings> mk_logout_settings() {
        return std::unique_ptr<LogOutputSettings>(new LogOutputSettings());
    }

    const std::string &get_name(const ThingC &thing) { return thing.appname; }

    void do_thing(SharedThing state) { print_r(*state.y); }

    void query_samples_complete(QuerySampleResponse &responses, size_t response_count) {
      QuerySamplesComplete(&responses, response_count);
    }

    void print_log(const LogOutputSettings& lg_output) {
      std::cerr << "printing the los:\n";
      std::cerr << lg_output.prefix << "\n";
    }

    void assign_prefix(LogOutputSettings& los, rust::Str prefix) {
      los.prefix = std::string(prefix);
    }
    void assign_suffix(LogOutputSettings& los, rust::Str suffix) {
      los.suffix = std::string(suffix);
    }
    void assign_outdir(LogOutputSettings& los, rust::Str outdir) {
      los.outdir = std::string(outdir);
    }
    const std::string& prefix(const LogOutputSettings& los) {
      return los.prefix;
    }

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

    const LogOutputSettings& new_log(const int64_t& x) {
    /* void new_log() { */
      LogOutputSettings* los = new LogOutputSettings;
      LogOutputSettings& los_ref = *los;
      return los_ref;
      /* std::cerr << lg_output.prefix << "\n"; */
    }

    /* void print_log(const LogOutputSettings& lg_output) { */
    /*   std::cerr << lg_output.prefix << "\n"; */
    /* } */

    /* void print_log(const LogOutputSettings& lg_output) { */
    /*   std::cerr << lg_output.prefix << "\n"; */
    /* } */

    LOS::LOS() {}
    LOS::~LOS() { std::cerr << "done with LOS\n"; }
    std::unique_ptr<LOS> mk_los() {
        return std::unique_ptr<LOS>(new LOS());
    }
    const LogOutputSettings& get_los(const LOS& thing) { return thing.los; }
    LogOutputSettings& get_los_mut(LOS& thing) { return thing.los; }

        /* type LOS; */

        /* fn new_los() -> UniquePtr<LOS>; */
        /* fn get_nm(thing: &LOS) -> &CxxString; */


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
