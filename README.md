# UP Bank to FireFly III Data Importer

**This is still a work in progress**

This program is designed to import transaction from [Up](https://up.com.au) bank into [Firefly III](https://firefly-iii.org/). It is able to import transactions from Up Bank into Firefly III either as a once off or continuously. It is also able to detect if a transaction has been updated such as a new tag is added or category change in the Up Bank portal which will then update the transaction in Firefly to reflect this change.

---

## Build from Source

Building is simple, just ensure you have **rust** and **cargo** installed on your system:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone the repository and build:

```bash
cargo build
```

or

```bash
cargo build --release
```

---

## Setup

### Step 1: Config File

1. Duplicate `settings-template.yaml` as `settings.yaml` in the same directory.
2. Modify the settings.yaml file with your details:

   - Add your **Up** personal access token. Generate a token here: [https://api.up.com.au](https://api.up.com.au)
   - Add your **Firefly** personal access token. Go to `Options` -> `Profile` -> `OAuth` -> `Create new token`.
   - Add the URL to your Firefly instance (eg `192.168.0.2:8083`)
   - (optional) Import period in hours (for "periodic import" mode)

### Step 2: Firefly III Setup

**Note**: You'll need a list of the Up IDs for each account. You can get these by:

   - Running `up_bank_fidi get-account-information`, or
   - Using the Up API directly, for example, using Postman with your personal access token.

Then set up and link Firefly accounts for your Up accounts:

1. Create a Firefly account for each of your Up accounts ("_Spending_" account, plus each "_Saver_").
2. Set the Firefly `account number` field to the associated Up account's unique ID.
3. Add the Up ID into the accounts section in the settings.yaml, this tells the importer to only import this data.

---

## CLI Usage

Ensure you have setup your settings.yaml file before continuing.
Run the CLI tool using `up_bank_fidi` (or `up_bank_fidi.exe` on Windows).

- Command help:
  ```bash
  up_bank_fidi -h
  ```
- Getting Up bank account details:
  ```bash
  up_bank_fidi get-account-information
  ```
- Manually run an import (useful for testing):
  ```bash
  up_bank_fidi import
  ```
- Running the migrator tool:
  ```bash
  up_bank_fidi
  ```

---

## Docker

- This program is best used from a docker container.
- Provided is both a Dockerfile and a template `compose.yml` file.
- Docker compose is setup to be built from a repo clone.
- You may also point your host volumes elsewhere if required.

Example `compose.yml` file:

```yaml
services:
  up_fidi:
    build: .
    container_name: up_fidi
    restart: unless-stopped
    environment:
      - RUST_LOG=info
      - ACTION=periodic-import
    volumes:
      - ./config/settings.yaml:/config/settings.yaml
      - ./logs:/logs
      - /etc/localtime:/etc/localtime:ro
```