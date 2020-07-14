# I used this to make the file from helloworld.proto in the current repository. 
protoc -I helloworld/ helloworld/helloworld.proto --go_out=plugins=grpc:helloworld # helloworld is variable, you can use api etc.
