{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShellNoCC {
	nativeBuildInputs = with pkgs.buildPackages; [
		openssl
		pkg-config
	];
}
