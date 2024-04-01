{pkgs ? import <nixpkgs> {}}: let
  lib = pkgs.lib;
  name = "adapted-chaum-pederson";
  ver = "0.1.0";
  homepage = "https://github.com/glottologist/adapted-chaum-pederson";
  description = "Chaum-Pederson adapted for single factor auth.";
  license = lib.licences.mit;
  maintainers = with lib.maintainers; [
    {
      name = "Jason Ridgway-Taylor";
      email = "jason@glottologist.co.uk";
      github = "glottologist";
    }
  ];
in
  pkgs.rustPlatform.buildRustPackage rec {
    inherit name ver;
    pname = name;
    version = ver;
    src = pkgs.lib.cleanSource ./.;
    # Specify the binary that will be installed
    cargoBinName = name;

    buildInputs = with pkgs; [
      openssl
      protobuf
    ];

    preConfigure = ''
      export PROTOBUF_LOCATION=${pkgs.protobuf}
      export PROTOC=$PROTOBUF_LOCATION/bin/protoc
      export PROTOC_INCLUDE=$PROTOBUF_LOCATION/include
    '';

    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    # The package manager needs to know the SHA-256 hash of your dependencies
    cargoSha256 = "565hrIUXGuOHoxiUEh5CsgUWgD3nUTNKGwZ2b+4FWog=";

    meta = with pkgs.stdenv.lib; {
      inherit maintainers homepage description licenses;
    };
  }
