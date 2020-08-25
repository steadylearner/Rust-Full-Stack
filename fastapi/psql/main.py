# 1. Include migration.
# 2. Validate email.
# 3. Include tests/

# # 1. Solve Pylint import problem with .vscode/settings.json
# https://donjayamanne.github.io/pythonVSCodeDocs/docs/troubleshooting_linting/
# https://supunkavinda.blog/vscode-editing-settings-json#folder

# https://stackoverflow.com/questions/40022220/attempted-relative-import-beyond-toplevel-package

# RecursionError: maximum recursion depth exceeded while getting the str of an object
# When I hit this issue, it was because there was a circular reference in
# my database schema. I had accounts.owner_id referencing users.id,
# and users.account_id referencing accounts.id,
# which caused infinite recursion ending in the above backtrace.

# 1. Include tests/. (https://fastapi.tiangolo.com/tutorial/testing/)

# 2. Validate email.
# 3. Make BrowserSession with id, session_id, identity_id
# 4. Make routes/ (Extract /api/v1/)

# 1. Make models
#    users(id(user_id) with uuid, name, date_of_birth, password(hashed with argon)),
#    sessions(id, session_id, identity_id, user_id)
#    profils(id, description, image, user_id, view_count),
# 2. Make auth process work.

from typing import List

from fastapi import Depends, FastAPI, HTTPException
from sqlalchemy.orm import Session

import models

from schemas.user import (
    User,
    UserCreate,
)
from schemas.item import (
    Item,
    ItemCreate
)

from handlers.user import (
    get_user_by_email,
    get_user,
    get_users,
    create_user,
)
from handlers.item import (
    get_items,
    create_user_item
)

from db.psql import (
    SessionLocal,
)

app = FastAPI()


# Dependency
def get_db():
    try:
        db = SessionLocal()
        yield db
    finally:
        db.close()

# All the unidentified was from the naming collision.
# Could handle it by using register_user instead.

# Extract "/api/v1" from paths later.
@app.post("/api/v1/users/", response_model=User)
def register_user(user: UserCreate, db: Session = Depends(get_db)):
    db_user = get_user_by_email(db, email=user.email)
    if db_user:
        raise HTTPException(status_code=400, detail="Email already registered")
    return create_user(db=db, user=user)


@app.get("/api/v1/users/", response_model=List[User])
def read_users(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    users = get_users(db, skip=skip, limit=limit)
    return users


@app.get("/api/v1/users/{user_id}", response_model=User)
def read_user(user_id: int, db: Session = Depends(get_db)):
    db_user = get_user(db, user_id=user_id)
    if db_user is None:
        raise HTTPException(status_code=404, detail="User not found")
    return db_user


@app.post("/api/v1/users/{user_id}/items/", response_model=Item)
def register_item_for_user(
    user_id: int, item: ItemCreate, db: Session = Depends(get_db)
):
    return create_user_item(db=db, item=item, user_id=user_id)


@app.get("/api/v1/items/", response_model=List[Item])
def read_items(skip: int = 0, limit: int = 100, db: Session = Depends(get_db)):
    items = get_items(db, skip=skip, limit=limit)
    return items
