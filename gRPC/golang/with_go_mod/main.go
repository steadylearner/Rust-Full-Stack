package main

import (
	"context"
	"log"
	"net"

	"google.golang.org/grpc"
	pb "steadylearner.com/grpc/helloworld"
)

const (
	port = ":50051"
)

// server is used to implement pb.pb.GreeterServer.
type server struct {
}

func (s *server) SayHello(ctx context.Context, in *pb.HelloRequest) (*pb.HelloReply, error) {
	log.Printf("Received: %v", in.GetName())
	return &pb.HelloReply{Message: "Hello " + in.GetName()}, nil
}

func main() {
	// fmt.Println("%s", pb)
	lis, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := grpc.NewServer()

	pb.RegisterGreeterServer(s, &server{})
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
