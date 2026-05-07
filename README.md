# Pipeline Distribuido IoT/Edge con Tolerancia a Fallos

Proyecto 2 de **IL355 — Programación de Sistemas Avanzados**. Este repositorio documenta y contiene la base de un pipeline distribuido con tres roles principales:

- **sensor**: genera lecturas IoT sintéticas.
- **edge**: recibe lecturas, calcula un reporte simple y envía heartbeats.
- **coordinator**: centraliza reportes y heartbeats.

El flujo principal del sistema es:

```text
sensor -> edge -> coordinator
           \------ heartbeat ----->
```

## 1. Descripción general de la arquitectura

La solución está organizada como un **workspace de Rust** y un despliegue con **Docker**. La conectividad distribuida entre nodos está pensada para ejecutarse sobre una **VPN WireGuard** y para ser validada bajo condiciones adversas de red usando **tc netem**.

### Componentes

- **Sensor (`rust/sensor`)**
  - Genera una lectura aleatoria de temperatura.
  - Serializa el mensaje a JSON.
  - Envía lecturas periódicamente al nodo edge por TCP.

- **Edge (`rust/edge`)**
  - Escucha lecturas entrantes en `EDGE_BIND_ADDR`.
  - Convierte cada `SensorReading` en un `EdgeReport`.
  - Reenvía el reporte al coordinator.
  - Envía un **heartbeat cada 3 segundos** en una tarea Tokio separada.

- **Coordinator (`rust/coordinator`)**
  - Escucha conexiones TCP en `COORDINATOR_BIND_ADDR`.
  - Recibe `EdgeReport` y `Heartbeat`.
  - Registra la actividad mediante `tracing`.

- **Contratos compartidos (`rust/shared`)**
  - Define las estructuras `SensorReading`, `EdgeReport`, `Heartbeat` y `CoordStatus`.

## 2. Estructura del repositorio

La estructura actual del repositorio es la siguiente:

```text
.
├── docker/
│   ├── coordinator/
│   │   └── Dockerfile
│   ├── edge/
│   │   └── Dockerfile
│   ├── sensor/
│   │   └── Dockerfile
│   ├── .env.example
│   └── docker-compose.yml
├── docs/
│   ├── riesgos_tecnicos_avance.md
│   └── topologia.png
├── netem/
│   ├── baseline.sh
│   ├── latencia_iot.sh
│   └── perdida_paquetes.sh
├── proyecto/
│   └── Proyecto2_DocD_Documentacion_Repositorio.pdf
├── rust/
│   ├── Cargo.toml
│   ├── coordinator/
│   ├── edge/
│   ├── sensor/
│   └── shared/
├── vpn/
│   ├── hub.conf
│   ├── peer1.conf
│   ├── peer2.conf
│   └── peer3.conf
├── .gitignore
└── README.md
```

### Relación con la estructura pedida en el Documento D

- **Presente**: `/vpn/`, `/docker/`, `/rust/`, `/netem/`, `/docs/`, `README.md`.
- **Opcional o según el caso**:
  - `/vpn/cgnat/`: no incluido actualmente.
  - `/kubernetes/`: no incluido actualmente.
- **Pendiente para cumplimiento total del bloque `netem/`**:
  - `enlace_limitado.sh`
  - `falla_nodo.sh`
  - `status.sh`

## 3. Requisitos de software

Para reproducir el sistema desde cero se recomienda contar con:

- **Rust 1.94 o superior**
  - Referencia: los Dockerfiles usan `rust:1.94-slim-bookworm`.
- **Cargo** (incluido con Rust)
- **Docker Engine**
- **Docker Compose v2**
- **WireGuard** (`wg`, `wg-quick`)
- **iproute2** (para `tc`)
- **iperf3** (recomendado para validar escenarios de red)
- **Linux con permisos de administrador** para ejecutar scripts de `tc netem`

## 4. Configuración de la VPN

