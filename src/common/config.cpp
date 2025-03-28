#include "arion/common/config.hpp"
#include <memory>

namespace arion {

    std::unique_ptr<Config> new_config() {
        return std::make_unique<Config>();
    }    
}
