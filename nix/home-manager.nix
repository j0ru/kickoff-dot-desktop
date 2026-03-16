self: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.programs.kickoff-dot-desktop;
in {
  options.programs.kickoff-dot-desktop = {
    enable = lib.mkEnableOption "kickoff-dot-desktop .desktop converter for kickoff";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.system}.default;
      defaultText = lib.literalExpression "pkgs.kickoff-dot-desktop";
      description = "The kickoff-dot-desktop package to install.";
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [cfg.package];
  };
}
