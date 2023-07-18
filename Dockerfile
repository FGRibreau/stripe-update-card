FROM rust:1.71-slim-bullseye as builder
ADD . /app
WORKDIR /app
# Make sure that this matches in .travis.yml
ARG RUST_TOOLCHAIN=1.71.0
RUN \
    rustup install ${RUST_TOOLCHAIN} && \
    rustup default ${RUST_TOOLCHAIN} && \
    cargo --version && \
    rustc --version && \
    mkdir -m 755 bin && \
    cargo build --release && \
    cp /app/target/release/stripe-update-card /app/bin && \
    cp -R -v /app/static /app/bin


FROM debian:bookworm-slim
MAINTAINER Francois-Guillaume Ribreau <docker@fgribreau.com>

COPY --from=builder /app/bin /app/bin

WORKDIR /app/bin

# mandatory settings
ENV STRIPE_PUBLISHABLE_KEY pk_test_xxxxxxxx
ENV STRIPE_SECRET_KEY sk_test_xxxx
ENV SUCCESS_REDIRECT_URL https://url.to.redirect/on/success

EXPOSE 8080

CMD ["./stripe-update-card"]
