// [[bin]]
// name = "grpc"
// path = "src/grpc/main.rs"

const projects = process.argv.slice(2);

console.log("\n")

projects.map(project => console.log(`[[bin]]
name = ${project}
path = src/${project}/main.rs`
));

console.log("\n")
