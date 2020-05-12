#include "ccc.h"
#include "src/main.rs.h"
#include <iostream>

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

} // namespace org
