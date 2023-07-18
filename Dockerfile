FROM rust:1.26.0-stretch as builder
ADD . /app
WORKDIR /app
# Make sure that this matches in .travis.yml
ARG RUST_TOOLCHAIN=nightly-2018-06-09
RUN \
    rustup install ${RUST_TOOLCHAIN} && \
    rustup default ${RUST_TOOLCHAIN} && \
    cargo --version && \
    rustc --version && \
    mkdir -m 755 bin && \
    cargo build --release && \
    cp /app/target/release/stripe-update-card /app/bin && \
    cp -v /app/Rocket.toml /app/bin && \
    cp -R -v /app/templates /app/bin


FROM debian:stretch-slim
MAINTAINER Francois-Guillaume Ribreau <docker@fgribreau.com>


COPY --from=builder /app/bin /app/bin

WORKDIR /app/bin

# mandatory settings
ENV STRIPE_PUBLISHABLE_KEY pk_test_xxxxxxxx
ENV STRIPE_SECRET_KEY sk_test_xxxx
ENV SUCCESS_REDIRECT_URL https://url.to.redirect/on/success

# server settings
ENV ROCKET_ADDRESS 0.0.0.0
ENV ROCKET_PORT 8080
ENV ROCKET_ENV production

EXPOSE 8080

CMD ["./stripe-update-card"]