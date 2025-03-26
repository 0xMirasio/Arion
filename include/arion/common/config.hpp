#ifndef ARION_CONFIG_HPP
#define ARION_CONFIG_HPP

#include <arion/common/global_defs.hpp>
#include "arion/common/global_excepts.hpp"
#include <any>
#include <map>
#include <string>
#include <memory>

class ARION_EXPORT Config {
private:
    std::map<std::string, std::any> config_map = {
        {"log_lvl", arion::ARION_LOG_LEVEL::INFO},
        {"enable_sleep_syscalls", false}
    };
public:

    template <typename T>
    void ARION_EXPORT set_field(const std::string& key, T value) {
        auto it = config_map.find(key);
        if (it == config_map.end())
            throw ConfigKeyNotFoundException(key);
        it->second = value;
    }

    template <typename T>
    T ARION_EXPORT get_field(const std::string& key) const {
        auto it = config_map.find(key);
        if (it == config_map.end())
            throw ConfigKeyNotFoundException(key);
        try {
            return std::any_cast<T>(it->second);
        } catch (const std::bad_any_cast&) {
            throw ConfigWrongTypeAccessException(key);
        }
    }

    Config ARION_EXPORT clone() const {
        Config newConfig;
        newConfig.config_map = this->config_map;
        return newConfig;
    }
};

// rust bindings getter and setter
std::unique_ptr<Config> new_config();
arion::ARION_LOG_LEVEL get_log_level();

#endif // ARION_CONFIG_HPP
