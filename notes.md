La idea de proyecto:

Mi idea principal es poder capturar dos tipos de datos TCP y UDP,poder almacenarlo
en un archivo cap para su análisis.

Tambien agregare los comandos: 


Como libreria principal utilizare la de std::net la libreria estandar y 
tambien utilizare std::fs::File para crear el archivo cap.

Para capturar datos de una conexión TCP/IP, puede utilizar la clase TcpListener 
para crear un servidor y la clase TcpStream para conectar un cliente. 
Una vez que haya establecido una conexión,puede utilizar el método read() 
de TcpStream para leer los datos del cliente.

Captura de datos de la conexion.

Para capturar los datos, debes estar en el medio, entre el cliente y el servidor.
Esto significa que debes estar conectado a la misma red que el cliente y el servidor.
Una vez que estés conectado a la misma red que el cliente y el servidor,
puedes configurar tu computadora para que escuche las conexiones.

[Cliente] <-> [Intermediación] <-> [Servidor]

