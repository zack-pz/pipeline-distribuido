#!/usr/bin/env bash

# Este script aplica un escenario de latencia IoT usando tc netem.
# Escenario solicitado: delay 80ms con jitter de 20ms sobre una interfaz elegida.

# set -u: si se usa una variable no definida, el script termina para evitar errores silenciosos.
set -u

# Verificamos permisos de administrador porque tc necesita privilegios de root.
if [[ "${EUID}" -ne 0 ]]; then
  # Indicamos cómo ejecutar correctamente el script.
  echo "Este script necesita permisos de administrador. Ejecutá: sudo ./latencia_iot.sh <interfaz>"
  # Salimos con error.
  exit 1
fi

# Validamos que el usuario haya pasado la interfaz como argumento.
if [[ $# -lt 1 ]]; then
  # Mostramos uso esperado si falta el parámetro.
  echo "Uso: sudo ./latencia_iot.sh <interfaz>"
  echo "Ejemplo: sudo ./latencia_iot.sh wg0"
  # Salimos con error por argumentos insuficientes.
  exit 1
fi

# Guardamos el nombre de la interfaz en una variable legible.
INTERFAZ="$1"

# Confirmamos que la interfaz exista en el sistema antes de aplicar cambios.
if [[ ! -d "/sys/class/net/${INTERFAZ}" ]]; then
  # Avisamos que la interfaz no existe para evitar comandos inválidos.
  echo "La interfaz '${INTERFAZ}' no existe en este host."
  # Salimos con error.
  exit 1
fi

# Informamos qué escenario se va a aplicar y dónde.
echo "[latencia_iot] Aplicando delay 80ms jitter 20ms en interfaz: ${INTERFAZ}"

# Aplicamos (o reemplazamos) la qdisc raíz con netem para inyectar latencia y variación.
# 'replace' permite que el comando sea idempotente: crea si no existe o reemplaza si ya existe.
tc qdisc replace dev "${INTERFAZ}" root netem delay 80ms 20ms

# Mostramos la configuración activa para dejar evidencia del estado aplicado.
tc qdisc show dev "${INTERFAZ}"

# Confirmación final.
echo "[latencia_iot] Escenario aplicado correctamente en ${INTERFAZ}."
