//
// Ejemplo de suscripción al nodo de Bitcoin usando ZeroMQ - rawblock
//  

use std::thread;
use std::time::Duration;
use zmq::Context;
use rustc_hex::ToHex;
extern crate hex;
use std::process::Command;
use std::str;

fn mempool_subscriber() -> Result<(), Box<dyn std::error::Error>> {
    
    // Crear un contexto zmq
    let context = Context::new();

    // Crear un socket de tipo SUB
    let subscriber = match context.socket(zmq::SUB) {
        Ok(socket) => socket,
        Err(e) => {
            eprintln!("Error al crear el socket: {:?}", e);
            return Err(e.into());
        }
    };

    // Conectar al nodo Bitcoin  (puerto 28332)
    if let Err(e) = subscriber.connect("tcp://127.0.0.1:28332") {
        eprintln!("Error al conectar al nodo Bitcoin: {:?}", e);
        return Err(e.into());
    }

    // Suscribirse a eventos relacionados con la mempool
    if let Err(e) = subscriber.set_subscribe(b"rawblock") {
        eprintln!("Error al suscribirse: {:?}", e);
        return Err(e.into());
    }

    println!("\n");
    println!("Esperando mensajes (rawblock) del nodo Bitcoin ...");
    println!("\n");

    // Recibir y procesar mensajes
    loop {
        
        // El primer mensaje contiene el canal de suscripción, por lo que lo descartamos
        let _ = match subscriber.recv_bytes(0) {
            Ok(channel) => channel,
            Err(e) => {
                eprintln!("Error al recibir el canal: {:?}", e);
                continue;
            }
        };
        
        println!("\n");

        match subscriber.recv_bytes(0) {
            Ok(tx_hex) => {
                // Convert bytes to a hexadecimal string
                let hex_string = tx_hex.to_hex::<String>();

                process_hex_string(&hex_string);

                Some(hex_string)
            },
            Err(e) => {
                println!("Failed to receive bytes: {}", e);
                None
            },
        };
        
        
        // Pausar para evitar un consumo excesivo de recursos
        thread::sleep(Duration::from_secs(5));
    
    }   // Fin del loop

}


// Procesa los distintos tipos de cadenas hexadecimales recibidas del nodo Bitcoin
fn process_hex_string(hex_string: &str) {
    // Longitud de la cadena hexadecimal
   let len = hex_string.len();

   match len {
       _ if len > 10 => {
            // Read hex string and convert to JSON
            println!("Block:");
            decode_raw_transaction(hex_string);
       },
       10 => {
           println!("Topic: {:?}", "rawtx");
       },
       8 => {
           let bytes = hex::decode(&hex_string).expect("Error al decodificar la cadena hexadecimal");
           let text_string = String::from_utf8_lossy(&bytes);
           println!("Sequence number: {} -> {:?}", &hex_string , text_string);
           println!("\n");
       },
       _ => {
           println!("Unrecognized hex string length: {}", len);
       }
   }
}


fn decode_raw_transaction(raw_transaction_hex: &str) {

    // Ejecutar el comando bitcoin-cli decoderawtransaction
    let output = Command::new("bitcoin-cli")
        .arg("decoderawblock")
        .arg(raw_transaction_hex)
        .output()
        .expect("Error al ejecutar el comando");

    // Verificar si la ejecución fue exitosa
    if output.status.success() {
        // Convertir la salida a una cadena UTF-8
        let decoded_output = str::from_utf8(&output.stdout).expect("Error al decodificar la salida");

        // Imprimir la salida decodificada
        println!("{}", decoded_output);
    } else {
        // Imprimir mensajes de error si la ejecución no fue exitosa
        let stderr = str::from_utf8(&output.stderr).expect("Error al decodificar la salida de error");
        eprintln!("Error: {}", stderr);
    }
} 


fn main() {
    // Iniciar el suscriptor en el hilo principal
    match mempool_subscriber() {
        Ok(_) => {
            // La función se ejecutó con éxito
            println!("Terminado"); 
        },
        Err(e) => {
            // Hubo un error al ejecutar la función, puedes manejarlo aquí
            eprintln!("Error al ejecutar mempool_subscriber: {:?}", e);
        },
    }
}
