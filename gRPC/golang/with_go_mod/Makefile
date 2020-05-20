# $make help 
# Find how to use this.

SERVER_OUT := "server.bin"
CLIENT_OUT := "client.bin"
API_OUT := "api/api.pb.go"
PKG := "gitlab.com/pantomath-io/demo-grpc"
SERVER_PKG_BUILD := "${PKG}/server"
CLIENT_PKG_BUILD := "${PKG}/client"
PKG_LIST := $(shell go list ${PKG}/... | grep -v /vendor/)

.PHONY: all api build_server build_client

all: build_server build_client

api/api.pb.go: api/api.proto
	@protoc -I api/ \
		-I${GOPATH}/src \
		--go_out=plugins=grpc:api \
		api/api.proto

api: api/api.pb.go ## Auto-generate grpc go sources

dep: ## Get the dependencies
	@go get -v -d ./...

build_server: dep api ## Build the binary file for server
	@go build -i -v -o $(SERVER_OUT) $(SERVER_PKG_BUILD)

build_client: dep api ## Build the binary file for client
	@go build -i -v -o $(CLIENT_OUT) $(CLIENT_PKG_BUILD)

clean: ## Remove previous builds
	@rm $(SERVER_OUT) $(CLIENT_OUT) $(API_OUT)

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

