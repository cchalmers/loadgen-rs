let
  sources = import nix/sources.nix;

  loadgen-overlay = super: self: {
    concurrentqueue = self.callPackage nix/concurrentqueue.nix {};
    loadgen = self.callPackage nix/loadgen.nix { inference-src = sources.inference; };
  };

  moz-overlay = import sources.nixpkgs-mozilla;
  rust-overlay = self: super:
    let rust-nightly = super.rustChannelOf { date = "2020-05-04"; };
    in {
      rustc = rust-nightly.rust;
      cargo = rust-nightly.cargo;
    };

  nixpkgs = import sources.nixpkgs { overlays = [ moz-overlay rust-overlay loadgen-overlay ]; };
  naersk = nixpkgs.callPackage sources.naersk {};
  nixc = import ./nixc { inherit nixpkgs; };

  filterSource = with nixpkgs.lib; builtins.filterSource (path: type:
      type != "unknown" &&
      baseNameOf path != "target" &&
      baseNameOf path != "result" &&
      baseNameOf path != ".git" &&
      baseNameOf path != ".gitignore" &&
      !(hasSuffix ".nix" path) &&
      (baseNameOf path == "build" -> type != "directory") &&
      (baseNameOf path == "nix" -> type != "directory")
      );

  crate2nix = nixpkgs.callPackage (nixpkgs.srcOnly {
      name = "crate2nix-src";
      src = builtins.fetchTarball {
        # Using this pre-release because it contains fixes for hacks we're currently doing. Update
        # to 0.7 when it's released.
        url = https://github.com/kolloch/crate2nix/archive/crate2nix-v0.7.0-rc.2.tar.gz;
        sha256 = "0p2ja8haafizjlnn0608hc0glfjkknwabqlnv7yrfgk92z1z563v";
      };
    }) {};

  crateNix = nixpkgs.callPackage ./Cargo.nix { defaultCrateOverrides = crateOverrides; };
  crateOverrides = nixpkgs.defaultCrateOverrides // {
    loadgen = with nixpkgs; old: {
      LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
      buildInputs = old.buildInputs or [] ++ [pkg-config nixpkgs.loadgen];
    };
  };

in rec {
  inherit crateNix;
  loadgen-crate = crateNix.rootCrate.build;
  shell = nixpkgs.stdenv.mkDerivation {
    name = "shell";
    src = ":";
    buildInputs = [
      crate2nix
      nixpkgs.pkg-config
      nixpkgs.loadgen
    ];

    LIBCLANG_PATH = "${nixpkgs.llvmPackages.libclang}/lib";
  };
  loadgen = nixpkgs.loadgen;
  loadgen-rs = naersk.buildPackage {
    buildInputs = [
      nixpkgs.loadgen
      nixpkgs.pkg-config
    ];

    root = filterSource ./.;

    LIBCLANG_PATH = "${nixpkgs.llvmPackages.libclang}/lib";
  };
}
