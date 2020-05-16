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



    const std::string &get_name(const ThingC &thing) { return thing.appname; }

    void do_thing(SharedThing state) { print_r(*state.y); }

    void query_samples_complete(QuerySampleResponse &responses, size_t response_count) {
      QuerySamplesComplete(&responses, response_count);
    }


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


/* std::unique_ptr<LogSettings> new_log() { */
void new_log() {
  /* return std::unique_ptr<LogSettings> (new LogSettings()) */
  LogSettings settings;
  /* return std::unique_ptr<LogSettings>(new settings); */
}

} // namespace org
