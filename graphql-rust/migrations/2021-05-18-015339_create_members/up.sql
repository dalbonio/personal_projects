-- Your SQL goes here
CREATE TABLE addresses (
    id SERIAL PRIMARY KEY,
    street VARCHAR NOT NULL,
    addr_number VARCHAR NOT NULL,
    city VARCHAR NOT NULL
);

CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    address_id INT NOT NULL,
    FOREIGN KEY (address_id) REFERENCES addresses(id)
    ON DELETE CASCADE ON UPDATE CASCADE
);

INSERT INTO addresses(id, street, addr_number, city) VALUES(1, 'Rua 1', '13', 'TestCity1');
INSERT INTO clients(name, address_id) VALUES ('Client 1', 1);
INSERT INTO addresses(id, street, addr_number, city) VALUES(2, 'Rua 2', '23', 'TestCity2');
INSERT INTO clients(name, address_id) VALUES ('Client 2', 2);
INSERT INTO addresses(id, street, addr_number, city) VALUES(3, 'Rua 3', '33', 'TestCity3');
INSERT INTO clients(name, address_id) VALUES ('Client 3', 3);