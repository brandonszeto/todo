from configparser import ConfigParser
from todoist_api_python.api import TodoistAPI

config = ConfigParser()
config.read("api_key.ini")

apitoken = input("Enter your API token to authenticate:") 
api = TodoistAPI(apitoken)

try:
    projects = api.get_projects()
    print(projects)
except Exception as error:
    print(error)
    print('Invalid API or no API provided')
