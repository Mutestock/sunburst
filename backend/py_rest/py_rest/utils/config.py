import rtoml
import os

_ROOT_DIR = os.path.dirname(os.path.abspath(__file__))
_ROOT_DIR = os.path.dirname(_ROOT_DIR)
_ROOT_DIR = os.path.dirname(_ROOT_DIR)
_ROOT_DIR = os.path.dirname(_ROOT_DIR)
_ROOT_DIR = os.path.dirname(_ROOT_DIR)
SUNBURST_ROOT_DIR = _ROOT_DIR

LOG_FILE_PATH = SUNBURST_ROOT_DIR + "/logs"
CONFIG_PATH = SUNBURST_ROOT_DIR + "/config.toml"

CONFIG: dict = {}
DB_CONF: dict = {}
DISTRIBUTOR_CONF: dict = {}
PY_REST_CONF: dict = {}

for toml_file in [CONFIG_PATH]:
    with open(toml_file, "r") as file_reader:
        content = file_reader.read()
        if content:
            CONFIG.update(rtoml.loads(content))


if os.getenv("CONTAINERIZED"):
    DISTRIBUTOR_CONF = CONFIG["distributor"]["containerized"]
    PY_REST_CONF = CONFIG["rest"]["py"]["containerized"]
elif os.getenv("PRODUCTION"):
    DISTRIBUTOR_CONF = CONFIG["distributor"]["dev"]
    PY_REST_CONF = CONFIG["rest"]["py"]["prod"]
else:
    DISTRIBUTOR_CONF = CONFIG["distributor"]["dev"]
    PY_REST_CONF = CONFIG["rest"]["py"]["dev"]
