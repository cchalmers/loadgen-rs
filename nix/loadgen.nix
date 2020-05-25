{ inference-src, stdenv, cmake, python36Packages }:

stdenv.mkDerivation rec {
  pname = "loadgen";

  # This is according to loadgen/version_generator.py in the source
  version = ".5a1";

  src = inference-src;

  # The original c header file was in a bindings dir. Could either add the folder back or remove the
  # ../ in the includes. This does the latter.
  postFixup = ''
    sed -i '/include/ s:\.\./::' $out/include/loadgen/c_api.h
  '';

  # Build shared, in a loadgen include folder without build time in product.
  patches = [ ./loadgen.patch ./log_settings.patch ];

  cmakeDir = "../loadgen";

  nativeBuildInputs = [
    cmake
    python36Packages.absl-py
  ];
}
