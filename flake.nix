{
  description = "Flake for Adapted Chaum-Pederson";

  inputs = {
    nixpkgs.url = "github:glottologist/nixpkgs/master";
    fenix.url = "github:nix-community/fenix";
    devenv.url = "github:cachix/devenv";
    devenv.inputs.nixpkgs.follows = "nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-substituters = [
      "https://tweag-jupyter.cachix.org"
      "https://devenv.cachix.org"
    ];
    extra-trusted-public-keys = [
      "tweag-jupyter.cachix.org-1:UtNH4Zs6hVUFpFBTLaA4ejYavPo5EFFqgd7G7FxGW9g="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
    ];
  };

  outputs = inputs @ {
    flake-parts,
    flake-utils,
    nixpkgs,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = inputs.nixpkgs.lib.systems.flakeExposed;

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: rec {
        packages = rec {
          default = pkgs.callPackage ./default.nix {inherit pkgs;};
        };

        devenv.shells.default = {
          name = "Shell for Adapted Chaum-Pederson";
          env.GREET = "Welcome to the ACP dev shell";
          packages = with pkgs; [
            git
            mdbook
            mdbook-i18n-helpers
            mdbook-mermaid
            protobuf
          ];
          enterShell = ''
            export PROTOBUF_LOCATION=${pkgs.protobuf}
            export PROTOC=$PROTOBUF_LOCATION/bin/protoc
            export PROTOC_INCLUDE=$PROTOBUF_LOCATION/include
            git --version
            rustc --version
            cargo --version
            mdbook --version
          '';
          languages = {
            rust.enable = true;
            rust.channel = "nightly";
            nix.enable = true;
          };

          scripts.client.exec = ''
            cargo run --bin client
          '';
          scripts.server.exec = ''
            cargo run --bin server
          '';

          dotenv.enable = true;
          difftastic.enable = true;
          pre-commit = {
            hooks = {
              alejandra.enable = true;
              commitizen.enable = true;
              cargo-check.enable = true;
              clippy.enable = true;
              rustfmt.enable = true;
            };
            settings.rust.cargoManifestPath = "./Cargo.toml";
          };
        };
      };
    };
}
