with import <nixpkgs> {}; {
  devShell = pkgs.mkShell {
    packages = with pkgs; [
      rust-analyzer
      cargo
      rustc
      rustfmt
      clippy
      cmake
      clang
      pkg-config
      libGL
      xorg.libXrandr
      xorg.libXinerama
      xorg.libXcursor
      xorg.libXi
    ];

    nativeBuildInputs = [pkgs.llvmPackages_16.libclang];

    LIBCLANG_PATH = "${pkgs.llvmPackages_16.libclang.lib}/lib";
    LD_LIBRARY_PATH = "${pkgs.libGL}/lib:${pkgs.xorg.libXi}/lib:${pkgs.xorg.libXcursor}/lib";
  };
}
