ARG ARCH=x86_64

FROM node:18.17.1-buster  as node
ARG VITE_ENVIRONMENT
RUN mkdir -p /frontend
COPY frontend/package.json frontend/
COPY frontend/yarn.lock frontend/
RUN cd frontend && yarn install
COPY . .
RUN cd frontend && yarn build

FROM ohaddahan/brain-wars-base:${ARCH} AS chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS rust
COPY --from=planner recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY . .
RUN cargo build --release --frozen
#RUN cargo install sccache --locked
#COPY . .
#ENV SQLX_OFFLINE true
#RUN cargo build --release

FROM ohaddahan/brain-wars-base:${ARCH} AS runtime
RUN mkdir -p /frontend
RUN mkdir -p assets
RUN mkdir -p configuration
RUN mkdir -p /target/release
COPY configuration configuration
COPY migrations migrations
COPY --from=rust /target/release/backend /target/release/backend
COPY --from=rust /assets /backend/assets
COPY --from=node /frontend/dist /frontend/dist

ARG APP_ENVIRONMENT
ARG DATABASE_URL
ARG ASSETS_DIR
ARG VITE_DIR
ARG PORT
ARG VITE_ENVIRONMENT

ENTRYPOINT ["/bin/bash", "-c", "sqlx migrate run --source migrations && /target/release/backend"]