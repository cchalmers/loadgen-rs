{ stdenv, fetchFromGitHub, cmake }:

stdenv.mkDerivation rec {
  pname = "concurrentqueue";
  version = "1.0.0-beta";

  src = fetchFromGitHub {
    owner = "cameron314";
    repo = "concurrentqueue";
    rev = "v${version}";
    sha256 = "1rdjf2h0j1jgfgifh0aic8f5rrgg9bj5vgwr338klf0w7lqvj1wb";
  };

  buildPhase = ":";

  installPhase = ''
    mkdir -p $out/include
    mv blockingconcurrentqueue.h concurrentqueue.h $out/include
  '';
}
