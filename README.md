# Stripe Update Card microservice

Expose a page that let your customers update their payment information on Stripe.

[![Travis](https://img.shields.io/travis/FGRibreau/stripe-update-card.svg)](https://travis-ci.org/FGRibreau/stripe-update-card)
[![Cargo version](https://img.shields.io/crates/v/stripe-update-card.svg)](https://crates.io/crates/stripe-update-card) [![Crates.io](https://img.shields.io/crates/l/stripe-update-card.svg)](https://crates.io/crates/stripe-update-card) [![Crates.io](https://img.shields.io/crates/d/stripe-update-card.svg)](https://crates.io/crates/stripe-update-card) [![Docker Build Status](https://img.shields.io/docker/build/fgribreau/stripe-update-card.svg)](https://hub.docker.com/r/fgribreau/stripe-update-card/) [![MicroBadger Size](https://img.shields.io/microbadger/image-size/fgribreau/stripe-update-card.svg)](https://hub.docker.com/r/fgribreau/stripe-update-card/) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)

<!-- [![codecov](https://codecov.io/gh/FGRibreau/stripe-update-card/branch/master/graph/badge.svg)](https://codecov.io/gh/FGRibreau/stripe-update-card) -->

<p align="center">
  <img src="docs/screencast.gif"/>  
</p>


### 🐳 Getting started

```bash
docker run -it \
 -e STRIPE_PUBLISHABLE_KEY=pk_test_xxx \
 -e STRIPE_SECRET_KEY=sk_test_xxx \
 -e SUCCESS_REDIRECT_URL=https://url.to.redirect/on/success \
 -p 8080:8080 \
 fgribreau/stripe-update-card

# open payment update page for customer id: cus_D1Cj3rjHrjPQg5
open http://localhost:8080/cus_XXXXXXXXXXXX
```

Don't forget to change `cus_XXXXXXXXXXXX` with the Stripe customer id and then expose the URL `http://sub.domain.tld/cus_XXXXXXXXXXXX` from your app.

## Configuration

Configuration is managed through environment variables, see [.env.dist](./.env.dist) for the full-list.


```bash
# mandatory config
STRIPE_PUBLISHABLE_KEY=pk_test_xxxxxxxx
STRIPE_SECRET_KEY=sk_test_xxxx
SUCCESS_REDIRECT_URL=https://url.to.redirect/on/success

# optional config
PAGE_TITLE="Update Card"
FORM_DATA_IMAGE=/path/to/your/logo.png
FORM_DATA_NAME="The name of your company or website"
FORM_DATA_DESCRIPTION="A description of the product or service being purchased"
FORM_DATA_PANEL_LABEL="Update Card Details"
FORM_DATA_COLLECT_BILLING_ADDRESS=false
FORM_DATA_LABEL="Update Card Details"
FORM_DATA_ALLOW_REMEMBER_ME=false
FORM_DATA_LOCALE=auto
```

See [stripe-update-card](https://hub.docker.com/r/fgribreau/stripe-update-card/) on docker hub.

## Running in production at

- [Image-Charts](https://payment.image-charts.com/cus_XXXXXX)
- [Redsmin](https://payment.redsmin.com/cus_XXXXXX)

## 🦄 Do you use it in production? Please support my work :)

<span class="badge-patreon"><a href="https://patreon.com/fgribreau" title="Donate to this project using Patreon"><img src="https://img.shields.io/badge/patreon-donate-yellow.svg" alt="Patreon donate button" /></a></span>
<span class="badge-gratipay"><a href="https://www.gratipay.com/fgribreau" title="Donate weekly to this project using Gratipay"><img src="https://img.shields.io/badge/gratipay-donate-yellow.svg" alt="Gratipay donate button" /></a></span>
<span class="badge-flattr"><a href="https://flattr.com/profile/fgribreau" title="Donate to this project using Flattr"><img src="https://img.shields.io/badge/flattr-donate-yellow.svg" alt="Flattr donate button" /></a></span>
<span class="badge-paypal"><a href="https://fgribreau.me/paypal" title="Donate to this project using Paypal"><img src="https://img.shields.io/badge/paypal-donate-yellow.svg" alt="PayPal donate button" /></a></span>
<span class="badge-bitcoin"><a href="https://www.coinbase.com/fgribreau" title="Donate once-off to this project using Bitcoin"><img src="https://img.shields.io/badge/bitcoin-donate-yellow.svg" alt="Bitcoin donate button" /></a></span>


## ⛴ Cargo install

```bash
cargo install stripe-update-card
```


## ⚙️ Deployment 

- Deploy it (the fastest way is to use [Clever-cloud](https://www.clever-cloud.com/doc/rust/rust/) thanks to their awesome native Rust support)
- Set environment variables
- Don't forget to add "RUSTUP_CHANNEL=nightly" env. variable for Rocket 
- Done!


## Sponsors

<table>
  <tr>
    <td align="center" width="175">
      <a href="https://france-nuage.fr/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=france-nuage&mtm_content=stripe-update-card">
        <img src="assets/sponsors/france-nuage.svg" height="60" alt="France-Nuage"/><br/>
        <b>France-Nuage</b>
      </a><br/>
      <sub>Sovereign French cloud for your billing stack. EU-only, no lock-in.</sub>
    </td>
    <td align="center" width="175">
      <a href="https://www.hook0.com/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=hook0&mtm_content=stripe-update-card">
        <img src="assets/sponsors/hook0.png" height="60" alt="Hook0"/><br/>
        <b>Hook0</b>
      </a><br/>
      <sub>Receive Stripe webhooks reliably. Self-hosted, signed, retries built-in.</sub>
    </td>
    <td align="center" width="175">
      <a href="https://getnatalia.com/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=natalia&mtm_content=stripe-update-card">
        <img src="assets/sponsors/natalia.svg" height="60" alt="Natalia"/><br/>
        <b>Natalia</b>
      </a><br/>
      <sub>Failed card? Natalia calls the customer 24/7 to recover the payment.</sub>
    </td>
    <td align="center" width="175">
      <a href="https://netir.fr/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=netir&mtm_content=stripe-update-card">
        <img src="assets/sponsors/netir.svg" height="60" alt="NetIR"/><br/>
        <b>NetIR</b>
      </a><br/>
      <sub>Hire vetted French freelance billing devs via mentored marketplace.</sub>
    </td>
  </tr>
  <tr>
    <td align="center" width="233">
      <a href="https://nobullshitconseil.com/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=nbc&mtm_content=stripe-update-card">
        <img src="assets/sponsors/nobullshitconseil.svg" height="60" alt="NoBullshitConseil"/><br/>
        <b>NoBullshitConseil</b>
      </a><br/>
      <sub>No-bullshit tech advisory. Billing &amp; SaaS strategy for founders.</sub>
    </td>
    <td align="center" width="233">
      <a href="https://qualneo.fr/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=qualneo&mtm_content=stripe-update-card">
        <img src="assets/sponsors/qualneo.svg" height="60" alt="Qualneo"/><br/>
        <b>Qualneo</b>
      </a><br/>
      <sub>Qualiopi LMS billing French trainees via Stripe. 32 indicators wired.</sub>
    </td>
    <td align="center" width="233">
      <a href="https://recapro.ai/?mtm_source=github&mtm_medium=sponsor&mtm_campaign=recapro&mtm_content=stripe-update-card">
        <img src="assets/sponsors/recapro.png" height="60" alt="Recapro"/><br/>
        <b>Recapro</b>
      </a><br/>
      <sub>Sovereign AI transcribes customer billing calls &amp; drafts the report.</sub>
    </td>
  </tr>
</table>

> **Interested in sponsoring?** [Get in touch](mailto:rust@fgribreau.com)

## Development


```
cargo install cargo-watch
cargo install cargo-release

source .env

# watch for changes and restart everytime
RUST_LOG=debug cargo watch -x run 
```

- Use Stripe test tokens
- Use credit card number `4242 4242 4242 4242` and any date & CVV


## CI

This project use Travis-CI to run tests & do builds.

Required environment variables are:

- CODECOV_TOKEN (get one at https://codecov.io/gh/FGRibreau/ )
