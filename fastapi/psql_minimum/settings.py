# pip install python-dotenv

from dotenv import load_dotenv
from pathlib import Path
import os

# Search "How to use Python pathlib"(/ here equlas to os.path.join)
env_path = Path('.') / '.env'
load_dotenv(dotenv_path=env_path)

PSQL_DATABASE_URL = os.getenv("PSQL_DATABASE_URL")
