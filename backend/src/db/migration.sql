drop table if exists usuario;
drop table if exists persona;
drop table if exists nucleo;
drop table if exists bodega;
drop table if exists oficina;
drop table if exists municipio;
drop table if exists provincia;

create table provincia (
    id int primary key,
    nombre text not null unique
);

create table municipio (
    id serial primary key,
    nombre text not null,
    provincia_id int not null references provincia(id)
);

create table oficina (
    id serial primary key,
    nombre text not null,
    municipio_id int not null references municipio(id)
);

create table bodega (
    id serial primary key,
    nombre text not null,
    oficina_id int not null references oficina(id)
);

create table nucleo (
    id serial primary key,
    direccion text not null,
    bodega_id int not null references bodega(id)
);

create table persona (
    id uuid default gen_random_uuid() primary key,
    nombre text not null,
    apellido text not null,
    carnet text not null,
    nucleo_id int not null references nucleo(id),
    CONSTRAINT chk_carnet_length CHECK (LENGTH(carnet) = 11)
);

create table usuario (
    id uuid default gen_random_uuid() primary key,
    email text not null unique,
    pass_word text not null,
    persona_id uuid not null references persona(id)
);


insert into provincia (id, nombre) values
(30, 'PINAR DEL RIO'),
(31, 'ARTEMISA'),
(32, 'LA HABANA'),
(33, 'MAYABEQUE'),
(34, 'MATANZAS'),
(35, 'VILLA CLARA'),
(36, 'CIENFUEGOS'),
(37, 'SANCTI SPIRITUS'),
(38, 'CIEGO DE AVILA'),
(39, 'CAMAGUEY'),
(40, 'LAS TUNAS'),
(41, 'HOLGUIN'),
(42, 'GRANMA'),
(43, 'SANTIAGO DE CUBA'),
(44, 'GUANTANAMO'),
(45, 'ISLA DE LA JUVENTUD');


INSERT INTO municipio
(nombre, provincia_id)
VALUES('SANDINO', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MANTUA', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MINAS DE MATAHAMBRE', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('VINALES', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LA PALMA', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LOS PALACIOS', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CONSOLACION DEL SUR', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PINAR DEL RIO', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN LUIS', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN JUAN Y MARTINEZ', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUANE', 30);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BAHIA HONDA', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MARIEL', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUANAJAY', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CAIMITO', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BAUTA', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN ANTONIO DE LOS BANOS', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUIRA DE MELENA', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ALQUIZAR', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ARTEMISA', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CANDELARIA', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN CRISTOBAL', 31);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PLAYA', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PLAZA DE LA REVOLUCION', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CENTRO HABANA', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LA HABANA VIEJA', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('REGLA', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LA HABANA DEL ESTE', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUANABACOA', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN MIGUEL DEL PADRON', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('DIEZ DE OCTUBRE', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CERRO', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MARIANAO', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LA LISA', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BOYEROS', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ARROYO NARANJO', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('COTORRO', 32);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BEJUCAL', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN JOSE DE LAS LAJAS', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JARUCO', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SANTA CRUZ DEL NORTE', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MADRUGA', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('NUEVA PAZ', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN NICOLAS', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUINES', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MELENA DEL SUR', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BATABANO', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('QUIVICAN', 33);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MATANZAS', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CARDENAS', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MARTI', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('COLON', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PERICO', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JOVELLANOS', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PEDRO BETANCOURT', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LIMONAR', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('UNION DE REYES', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CIENAGA DE ZAPATA', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JAGUEY GRANDE', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CALIMETE', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LOS ARABOS', 34);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CORRALILLO', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('QUEMADO DE GUINES', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAGUA LA GRANDE', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ENCRUCIJADA', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CAMAJUANI', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CAIBARIEN', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('REMEDIOS', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PLACETAS', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SANTA CLARA', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CIFUENTES', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SANTO DOMINGO', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('RANCHUELO', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MANICARAGUA', 35);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('AGUADA DE PASAJEROS', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('RODAS', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PALMIRA', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LAJAS', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CRUCES', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CUMANAYAGUA', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CIENFUEGOS', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ABREUS', 36);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('YAGUAJAY', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JATIBONICO', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('TAGUASCO', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CABAIGUAN', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('FOMENTO', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('TRINIDAD', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SANCTI SPIRITUS', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LA SIERPE', 37);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CHAMBAS', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MORON', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BOLIVIA', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PRIMERO DE ENERO', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CIRO REDONDO', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('FLORENCIA', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MAJAGUA', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CIEGO DE AVILA', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('VENEZUELA', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BARAGUA', 38);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CARLOS MANUEL DE CESPEDES', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ESMERALDA', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SIERRA DE CUBITAS', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MINAS', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('NUEVITAS', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUAIMARO', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SIBANICU', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CAMAGUEY', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('FLORIDA', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('VERTIENTES', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JIMAGUAYU', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('NAJASA', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SANTA CRUZ DEL SUR', 39);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MANATI', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PUERTO PADRE', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JESUS MENENDEZ', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MAJIBACOA', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('LAS TUNAS', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JOBABO', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('COLOMBIA', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('AMANCIO', 40);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GIBARA', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('RAFAEL FREYRE', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BANES', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ANTILLA', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BAGUANOS', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('HOLGUIN', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CALIXTO GARCIA', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CACOCUM', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('URBANO NORIS', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CUETO', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MAYARI', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('FRANK PAIS', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAGUA DE TANAMO', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MOA', 41);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('RIO CAUTO', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CAUTO CRISTO', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('JIGUANI', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BAYAMO', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('YARA', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MANZANILLO', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CAMPECHUELA', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MEDIA LUNA', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('NIQUERO', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PILON', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BARTOLOME MASO', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BUEY ARRIBA', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUISA', 42);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CONTRAMAESTRE', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MELLA', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN LUIS', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SEGUNDO FRENTE', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SONGO - LA MAYA', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SANTIAGO DE CUBA', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('PALMA SORIANO', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('TERCER FRENTE', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUAMA', 43);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('EL SALVADOR', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MANUEL TAMES', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('YATERAS', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('BARACOA', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('MAISI', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('IMIAS', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('SAN ANTONIO DEL SUR', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('CAIMANERA', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('GUANTANAMO', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('NICETO PEREZ', 44);
INSERT INTO municipio
(nombre, provincia_id)
VALUES('ISLA DE LA JUVENTUD', 45);