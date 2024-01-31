let

  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-23.11";
  pkgs = import nixpkgs { config = {
    users.defaultUserShell = pkgs.zsh;
  }; overlays = []; };

in


pkgs.mkShell {
  packages = with pkgs; [
        git
        cargo
  ];
}
