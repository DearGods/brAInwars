FROM rust:1.75.0-buster
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y pkg-config build-essential libudev-dev
RUN bash -c "$(curl -sSfL https://release.solana.com/v1.17.16/install)"
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
RUN cargo install cargo-chef --locked --version 0.1.62
RUN cargo install sqlx-cli --locked --version 0.7.3
RUN bash -c "source ~/.bashrc && nvm install 18.17.1"
#RUN cargo init --name dummy
# Copy only the dependencies
#COPY Cargo.toml /Cargo.toml
#COPY Cargo.lock /Cargo.lock
#RUN mkdir -p /brain-wars
#COPY . /brain-wars
#WORKDIR /brain-wars
#RUN cargo build --release --locked
SHELL ["/bin/bash", "-c"]