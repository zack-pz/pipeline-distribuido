#!/usr/bin/env bash

# Este script elimina las reglas de tc (traffic control) para volver a una red "baseline".
# La idea es dejar cada interfaz sin degradaciones de netem ni colas personalizadas activas.

# set -u: hace que el script falle si se usa una variable no definida (evita errores silenciosos).
set -u

# Verificamos que el script se ejecute como root, porque tc necesita permisos de administrador.
if [[ "${EUID}" -ne 0 ]]; then
  # Mostramos cómo ejecutarlo correctamente con sudo.
  echo "Este script necesita permisos de administrador. Ejecutá: sudo ./baseline.sh"
  # Terminamos el script con código de error 1.
  exit 1
fi

# Recorremos todas las interfaces detectadas en /sys/class/net.
for iface_path in /sys/class/net/*; do
  # basename obtiene el nombre de la interfaz (por ejemplo: eth0, enp0s3, wg0).
  iface="$(basename "${iface_path}")"

  # Saltamos la interfaz loopback (lo), porque no es parte del tráfico de red real entre nodos.
  if [[ "${iface}" == "lo" ]]; then
    continue
  fi

  # Informamos qué interfaz se está limpiando.
  echo "[baseline] Limpiando interfaz: ${iface}"

  # Eliminamos la qdisc raíz de la interfaz.
  # Si había netem (delay/loss/rate) u otras colas hijas, esto las remueve.
  # Redirigimos errores a /dev/null y usamos || true para no cortar el script si no existía qdisc.
  tc qdisc del dev "${iface}" root 2>/dev/null || true

  # Eliminamos también la qdisc de ingress (entrada), por si había reglas aplicadas al tráfico entrante.
  # Igual que arriba: ignoramos error si no existe.
  tc qdisc del dev "${iface}" ingress 2>/dev/null || true

  # Mostramos el estado final de qdisc en esa interfaz para verificar que quedó limpia.
  tc qdisc show dev "${iface}"
done

# Mensaje final de confirmación.
echo "[baseline] Red restaurada a estado limpio (sin reglas tc personalizadas)."
