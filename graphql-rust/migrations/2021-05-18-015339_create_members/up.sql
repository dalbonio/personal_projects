-- Your SQL goes here
CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE addresses (
    id SERIAL PRIMARY KEY,
    street VARCHAR NOT NULL,
    addr_number VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    client_id INT NOT NULL,
    FOREIGN KEY (client_id) REFERENCES clients(id)
);


INSERT INTO clients(id, name) VALUES (1, 'Client 1');
INSERT INTO addresses(id, street, addr_number, city, client_id) VALUES(1, 'Rua 1', '13', 'TestCity1', 1);
INSERT INTO clients(id, name) VALUES (2, 'Client 2');
INSERT INTO addresses(id, street, addr_number, city, client_id) VALUES(2, 'Rua 2', '23', 'TestCity2', 2);
INSERT INTO clients(id, name) VALUES (3, 'Client 3');
INSERT INTO addresses(id, street, addr_number, city, client_id) VALUES(3, 'Rua 3', '33', 'TestCity3', 3);