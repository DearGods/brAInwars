FROM alpine:3.19.1 as builder
ARG GITHUB_TOKEN
WORKDIR /opt/
RUN apk add tmux curl protoc musl-dev gzip git nodejs npm jq
# tailwind
#RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
#  && chmod +x tailwindcss-linux-x64 \
#  && mv tailwindcss-linux-x64 tailwindcss
# for configuration, migrations and templates

# Set up git to use the token for authentication
RUN git config --global credential.helper 'store --file=/tmp/.gitcredentials'
RUN echo "https://x-access-token:$GITHUB_TOKEN@github.com" > /tmp/.gitcredentials
#RUN git clone https://${GITHUB_TOKEN}@github.com/ohaddahan/brAInwars.git
RUN git clone https://github.com/ohaddahan/brAInwars.git
RUN cp -fr brAInwars/backend/* .
RUN cd brAInwars/client/brain-war-client && npm install && npm run build
RUN cp -fr brAInwars/client/brain-war-client/dist ./
RUN mkdir tmp && cd tmp && \
    curl -sL -H "Authorization: token ${GITHUB_TOKEN}" "https://api.github.com/repos/ohaddahan/brAInwars/releases/latest" | \
    jq '.assets[1] | .id' | \
    xargs -I {} curl -L -s -H "Authorization: token ${GITHUB_TOKEN}" -H 'Accept:application/octet-stream' \
    "https://api.github.com/repos/ohaddahan/brAInwars/releases/assets/{}" \
    -o backend-x86_64-unknown-linux-musl.tar.gz \
    && tar -xvf backend-x86_64-unknown-linux-musl.tar.gz \
    && chmod +x backend \
    && mv backend /opt/backend-bin
RUN rm -fr /tmp/.gitcredentials
ARG APP_ENVIRONMENT
ARG DATABASE_URL
ARG ASSETS_DIR
ARG VITE_DIR
ARG PORT
ARG HOST
ARG VITE_ENVIRONMENT

#ENV FULL_BASE_PATH /opt/brAInwars/backend
#ENV WALLET_MNEMONIC " /opt/brAInwars/backend/fixtures/crank.json"
CMD ["/opt/backend-bin"]