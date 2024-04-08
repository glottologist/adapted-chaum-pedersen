# Adapted-Chaum-Pedersen

This repo is an example of adapting the Chaum–Pedersen protocol for 1 factor authentication.

## Usage

You can run the client-server authentication in a few different ways:

- Nix
- Rust
- Docker

### Nix

#### Install Nix

Full instructions on how to install Nix can be found [here](https://nixos.org/download/) but the recommended why is:

```bash
sh <(curl -L https://nixos.org/nix/install) --daemon
```

#### Enable Flakes

There are a few ways to enable flakes depending on environment; they are listed [here](https://nixos.wiki/wiki/flakes#Installing_flakes).

#### Usage

After flakes have been enabled, the environment can be bootstrapped by running:

```
nix develop --impure
```

> Note: You will need to do this in two different terminal windows; one for the server and one for the client

In the first terminal simply run:

```
server
```
This will run the server half locally on port 8080.

In the second window, you will need to register your secret password first.  This can be done by running:

```
register <username>
```
You will be asked to enter a secret password that corresponds to the *username*.

Once registered, you can attempt to authenticate by running:

```
authenticate <username>
```
You will once again be asked to enter the secret password, and if it is the same password that you registered, you should get a successful authentication message.














