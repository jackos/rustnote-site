# Godot

Compile from source

Required packages:
```bash
sudo pacman -S --needed scons pkgconf gcc libxcursor libxinerama libxi libxrandr mesa glu libglvnd \
    alsa-lib pulseaudio yasm
```

Run command:
```bash
scons -j$(nproc) platform=linuxbsd
```