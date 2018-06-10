FROM scorpil/rust:nightly-onbuild
MAINTAINER Francois-Guillaume Ribreau <docker@fgribreau.com>

ENV STRIPE_PUBLISHABLE_KEY pk_test_xxxxxxxx
ENV STRIPE_SECRET_KEY sk_test_xxxx
ENV SUCCESS_REDIRECT_URL https://url.to.redirect/on/success