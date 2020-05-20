// https://github.com/moby/moby/tree/master/client

// $go mod init github.com/steadylearner/docker
// $go get github.com/docker/docker/client github.com/docker/docker/api/types

package main

import (
	"context"
	"fmt"

	"github.com/docker/docker/api/types"
	"github.com/docker/docker/client"
)

// https://github.com/moby/moby/issues/40185
// It shows only active containers
// Use $docker restart <containername> first if there were none.
func main() {
	// cli, err := client.NewClientWithOpts(client.FromEnv)
        // Use old methods to make this work.
	cli, err := client.NewEnvClient()
        if err != nil {
		panic(err)
	}

	containers, err := cli.ContainerList(context.Background(), types.ContainerListOptions{})
	if err != nil {
		panic(err)
	}

	for _, container := range containers {
		fmt.Printf("%s %s\n", container.ID[:10], container.Image)
	}
}
