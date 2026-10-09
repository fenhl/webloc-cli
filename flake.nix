{
    inputs = {
        flake.url = "github:fenhl/flake";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "flake/nixpkgs";
        };
    };
    outputs = attrs: attrs.flake.lib {
        overlays = [
            attrs.rust-overlay.overlays.default # required for cargo-script
        ];
        devShells = {
            default = { pkgs, ... }: {
                packages = with pkgs; [
                    cargo
                ];
            };
            pre-commit = { pkgs, ... }: {
                packages = with pkgs; [
                    rust-bin.nightly.latest.default # nightly cargo, required to run the pre-commit script
                    cargo-deny
                ];
            };
        };
        packages.default = { pkgs, ... }: let
            manifest = (pkgs.lib.importTOML ./Cargo.toml).workspace.package;
        in pkgs.rustPlatform.buildRustPackage {
            inherit (manifest) version;
            pname = "webloc";
            buildAndTestSubdir = "crate/webloc-cli";
            cargoLock = {
                allowBuiltinFetchGit = true; # and allows omitting cargoLock.outputHashes
                lockFile = ./Cargo.lock;
            };
            nativeBuildInputs = with pkgs; [
                installShellFiles # required for `installShellCompletion` in postInstall hook
            ];
            postInstall = let
                webloc = "${pkgs.stdenv.hostPlatform.emulator pkgs.buildPackages} $out/bin/webloc";
            in pkgs.lib.optionalString (pkgs.stdenv.hostPlatform.emulatorAvailable pkgs.buildPackages) ''
                installShellCompletion --cmd webloc \
                    --bash <(COMPLETE=bash ${webloc}) \
                    --fish <(COMPLETE=fish ${webloc}) \
                    --zsh <(COMPLETE=zsh ${webloc})
            '';
            src = ./.;
        };
    };
}