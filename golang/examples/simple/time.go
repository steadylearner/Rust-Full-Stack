package main

import (
    "log"
    "time"
    "fmt"
    "reflect"
    "runtime"
    "strings"
)

func timeTrack(start time.Time, name string) {
        elapsed := time.Since(start)
        log.Printf("function %s took %s", name, elapsed)
}

func GetFunctionName(i interface{}) string {
    name := runtime.FuncForPC(reflect.ValueOf(i).Pointer()).Name() // package.something
    without_package = strings.Split(name, ".")[1]
    return without_package
}

func main() {
        defer timeTrack(time.Now(), "main")
        // or defer timeTrack(time.Now(), GetFunctionName(main))
        for i := 0; i < 100; i++ {
                fmt.Printf("I can count: %d\n", i)
        }
}


