{
  description = "Low cost HSI flake using uv2nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    inherit (nixpkgs) lib;
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
    python = pkgs.python313;
  in {
    devShells.${system}.default = let
      basic-libs =
        lib.makeLibraryPath
        pkgs.pythonManylinuxPackages.manylinux1;
      additional-libs = pkgs.lib.makeLibraryPath (with pkgs; [
        openblas
        stdenv.cc.cc.lib
        pkgs.cudaPackages.cudatoolkit
        linuxPackages.nvidia_x11
        pkgs.libgcc

        # matplotlib and pyside6
        zstd
        libGL
        libxkbcommon
        fontconfig
        libx11
        glib
        freetype
        dbus
        kdePackages.wayland
        kdePackages.qtwayland
        libxcb-util
        libxcb-cursor
        libxcb-wm
        libxcb-keysyms
        libxcb-render-util
        libxcb-image
        libxcb
        libdrm
      ]);
    in
      pkgs.mkShell {
        name = "devShell";
        packages = [
          python
          pkgs.uv
          # pkgs.ruff
        ];
        buildInputs = [
          # pkgs.qt6.qtbase
          # pkgs.qt6.qtwayland
          # py-pkgs.matplotlib
          # py-pkgs.pyside6
        ];
        env =
          {
            # Prevent uv from managing Python downloads
            UV_PYTHON_DOWNLOADS = "never";
            # Force uv to use nixpkgs Python interpreter
            UV_PYTHON = python.interpreter;
          }
          // lib.optionalAttrs pkgs.stdenv.isLinux {
            LD_LIBRARY_PATH = "${additional-libs}:${basic-libs}";
            # QT_STYLE_OVERRIDE = "Fusion";
            QT_STYLE_OVERRIDE = "";
            QT_PLUGIN_PATH = "${pkgs.kdePackages.qtwayland}/lib/qt-6/plugins";
            QT_QPA_PLATFORM_PLUGIN_PATH = "${pkgs.kdePackages.qtwayland}/lib/qt-6/plugins/platforms";

            # in case of issues, set to 1
            QT_DEBUG_PLUGINS = 0;
          };
        shellHook = ''
          unset PYTHONPATH
          if [ ! -d .venv  ]; then
            uv sync
          fi
          source .venv/bin/activate
        '';
      };
  };
}
