# How to use it

Use refer if you want to follow the minimal example.

## Examples

1. query user

```javascript
query {
  user(id: "It works") {
    id
    firstName
    lastName
    dateOfBirth
  }
}
```

2. createUser

```javascript
mutation {
  createUser(newUser: {
    firstName: "It also works!"
    lastName: "It also works!"
    dateOfBirth: "It also works!"
  }) {
    id
    firstName
    lastName
    dateOfBirth
  }
}
```
