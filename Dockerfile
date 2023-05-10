# we use the latest rust stable release as base image
FROM rust:1.63.0

# let's switch out working directory 'app' (equivalent to 'cd app')
# the 'app' folder will be created for us by Docker in case it does not
# exist already
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN APT update && apt install lld clang -y 

# Copy all files from our working environment to our Docker image
COPY . .
# Let's build our binary!
# We'll use the release profile to make it faaaaast
RUN cargo build --release
#  When 'docker run' is executed, launc the binary
ENTRYPOINT ["./target/release/zero2prod"] 