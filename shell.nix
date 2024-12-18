{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    rustup
  ];

  shellHook = ''rustup default nightly'';
}