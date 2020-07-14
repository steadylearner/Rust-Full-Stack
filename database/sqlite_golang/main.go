// https://github.com/mattn/go-sqlite3/issues/274
// https://github.com/mattn/go-sqlite3/issues/569

package main

import (
	"database/sql"
	"fmt"
	"strconv"
	"strings"

	"bufio"
	"os"

	"flag"

	_ "github.com/mattn/go-sqlite3"

	"github.com/pkg/errors"
	log "github.com/sirupsen/logrus"
	Models "steadylearner.com/sqlite/models"
)

var (
	action string
)

func important(err error, reason string) {
	if err != nil {
		log.Fatal(errors.Wrap(err, reason))
		return
	}
}

func createUser(db *sql.DB, username string) {
	tx, err := db.Begin()
	important(err, fmt.Sprintf("Fail to begin db to create the user %s.", username)) // Substitute it with logrus.

	cmd := "INSERT INTO users (username) values (?)"
	stmt, err := tx.Prepare(cmd)
	important(err, fmt.Sprintf("Fail to prepare sql statement(%s).", cmd)) // Substitute it with logrus.
	defer stmt.Close()

	// https://godoc.org/database/sql/driver#Result
	_, err = stmt.Exec(username)                                        // What is ok value here?
	important(err, fmt.Sprintf("Fail to create the user %s", username)) // Substitute it with logrus.

	// https://github.com/mattn/go-sqlite3/blob/master/sqlite3.go#L443
	tx.Commit()
}

func updateUser(db *sql.DB, id int, username string) {
	sid := strconv.Itoa(id) // int to string

	tx, err := db.Begin()
	important(err, fmt.Sprintf("Fail to begin db to update the username to %s.", username)) // Substitute it with logrus.

	cmd := "UPDATE users SET username=? where id=?"
	stmt, err := db.Prepare(cmd)
	important(err, fmt.Sprintf("Fail to prepare sql statement(%s).", cmd)) // Substitute it with logrus.
	defer stmt.Close()

	result, err := stmt.Exec(username, sid)
	important(err, fmt.Sprintf("Fail to update the user %s.", username)) // Substitute it with logrus.

	updatedRows, err := result.RowsAffected()
	important(err, fmt.Sprintf("Fail to find the number of rows affected with %s. ", cmd)) // Substitute it with logrus.

	fmt.Println(updatedRows)

	tx.Commit()
}

func getUser(db *sql.DB, id int) Models.User { // Models.User
	sid := strconv.Itoa(id) // int to string

	tx, err := db.Begin()
	important(err, fmt.Sprintf("Fail to begin db to get the user with id %s.", sid)) // Substitute it with logrus.

	cmd := fmt.Sprintf("SELECT * FROM users WHERE id=%s", sid)
	rows, err := tx.Query(cmd)                                           // https://godoc.org/database/sql/driver#Rows
	important(err, fmt.Sprintf("Fail to query sql statement(%s).", cmd)) // Substitute it with logrus.
	defer rows.Close()

	if rows.Next() {
		var user Models.User

		err = rows.Scan(&user.Id, &user.Username)
		important(err, fmt.Sprintf("Fail to convert data from SQLite to the Models.User type.")) // Substitute it with logrus.
		return user
	}
	err = rows.Err()
	important(err, "Problem while reading a row in getUser function.")

	return Models.User{}
}

func listUsers(db *sql.DB) []Models.User {
	rows, err := db.Query("SELECT * FROM users")
	important(err, "Fail to list users") // Substitute it with logrus.

	var users []Models.User
	for rows.Next() {
		var user Models.User
		err = rows.Scan(&user.Id, &user.Username)
		important(err, fmt.Sprintf("Fail to convert data from SQLite to the Models.User type.")) // Substitute it with logrus.
		users = append(users, user)
	}
	err = rows.Err()
	important(err, "Problem while reading rows in listUsers function.")

	return users
}