La carpeta [`/vpn`](./vpn) contiene configuraciones sanitizadas para la topología WireGuard tipo **hub-and-spoke**.

### Archivos disponibles

- `vpn/hub.conf`
- `vpn/peer1.conf`
- `vpn/peer2.conf`
- `vpn/peer3.conf`

### Pasos sugeridos

1. Elegí el archivo correspondiente al nodo que vas a configurar.
2. Copialo a una configuración local real, por ejemplo `wg0.conf`.
3. Reemplazá los placeholders por:
   - clave privada real
   - clave pública real
   - endpoint/IP pública real
4. Levantá la interfaz:

   ```bash
   sudo wg-quick up wg0
   ```

5. Verificá el estado:

   ```bash
   sudo wg show
   ping 10.10.10.1
   ```

### Notas de seguridad

- **No subas llaves privadas ni secretos al repositorio.**
- Usá los archivos de `/vpn/` solo como **plantillas sanitizadas**.
- La configuración real `wg0.conf` debe permanecer fuera del control de versiones.

## 5. Construcción de imágenes Docker

Los tres roles tienen Dockerfiles independientes y usan compilación **multi-stage**.

Ejecutá estos comandos desde la **raíz del repositorio**:

```bash
docker build -f docker/coordinator/Dockerfile -t pipeline-coordinator .
docker build -f docker/edge/Dockerfile -t pipeline-edge .
docker build -f docker/sensor/Dockerfile -t pipeline-sensor .
```

## 6. Despliegue completo con Docker Compose

El archivo principal de orquestación es [`docker/docker-compose.yml`](./docker/docker-compose.yml).

### Variables de entorno utilizadas

- `RUST_LOG`
- `COORDINATOR_BIND_ADDR`
- `COORDINATOR_PORT`
- `EDGE_ID`
- `EDGE_BIND_ADDR`
- `COORDINATOR_ADDR`
- `EDGE_PORT`
- `SENSOR_ID`
- `EDGE_ADDR`
- `INTERVAL_MS`

### Ejecución rápida con valores por defecto

```bash
docker compose -f docker/docker-compose.yml up --build
```

Si preferís ejecutar Compose desde la carpeta `docker/`, el comando equivalente es:

```bash
docker compose up --build
```

### Ejecución explícita con variables de entorno

```bash
RUST_LOG=info \
COORDINATOR_BIND_ADDR=0.0.0.0:9001 \
COORDINATOR_PORT=9001 \
EDGE_ID=edge-1 \
EDGE_BIND_ADDR=0.0.0.0:8001 \
COORDINATOR_ADDR=coordinator:9001 \
EDGE_PORT=8001 \
SENSOR_ID=sensor-1 \
EDGE_ADDR=edge:8001 \
INTERVAL_MS=1000 \
docker compose -f docker/docker-compose.yml up --build
```

### Servicios levantados

- `coordinator`: expone el puerto `9001`
- `edge`: expone el puerto `8001`
- `sensor`: no publica puertos al host; envía lecturas al servicio edge

### Ejecución distribuida sobre VPN

Si se despliega entre nodos reales por WireGuard, ajustá al menos estas variables:

- `COORDINATOR_ADDR=10.10.10.1:9001`
- `EDGE_ADDR=10.10.10.X:8001`

## 7. Compilación y ejecución en Rust

### Compilar todo el workspace

Desde la raíz del repositorio:

```bash
cargo build --release --manifest-path rust/Cargo.toml --workspace
```

Si ya estás ubicado dentro de `rust/`, el comando equivalente es:

```bash
cargo build --release
```

### Ejecutar los binarios manualmente

Abrí **tres terminales** y ejecutá en este orden.

#### 1. Coordinator

```bash
COORDINATOR_BIND_ADDR=0.0.0.0:9001 \
cargo run --release --manifest-path rust/Cargo.toml -p coordinator
```

#### 2. Edge

