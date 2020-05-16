let sources = import nix/sources.nix;

    loadgen-overlay = super: self: {
      concurrentqueue = self.callPackage nix/concurrentqueue.nix {};
      loadgen = self.callPackage nix/loadgen.nix { inference-src = sources.inference; };
    };

    moz-overlay = import sources.nixpkgs-mozilla;

    nixpkgs = import sources.nixpkgs { overlays = [ moz-overlay loadgen-overlay ]; };
    naersk = nixpkgs.callPackage sources.naersk {};
    nixc = import ./nixc { inherit nixpkgs; };

    filterSource = with nixpkgs.lib; builtins.filterSource (path: type:
        type != "unknown" &&
        baseNameOf path != "target" &&
        baseNameOf path != "result" &&
        baseNameOf path != ".git" &&
        # Exceptions for files starting with .
        (baseNameOf path == "build" -> type != "directory") &&
        (baseNameOf path == "nix" -> type != "directory")
        );

# in naersk.buildPackage {

#     hardeningDisable = [ "all" ];

#     buildInputs = [
#       nixpkgs.loadgen
#       nixpkgs.pkg-config
#     ];

#     LIBCLANG_PATH = "${nixpkgs.llvmPackages.libclang}/lib";
#     PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

#     root = filterSource ./.;
#   }

in rec {
  shell = nixpkgs.stdenv.mkDerivation {
    name = "shell";
    buildInputs = [
      # nixpkgs.pkg-config
      nixpkgs.loadgen
      # nixpkgs.clang
      # nixpkgs.llvm
      nixpkgs.cmake
      nixpkgs.libcxx
    ];
    hardeningDisable = [ "all" ];
    LIBCLANG_PATH = "${nixpkgs.llvmPackages.libclang}/lib";
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";
    LOADGEN_PATH = "${nixpkgs.loadgen}";
  };
  loadgen = nixpkgs.loadgen;
  loadgen-rs = naersk.buildPackage {

    hardeningDisable = [ "all" ];

    buildInputs = [
      nixpkgs.loadgen
      nixpkgs.pkg-config
      # nixpkgs.llvmclang
      nixpkgs.llvm
    ];

    LIBCLANG_PATH = "${nixpkgs.llvmPackages.libclang}/lib";
    PKG_CONFIG_PATH = "${nixpkgs.openssl.dev}/lib/pkgconfig";

    root = filterSource ./.;
  };
}
