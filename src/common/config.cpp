#include "arion/common/config.hpp"
#include <memory>

using namespace arion;

std::unique_ptr<Config> new_config() {
    return std::make_unique<Config>();
}

arion::ARION_LOG_LEVEL Config::get_log_level() {
    auto it = config_map.find("log_lvl");
    if (it != config_map.end())
        return std::any_cast<ARION_LOG_LEVEL>(it->second);
    return ARION_LOG_LEVEL::INFO; // fallback
}
