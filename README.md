
# Ejemplo de utilización la biblioteca ZeroMQ con rawblock 

1. **Crear un contexto zmq**: El programa comienza creando un nuevo contexto ZeroMQ. Este contexto se utiliza para crear sockets ZeroMQ.

2. **Crear un socket de tipo SUB**: El programa crea un socket SUB utilizando el contexto ZeroMQ. Si hay un error al crear el socket, el programa imprime el error y lo devuelve.

3. **Conectar al nodo Bitcoin**: El socket SUB se conecta a un nodo de Bitcoin que se ejecuta en localhost en el puerto 28332. Si hay un error al conectar, el programa imprime el error y lo devuelve. 
En el archivo ***bitcoin.conf*** se ha de incluir el comando: 
zmqpubrawblock=tcp://127.0.0.1:28332

4. **Suscribirse a eventos relacionados con la mempool**: El socket SUB se suscribe a los eventos "hashblock", que representan las notificionrs sobre todas las transacciones.

5. **Recibir y procesar mensajes**: El programa entra en un bucle donde recibe y procesa continuamente mensajes. El primer mensaje que recibe es el canal de suscripción, que descarta. El segundo mensaje que recibe es el hash del bloque de 32 bytes, la cual se pasa a string. La tercera parte es un numero de secuencia. Si hay un error al recibir cualquiera de los mensajes, el programa imprime el error y continúa con la siguiente iteración del bucle o devuelve el error.

6. **Pausa**: El programa hace una pausa de 2 segundos para evitar el consumo excesivo de recursos.



*Resultado del script:*

Block: 
```
{
  "hash": "00000000000000000000324a0860e13066d7beed9785d47523b46a64b6f380e8",
  "confirmations": 108,
  "height": 818016,
  "version": 603897856,
  "versionHex": "23fec000",
  "merkleroot": "a47d92add441e98c38dba4f01bb60a74d57d6ee138a1c712bcc4e6a71a3299df",
  "time": 1700695075,
  "mediantime": 1700689532,
  "nonce": 1874764091,
  "bits": "17045a12",
  "difficulty": 64678587803496.61,
  "chainwork": "00000000000000000000000000000000000000005e1537dbf0ef7349ae382061",
  "nTx": 2027,
  "previousblockhash": "00000000000000000002e6710101233b85e9977b8864239edce6ef0a3728993f",
  "nextblockhash": "00000000000000000000f87310c3e5a7903a19e6cada0777eedbb4b5a64465bd",
  "strippedsize": 797813,
  "size": 1599353,
  "weight": 3992792,
  "tx": [
	  "f945dcc657712f588b28bc06541a87fa0294437418c0b01cd41835e30f33c5b8",
	  "cc020c37bc9978711dca15e47f062a2b940abd8a3ce89f03ff53368ff3ceb087",
	  "e8f8158042fb0c1364726da1b0e93d7f207509e3df860408797f022618c22b94",
	  "febc294a7f848b314db774aec2dc96abf9f231373f5466f185758206b4ea92e0",
	  "6a0768985cd756517781ff1e781deab7bd52a61d1bf94af8b0fb18512a00bcb9",
	  "571d9a8e12117998cc2bf771b06ea6a09b662ad8957aa04ab86a8794bdee2842",
	  "e09bf6c16bb2e2567c8b
	…..

```



