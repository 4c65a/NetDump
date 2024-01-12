La idea de proyecto:

Mi idea principal es poder capturar dos tipos de datos TCP y UDP,poder almacenarlo
en un archivo cap para su análisis.


Captura de datos de conexiones TCP/IP

Para capturar datos de una conexión TCP/IP, puede utilizar la clase TcpListener 
para crear un servidor y la clase TcpStream para conectar un cliente. 
Una vez que haya establecido una conexión,puede utilizar el método read() 
de TcpStream para leer los datos del cliente.


