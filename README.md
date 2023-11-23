
# Ejemplo de utilización la biblioteca ZeroMQ con rawtx 

1. **Crear un contexto zmq**: El programa comienza creando un nuevo contexto ZeroMQ. Este contexto se utiliza para crear sockets ZeroMQ.

2. **Crear un socket de tipo SUB**: El programa crea un socket SUB utilizando el contexto ZeroMQ. Si hay un error al crear el socket, el programa imprime el error y lo devuelve.

3. **Conectar al nodo Bitcoin**: El socket SUB se conecta a un nodo de Bitcoin que se ejecuta en localhost en el puerto 28332. Si hay un error al conectar, el programa imprime el error y lo devuelve. 
En el archivo ***bitcoin.conf*** se ha de incluir el comando: 
zmqpubrawtx=tcp://127.0.0.1:28332

4. **Suscribirse a eventos relacionados con la mempool**: El socket SUB se suscribe a los eventos "hashblock", que representan las notificionrs sobre todas las transacciones.

5. **Recibir y procesar mensajes**: El programa entra en un bucle donde recibe y procesa continuamente mensajes. El primer mensaje que recibe es el canal de suscripción, que descarta. El segundo mensaje que recibe es el hash del bloque de 32 bytes, la cual se pasa a string. La tercera parte es un numero de secuencia. Si hay un error al recibir cualquiera de los mensajes, el programa imprime el error y continúa con la siguiente iteración del bucle o devuelve el error.

6. **Pausa**: El programa hace una pausa de 2 segundos para evitar el consumo excesivo de recursos.



*Resultado del script:*

Tx: 
```
{
  "txid": "f9b21fdaae08534f5d59dd1da6dd8238a039819da96c6d8a808bf67fd2353de1",
  "hash": "f14f56dcd542de92465ab1f11227ebb1e3400e28aad0769686eee818f78c86d2",
  "version": 1,
  "size": 225,
  "vsize": 144,
  "weight": 573,
  "locktime": 0,
  "vin": [
    {
      "txid": "bbee6621b2b3d8a5c811fdb76feef38e099adef62bcf16de0eceaf1d3ab14566",
      "vout": 1,
      "scriptSig": {
        "asm": "",
        "hex": ""
      },
      "txinwitness": [
        "3044022023a44fbbaf75efc94f 
        ...
```
Topic: "rawtx"

Sequence number: 94d40200 -> "��\u{2}\0"


