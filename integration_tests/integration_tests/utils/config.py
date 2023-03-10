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
DB_CONF: dict = {}
DISTRIBUTOR_CONF: dict = {}
RS_REST_CONF: dict = {}
PY_REST_CONF: dict = {}
CS_REST_CONF: dict = {}
JS_REST_CONF: dict = {}

for toml_file in [CONFIG_PATH]:
    with open(toml_file, "r") as file_reader:
        content = file_reader.read()
        if content:
            CONFIG.update(rtoml.loads(content))


if os.getenv("CONTAINERIZED"):
    DB_CONF = CONFIG["database"]["containerized"]
    DISTRIBUTOR_CONF = CONFIG["distributor"]["containerized"]
    RS_REST_CONF = CONFIG["rest"]["rs"]["containerized"]
    PY_REST_CONF = CONFIG["rest"]["py"]["containerized"]
    CS_REST_CONF = CONFIG["rest"]["cs"]["containerized"]
    JS_REST_CONF = CONFIG["rest"]["js"]["containerized"]
else:
    DB_CONF = CONFIG["database"]["local"]
    DISTRIBUTOR_CONF = CONFIG["distributor"]["dev"]
    RS_REST_CONF = CONFIG["rest"]["rs"]["dev"]
    PY_REST_CONF = CONFIG["rest"]["py"]["dev"]
    CS_REST_CONF = CONFIG["rest"]["cs"]["dev"]
    JS_REST_CONF = CONFIG["rest"]["js"]["dev"]
