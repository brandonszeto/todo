from configparser import ConfigParser

config = ConfigParser()

config["User"] = {
    "apitoken": 0,
}

with open("api_key.ini", "w") as f:
    config.write(f)
