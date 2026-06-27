{ self }:

{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.programs.cosmic-sys-monitor;
  defaultPackage = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
in
{
  options.programs.cosmic-sys-monitor = {
    enable = lib.mkEnableOption "COSMIC System Monitor applet";

    package = lib.mkOption {
      type = lib.types.package;
      default = defaultPackage;
      defaultText = lib.literalExpression "cosmic-system-monitor.packages.\${pkgs.stdenv.hostPlatform.system}.default";
      description = "Package to install for the COSMIC System Monitor applet.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
  };
}