func deleteUser(db *sql.DB, id int) {
	tx, err := db.Begin()
	important(err, fmt.Sprintf("Fail to begin db to delete the user with id %d.", id)) // Substitute it with logrus.

	sid := strconv.Itoa(id)

	cmd := "DELETE FROM users WHERE id=?"

	stmt, err := tx.Prepare(cmd)
	important(err, fmt.Sprintf("Fail to prepare sql statement(%s).", cmd)) // Substitute it with logrus.
	result, err := stmt.Exec(sid)
	important(err, fmt.Sprintf("Fail to delete the user with id %d", id)) // Substitute it with logrus
	num, err := result.RowsAffected()
	important(err, fmt.Sprintf("Fail to find the number of rows affected with %s", cmd))
	fmt.Printf("%d user deleted.\n", num)

	tx.Commit()
}

func init() {
	actionHelp := "Use [list, get, create, update, delete] to handle the data from users."
	flag.StringVar(&action, "action", "list", actionHelp)
	flag.Parse()
}

func main() {
	target := "database/users.db"

	db, err := sql.Open("sqlite3", target)
	important(err, fmt.Sprintf("Couldn't open %s", target))
	defer db.Close()

	// https://www.sqlitetutorial.net/sqlite-autoincrement/
	// db.Exec("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE)")

	// db.Exec("DROP TABLE users")
	// db.Exec("CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL UNIQUE)")

	switch action {
	case "list":
		// fmt.Println(listUsers(db))
		// fmt.Printf("%#v\n", listUsers(db))
		fmt.Printf("%+v\n", listUsers(db))
	case "get":
		idReader := bufio.NewReader(os.Stdin)
		fmt.Println("Type the id of a user get the data.")
		id, err := idReader.ReadString('\n')
		important(err, "Fail to read the id to get the data of a user.") // Substitute it with logrus

		if id == "\n" {
			fmt.Println("You should provide an id to make this work.")
		} else {
			i, err := strconv.Atoi(strings.TrimSuffix(id, "\n"))
			important(err, fmt.Sprintf("Fail to convert given id(%s) to int.", id)) // Substitute it with logrus

			// fmt.Println(getUser(db, i))
			// fmt.Printf("%#v\n", getUser(db, i))
			fmt.Printf("%+v\n", getUser(db, i))
		}
	case "create":
		usernameReader := bufio.NewReader(os.Stdin)
		fmt.Println("Type a username you want to use.")
		username, err := usernameReader.ReadString('\n')
		important(err, "Fail to read a username.") // Substitute it with logrus

		if username == "\n" {
			fmt.Println("You should provide username to make it work.")
		} else {
			createUser(db, strings.TrimSuffix(username, "\n"))
		}
	case "update":
		idReader := bufio.NewReader(os.Stdin)
		fmt.Println("Type the id of a user to update its username.")
		id, err := idReader.ReadString('\n')
		important(err, "Fail to read the id of a user to be updated.") // Substitute it with logrus

		if id == "\n" {
			fmt.Println("You should provide an id to make this work.")
			return
		}

		newUsernameReader := bufio.NewReader(os.Stdin)
		fmt.Println("Type a new username you want to use.")
		newUsername, err := newUsernameReader.ReadString('\n')
		important(err, "Fail to read a new username.") // Substitute it with logrus

		if newUsername == "\n" {
			fmt.Println("You should provide username to make it work.")
			return
		}

		i, err := strconv.Atoi(strings.TrimSuffix(id, "\n"))
		important(err, fmt.Sprintf("Fail to convert given id(%s) to int.", id)) // Substitute it with logrus

		updateUser(db, i, strings.TrimSuffix(newUsername, "\n"))
	case "delete":
		idReader := bufio.NewReader(os.Stdin)
		fmt.Println("Type the id of a user to delete.")
		id, err := idReader.ReadString('\n')
		important(err, "Fail to read the id to delete the data of a user.") // Substitute it with logrus

		if id == "\n" {
			fmt.Println("You should provide the id to make it work.")
		} else {
			i, err := strconv.Atoi(strings.TrimSuffix(id, "\n"))
			important(err, fmt.Sprintf("Fail to convert given id(%s) to int.", id)) // Substitute it with logrus

			deleteUser(db, i)
		}
	default:
		fmt.Println("You should use [list, get, create, update, delete].")
	}
}
