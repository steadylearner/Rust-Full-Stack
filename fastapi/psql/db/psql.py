# Turn it to a folder later and include psql.py and __init__.py?

# pip install psycopg2
# Refer to this when you want to use Docker.
# $docker volume create postgresqldata
# $docker run -d -v postgresqldata:/data/db --name postgresql -p 5432:5432 postgresql

from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

from settings import PSQL_DATABASE_URL

from sqlalchemy_utils import create_database, database_exists

if not database_exists(PSQL_DATABASE_URL):
    create_database(PSQL_DATABASE_URL)

SQLALCHEMY_DATABASE_URL = PSQL_DATABASE_URL

engine = create_engine(
    SQLALCHEMY_DATABASE_URL,
)
SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

Base = declarative_base()
