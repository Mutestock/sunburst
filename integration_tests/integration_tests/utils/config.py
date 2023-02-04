import rtoml
import os

_ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
_ROOT_DIR = os.path.dirname(_ROOT_DIR)
_ROOT_DIR = os.path.dirname(_ROOT_DIR)
INTEGRATION_TESTING_DIRECTORY = _ROOT_DIR
_ROOT_DIR = os.path.dirname(_ROOT_DIR)
SUNBURST_ROOT_DIR = _ROOT_DIR

LOG_FILE_PATH = SUNBURST_ROOT_DIR + "/logs"
CONFIG_PATH = SUNBURST_ROOT_DIR + "/config.toml"

CONFIG: dict = {}

for toml_file in [CONFIG_PATH]:
    with open(toml_file, "r") as file_reader:
        content = file_reader.read()
        if content:
            CONFIG.update(rtoml.loads(content))
            
            
            
def get_environment_specific_database_config(config: dict = CONFIG) -> dict:
    config_intermediate = {}
    
    if os.getenv("CONTAINERIZED"):
        config_intermediate = config["database"]["containerized"]
    else:
        config_intermediate = config["database"]["local"]
        
    return config_intermediate