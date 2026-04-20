#!/usr/bin/env bash

# Este script aplica un escenario de pérdida de paquetes usando tc netem.
# Escenario solicitado: loss 8% sobre una interfaz de red seleccionada.

# set -u: evita errores silenciosos cuando se usa una variable no definida.
set -u

# Verificamos permisos de administrador, porque tc requiere privilegios de root.
if [[ "${EUID}" -ne 0 ]]; then
  # Indicamos la forma correcta de ejecución.
  echo "Este script necesita permisos de administrador. Ejecutá: sudo ./perdida_paquetes.sh <interfaz>"
  # Terminamos con código de error.
  exit 1
fi

# Validamos que se haya enviado la interfaz por parámetro.
if [[ $# -lt 1 ]]; then
  # Mostramos uso correcto si falta el argumento.
  echo "Uso: sudo ./perdida_paquetes.sh <interfaz>"
  echo "Ejemplo: sudo ./perdida_paquetes.sh wg0"
  # Finalizamos con error por argumentos insuficientes.
  exit 1
fi

# Guardamos el nombre de la interfaz para usarlo en los comandos.
INTERFAZ="$1"

# Comprobamos que la interfaz exista en el sistema.
if [[ ! -d "/sys/class/net/${INTERFAZ}" ]]; then
  # Avisamos claramente si la interfaz no es válida.
  echo "La interfaz '${INTERFAZ}' no existe en este host."
  # Salimos con error para evitar ejecutar tc sobre algo inexistente.
  exit 1
fi

# Informamos qué escenario se va a aplicar.
echo "[perdida_paquetes] Aplicando loss 8% en interfaz: ${INTERFAZ}"

# Aplicamos o reemplazamos la qdisc raíz con netem para simular 8% de pérdida de paquetes.
# 'replace' hace el script idempotente: crea la regla si no existe o la reemplaza si ya estaba.
tc qdisc replace dev "${INTERFAZ}" root netem loss 8%

# Mostramos la configuración activa para dejar evidencia del escenario aplicado.
tc qdisc show dev "${INTERFAZ}"

# Mensaje final de confirmación.
echo "[perdida_paquetes] Escenario aplicado correctamente en ${INTERFAZ}."
