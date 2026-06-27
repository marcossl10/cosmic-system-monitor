{
  lib,
  rustPlatform,
  makeWrapper,
  pkg-config,
  dbus,
  expat,
  fontconfig,
  freetype,
  gtk3,
  libGL,
  libxkbcommon,
  lm_sensors,
  openssl,
  vulkan-loader,
  wayland,
}:

let
  runtimeLibs = [
    dbus
    expat
    fontconfig
    freetype
    gtk3
    libGL
    libxkbcommon
    lm_sensors
    openssl
    vulkan-loader
    wayland
  ];
in
rustPlatform.buildRustPackage {
  pname = "cosmic-sys-monitor";
  version = "0.1.0";

  src = lib.cleanSource ../.;

  cargoHash = "sha256-uZ2JCuZhdFWKMxqGuCb6LxzGL/AygJkvFHu3T4cb0ws=";

  nativeBuildInputs = [
    makeWrapper
    pkg-config
  ];

  buildInputs = runtimeLibs;

  postInstall = ''
    install -Dm0644 resources/app.desktop \
      $out/share/applications/io.github.marcos.SysMonitor.desktop
    install -Dm0644 resources/app.metainfo.xml \
      $out/share/metainfo/io.github.marcos.SysMonitor.metainfo.xml
    install -Dm0644 resources/icon-symbolic.svg \
      $out/share/icons/hicolor/symbolic/apps/io.github.marcos.SysMonitor-symbolic.svg

    wrapProgram $out/bin/cosmic-sys-monitor \
      --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath runtimeLibs}
  '';

  meta = {
    description = "COSMIC system monitor panel applet";
    homepage = "https://github.com/marcossl10/cosmic-system-monitor";
    license = lib.licenses.mit;
    mainProgram = "cosmic-sys-monitor";
    platforms = lib.platforms.linux;
  };
}
