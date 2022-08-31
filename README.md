# litentry-parachain
[![Build & Test](https://github.com/litentry/litentry-parachain/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build-and-test.yml)
[![Build wasm](https://github.com/litentry/litentry-parachain/actions/workflows/build-wasm.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/build-wasm.yml)
[![Benchmark](https://github.com/litentry/litentry-parachain/actions/workflows/benchmark-runtime-weights.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/benchmark-runtime-weights.yml)
[![Release](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml/badge.svg)](https://github.com/litentry/litentry-parachain/actions/workflows/create-release-draft.yml)

The Litentry parachain.

Similar to polkadot, different chain-specs/runtimes are compiled into one single binary: in our case it's:
- litentry-parachain-runtime (on polkadot)
- litmus-parachain-runtime   (on kusama)

Therefore, when building node binary or docker image, no distinction is required. But when building runtime/starting binary/running tests, the chain type must be explicitly given. See the examples below.
## Lists of make targets
Simply run
```
make help
```
to see the full lists of market targets and their short descriptions.

## manual builds

To build the litentry-parachain raw binary manually:
```
make build-node
```

To build the `litentry/litentry-parachain` docker image locally:
```
make build-docker-release
or
make build-docker-production
```
they will use `release` or `production` cargo profile, respectively.

To build the litentry-parachain runtime wasm manually:
```
make build-runtime-litentry
```
The wasms should be located under `target/release/wbuild/litentry-parachain-runtime/`

Similarly, use `make build-runtime-litmus` to build the litmus-parachain-runtime.

## launch of local network

To start a local network with 2 relaychain nodes and 1 parachain node, there're two ways:

### 1. use docker images for both polkadot and litentry-parachain (preferred)
Take the litentry-parachain for example:
```
make launch-docker-litentry
```
[parachain-launch](https://github.com/open-web3-stack/parachain-launch) will be installed and used to generate chain-specs and docker-compose files.

The generated files will be under `docker/generated-litentry/`.

When finished with the network, run
```
make clean-docker-litentry
```
to stop the processes and tidy things up.

### 2. use raw binaries for both polkadot and litentry-parachain

Only when option 1 doesn't work and you suspect the docker-image went wrong.

In this case we could try to launch the network with raw binaries.

**On Linux host:**

- you should have the locally compiled `./target/release/litentry-collator` binary.
- run `make launch-binary-litentry`

**On Non-Linux host:**

- you should have locally compiled binaries, for both `polkadot` and `litentry-collator`
- run `./scripts/launch-local-binary.sh litentry path-to-polkadot-bin path-to-litentry-parachain-bin`

When finished with the network, run
```
make clean-binary
```
to stop the processes and tidy things up.
Note this command should work for both litentry and litmus (you don't have to differentiate them).

## run ts tests locally

To run the ts tests locally, similar to launching the networks, it's possible to run them in either docker or binary mode:
```
make test-ts-docker-litentry
```
or
```
# if on Linux
make test-ts-binary-litentry

# otherwise
./scripts/launch-local-binary.sh litentry path-to-polkadot-bin path-to-litentry-parachain-bin
./scripts/run-ts-test.sh
```
Remember to run the clean-up afterwards.

## extend the leasing period

The default leasing duration for parachain is 1 day, in case you want to extend it (even after it's downgraded to parathread), simply do a `forceLease` via sudo, it should be upgraded to parachain soon again and start to produce blocks.

![image](https://user-images.githubusercontent.com/7630809/135689832-1f57cd5c-7f83-4fce-9bb0-832b77a38dcc.png)


## Español
El parachain de Litentry.

De forma similar a polkadot, se compilan diferentes cadenas-específicas/tiempos de ejecución en un único binario: en nuestro caso es
- litentry-parachain-runtime (en polkadot)
- litmus-parachain-runtime (en kusama)

Por lo tanto, cuando se construye el binario del nodo o la imagen docker, no se requiere ninguna distinción. Pero cuando se construye el tiempo de ejecución/se inicia el binario/se ejecutan las pruebas, el tipo de cadena debe darse explícitamente. 

Previamente se debe instalar cargo desde rustuo en el siguiente link:
https://rustup.rs

Clonamos el repositorio
git clone https://github.com/cocreatorsdao/LitentryParachainCoCreator.git
 y nos situamos en la carpeta de litentry
cd lytentri-parachain
## construcciones manuales

Para construir el binario raw litentry-parachain manualmente:
```
make build-node
```

Para construir la imagen docker `litentry/litentry-parachain` localmente:
```
make build-docker-release
o
make build-docker-production
```
utilizarán el perfil de carga `release` o `production`, respectivamente.

Para construir el wasm de ejecución litentry-parachain manualmente:
```
make build-runtime-litentry
```
Los wasms deben estar ubicados en `target/release/wbuild/litentry-parachain-runtime/`,
si no tenemos las herramientas wasm debemos instalarla en la ubicaciòn dada con el siguiente comando:
```
rustup target add wasm32-unknown-unknown
```
Del mismo modo, utilice `make build-runtime-litmus` para construir el litmus-parachain-runtime.

## lanzamiento de la red local

Para iniciar una red local con 2 nodos relaychain y 1 nodo parachain, hay dos maneras:

### 1. usar imágenes docker tanto para polkadot como para litentry-parachain (preferido)
Para poder lanzar el nodo localmente debemos tener abierta la aplicación docker de escritorio.
Toma el litentry-parachain por ejemplo:
```
make launch-docker-litentry
```
[parachain-launch](https://github.com/open-web3-stack/parachain-launch) se instalará y se utilizará para generar los archivos chain-specs y docker-compose.

Los archivos generados estarán bajo `docker/generated-litentry/`.

Cuando hayas terminado con la cadena, ejecuta
```
make clean-docker-litentry
```
para detener los procesos y ordenar las cosas.

### 2. usar binarios crudos tanto para polkadot como para litentry-parachain

Sólo cuando la opción 1 no funcione y sospeches que la imagen docker ha salido mal.

En este caso podríamos intentar lanzar la red con binarios crudos.

**En el host de Linux:**

- deberías tener el binario `./target/release/litentry-collator` compilado localmente.
- ejecutar `make launch-binary-litentry`.

**En un servidor que no sea Linux:**

- debe tener los binarios compilados localmente, tanto para `polkadot` como para `litentry-collator`.
- ejecutar `./scripts/launch-local-binary.sh litentry path-to-polkadot-bin path-to-litentry-parachain-bin`

Cuando termine con la red, ejecute
```
make clean-binary
```
para detener los procesos y ordenar las cosas.
Ten en cuenta que este comando debería funcionar tanto para litentry como para litmus (no tienes que diferenciarlos).

## ejecutar las pruebas ts localmente

Para ejecutar las pruebas ts localmente, de forma similar al lanzamiento de las redes, es posible ejecutarlas en modo docker o binario:
```
make test-ts-docker-litentry
```
o
```
# si en Linux
make test-ts-binary-litentry

# en caso contrario
./scripts/launch-local-binary.sh litentry path-to-polkadot-bin path-to-litentry-parachain-bin
./scripts/run-ts-test.sh
```
Recuerda ejecutar la limpieza después.

## ampliar el periodo de leasing

La duración por defecto del leasing para parachain es de 1 día, en caso de que quieras extenderlo (incluso después de que se haya degradado a parathread), simplemente haz un `forceLease` vía sudo, debería actualizarse a parachain pronto de nuevo y empezar a producir bloques.

![image](https://user-images.githubusercontent.com/7630809/135689832-1f57cd5c-7f83-4fce-9bb0-832b77a38dcc.png)

## Licencia
GPLv3
