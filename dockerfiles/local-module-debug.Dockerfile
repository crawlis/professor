##################################################
#                                                #
# THIS IMAGE SHOULD BE USED TO:                  #
#   - Run the microservice you are developping   #
#   - Run it in debug mode                       #
#                                                #
##################################################


# This is a development dockerfile optimized to :
#   - Reduce the build time: non-project binaries are cached
#   - Reduce the image space: the project is installed as a binary runnable from scratch image

##################################################
#                                                #
# BUILDER                                        #
#                                                #
# Debian-based image for openssl compillation    #
#                                                #
##################################################
FROM ekidd/rust-musl-builder as builder

ARG MODULE_NAME=module_base

RUN rustup self update
RUN rustup target add x86_64-unknown-linux-musl

# Create a new empty shell project to cache dependencies
RUN USER=root cargo new --bin --vcs none ${MODULE_NAME}
WORKDIR /home/rust/src/${MODULE_NAME}
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --target x86_64-unknown-linux-musl
RUN rm src/*.rs && \
    rm -rf ./target/x86_64-unknown-linux-musl/debug/deps/${MODULE_NAME}*

# Install the binary
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl
RUN chmod +x ./target/x86_64-unknown-linux-musl/debug/${MODULE_NAME}

##################################################
#                                                #
# SCRATCH                                        #
#                                                #
# Empty image to execute binary                  #
#                                                #
##################################################
FROM scratch

ARG MODULE_NAME=module_base

# Adding the binary
COPY --from=builder /home/rust/src/${MODULE_NAME}/target/x86_64-unknown-linux-musl/debug/${MODULE_NAME} ./app

# Adding SSL certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ["./app"]