```bash
EDGE_ID=edge-1 \
EDGE_BIND_ADDR=0.0.0.0:8001 \
COORDINATOR_ADDR=127.0.0.1:9001 \
cargo run --release --manifest-path rust/Cargo.toml -p edge
```

#### 3. Sensor

```bash
SENSOR_ID=sensor-1 \
EDGE_ADDR=127.0.0.1:8001 \
INTERVAL_MS=1000 \
cargo run --release --manifest-path rust/Cargo.toml -p sensor
```

Si estás dentro de `rust/`, podés ejecutar la misma secuencia con:

```bash
cargo run --release -p coordinator
cargo run --release -p edge
cargo run --release -p sensor
```

### Qué deberías observar

- El `sensor` envía una lectura periódica.
- El `edge` recibe la lectura y la transforma en un `EdgeReport`.
- El `coordinator` registra mensajes de tipo `DATA` y `HEARTBEAT`.

## 8. Escenarios `tc netem`

La carpeta [`/netem`](./netem) contiene scripts para degradar o restaurar la red.

> Recomendación: aplicar las reglas sobre la interfaz que el equipo haya definido para las pruebas, por ejemplo `wg0`, y validar el efecto con `tc qdisc show` e idealmente con `iperf3`.

### 8.1 Baseline

Restaura el estado limpio eliminando qdisc personalizadas.

```bash
sudo ./netem/baseline.sh
```

### 8.2 Latencia IoT

Aplica `delay 80ms 20ms` sobre la interfaz indicada.

```bash
sudo ./netem/latencia_iot.sh wg0
```

### 8.3 Pérdida de paquetes

Aplica `loss 8%` sobre la interfaz indicada.

```bash
sudo ./netem/perdida_paquetes.sh wg0
```

### Escenarios requeridos por el documento y pendientes en el repositorio

Según el Documento D, también deben existir estos scripts:

- `netem/enlace_limitado.sh` → `rate 512kbit delay 50ms`
- `netem/falla_nodo.sh` → detener un contenedor edge específico
- `netem/status.sh` → mostrar estado de `tc qdisc`

## 9. Evidencias, archivos prohibidos y buenas prácticas

Para mantener el repositorio evaluable y seguro:

- No subir `target/` ni binarios compilados de Rust.
- No subir `.env` con credenciales.
- No subir `wg0.conf` real.
- No subir llaves privadas ni certificados.
- Las capturas y gráficas deben ir en los **PDFs de entrega**, no como imágenes sueltas del repositorio.

El `.gitignore` actual ya contempla estos elementos base:

- `target/`
- `*.key, *.pem, *.p12`
- `wg0.conf`
- `.env`

## 10. Documentación complementaria

La carpeta [`/docs`](./docs) contiene material de apoyo del proyecto, por ejemplo:

- `docs/riesgos_tecnicos_avance.md`
- `docs/topologia.png`

## 11. Supuestos y limitaciones conocidas

- El pipeline actual representa una **base funcional de avance**, no una implementación final completa.
- El `edge` procesa una lectura por conexión y genera un reporte simple con `sample_count = 1`.
- El `coordinator` actualmente registra eventos en logs, pero no expone todavía un endpoint HTTP de estado.
- La carpeta `netem/` todavía **no contiene todos los scripts** requeridos por el Documento D.
- La carpeta `/kubernetes/` no está incluida en el estado actual del repositorio.
- Las configuraciones de `/vpn/` son plantillas; cada equipo debe completar localmente sus valores reales.

## 12. Recomendaciones de uso

1. Verificá primero la conectividad VPN.
2. Probá el flujo local con Rust o Docker antes de distribuirlo entre nodos.
3. Aplicá `baseline.sh` antes de cambiar de escenario `tc netem`.
4. Validá cada degradación con `tc qdisc show` y, si es posible, con `iperf3`.
5. Documentá cada prueba importante en los PDFs de entrega.
