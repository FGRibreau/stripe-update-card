FROM rust:1.26.0-stretch as builder
ADD . /app
WORKDIR /app
# Make sure that this matches in .travis.yml
ARG RUST_TOOLCHAIN=nightly
RUN \
    rustup default ${RUST_TOOLCHAIN} && \
    cargo --version && \
    rustc --version && \
    mkdir -m 755 bin && \
    cargo build --release && \
    cp /app/target/release/stripe-update-card /app/bin


FROM debian:stretch-slim
MAINTAINER Francois-Guillaume Ribreau <docker@fgribreau.com>


COPY --from=builder /app/bin /app/bin

WORKDIR /app
USER app

# mandatory settings
ENV STRIPE_PUBLISHABLE_KEY pk_test_xxxxxxxx
ENV STRIPE_SECRET_KEY sk_test_xxxx
ENV SUCCESS_REDIRECT_URL https://url.to.redirect/on/success

# server settings
ENV PORT 8080
ENV ROCKET_ADDRESS 0.0.0.0

CMD ["/app/bin/stripe-update-card"]